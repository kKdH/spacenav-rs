use crate::app::views;
use crate::app::views::{header_view, profiles_view};
use crate::assets::ImageHandles;
use crate::util::{load_profiles, load_profile_icons, store_profiles};
use iced::widget::container;
use iced::widget::image::Handle;
use iced::{widget, Task};
use iced::{Element, Subscription};
use iced_toaster::{Toast, ToastId, ToastLevel, Toaster};
use spacenav_client::SpaceNavClient;
use spacenav_settings::{Keybinding, KeybindingButton, KeybindingButtonState, MotionFunctionName, Profile, ProfileId, Profiles};
use std::collections::BTreeMap;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tracing::error;
use views::footer_view;

pub struct SpaceNavCockpit {
    pub state: State,
    pub profiles: Profiles,
    pub selected_profile: Option<ProfileId>,
    pub active_profile_configuration_view: ProfileConfigurationView,
    pub client: Option<SpaceNavClient>,
    pub device: Option<libspnav::Device>,
    pub axes_values: [f32; 6],
    pub toaster: Toaster<Message>,
    pub image_handles: ImageHandles,
    pub profiles_icon_handles: BTreeMap<ProfileId, Handle>,
    pub last_button_event: Option<libspnav::ButtonEvent>,
}

#[derive(Debug, Clone, Copy)]
pub enum ProfileConfigurationView {
    Motions,
    Keybindings,
}

#[derive(Debug, Clone)]
pub enum State {
    Disconnected,
    Connecting,
    Connected,
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadSettings,
    StoreSettings,
    Connect,
    Disconnect,
    ClientOpenEvent(Result<(), ()>),
    ClientCloseEvent(Result<(), ()>),
    ClientSubscriptionEvent(Result<(), ()>),
    ClientEvent(libspnav::Event),
    ClientGetDeviceEvent(Result<libspnav::Device, ()>),
    ClientSetAxesSpeedEvent(Result<(), ()>),
    ProfileSelected(ProfileId),
    ProfileMotionsConfigurationViewActivated(ProfileId),
    ProfileKeybindingsConfigurationViewActivated(ProfileId),
    PushToast(Toast<Message>),
    DismissToast(ToastId),
    SetHoveredToast(ToastId, bool),
    AxisSpeedChanged { profile_id: ProfileId, function_name: MotionFunctionName, speed: f32 },
    UpdateAxisSpeed { profile_id: ProfileId, function_name: MotionFunctionName },
    AxisThresholdChanged { profile_id: ProfileId, function_name: MotionFunctionName, threshold: u8 },
    AxisInvertedChanged { profile_id: ProfileId, function_name: MotionFunctionName, inverted: bool },
    AxisDisabledChanged { profile_id: ProfileId, function_name: MotionFunctionName, disabled: bool },
    UpdateAxisThreshold { profile_id: ProfileId, function_name: MotionFunctionName },
    AxisMappingChanged { profile_id: ProfileId, function_name: MotionFunctionName, axis: u8 },
    KeybindingSelectProfileChanged { profile_id: ProfileId, keybinding: usize, select: Option<ProfileId> },
    KeybindingButtonChanged { profile_id: ProfileId, keybinding: usize, button: Option<u8> },
    FireKeyEvent { profile_id: ProfileId, keybinding: usize },
    Tick,
}

impl SpaceNavCockpit {

    pub fn create() -> Self {
        Self {
            client: Some(SpaceNavClient::create()),
            state: State::Disconnected,
            profiles: Profiles::default(),
            selected_profile: None,
            active_profile_configuration_view: ProfileConfigurationView::Motions,
            device: None,
            axes_values: [0_f32; 6],
            toaster: iced_toaster::toaster(),
            image_handles: ImageHandles::new(),
            profiles_icon_handles: BTreeMap::new(),
            last_button_event: None,
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            iced::time::every(std::time::Duration::from_millis(200))
                .map(|_| Message::Tick)
        ])
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LoadSettings => {
                match load_profiles() {
                    Ok(mut profiles) => {
                        if profiles.is_empty() {
                            profiles.profiles.insert(
                                ProfileId::try_from("default").expect("Should be a valid ProfileId"),
                                Profile::new(String::from("Default"))
                            );
                        }
                        self.profiles = profiles;
                        self.profiles_icon_handles = load_profile_icons(self.profiles.profiles.iter()).expect("Failed to load profile icons"); // TODO: Handle error during profile icon loading.
                        self.toaster.push(iced_toaster::toast("Settings loaded successfully.")
                            .title("Success")
                            .duration(3)
                            .level(ToastLevel::Success));

                    }
                    Err(_) => {
                        self.toaster.push(iced_toaster::toast("Failed to load settings.")
                            .title("Error")
                            .duration(3)
                            .level(ToastLevel::Error));
                    }
                }
                let first = self.profiles.profiles.keys().next().expect("There should be at least one profile.");
                Task::done(Message::ProfileSelected(first.to_owned()))
            }
            Message::StoreSettings => {
                match store_profiles(&self.profiles) {
                    Ok(_) => {
                        self.toaster.push(iced_toaster::toast("Settings stored successfully.")
                            .title("Success")
                            .duration(3)
                            .level(ToastLevel::Success));
                    }
                    Err(_) => {
                        self.toaster.push(iced_toaster::toast("Failed to store settings.")
                            .title("Error")
                            .duration(3)
                            .level(ToastLevel::Error));
                    }
                }
                Task::none()
            }
            Message::Connect => {
                if matches!(self.state, State::Disconnected) {
                    self.state = State::Connecting;
                    let open = self.client.as_ref()
                        .expect("Client should be created before")
                        .open("SpaceNav Cockpit");
                    Task::perform(open, Message::ClientOpenEvent)
                }
                else {
                    Task::none()
                }
            }
            Message::Disconnect => {
                if matches!(self.state, State::Connected) {
                    let close = self.client.as_ref()
                        .expect("Client should be created before.")
                        .close();
                    Task::perform(close, Message::ClientCloseEvent)
                }
                else {
                    Task::none()
                }
            }
            Message::ClientOpenEvent(result) => {
                match result {
                    Ok(_) => {
                        self.state = State::Connected;
                        self.toaster.push(iced_toaster::toast("Connected to the SpaceNav daemon.")
                            .title("Success")
                            .duration(3)
                            .level(ToastLevel::Success));
                        let get_device = self.client.as_ref()
                            .expect("Client should be created before.")
                            .get_device();
                        Task::perform(get_device, Message::ClientGetDeviceEvent)
                    }
                    Err(_) => {
                        self.state = State::Disconnected;
                        self.toaster.push(iced_toaster::toast("Failed to connect to the SpaceNav daemon!")
                            .title("Failure")
                            .duration(3)
                            .level(ToastLevel::Error));
                        Task::none()
                    }
                }
            }
            Message::ClientCloseEvent(result) => {
                match result {
                    Ok(_) => {
                        self.state = State::Disconnected;
                        self.device = None;
                        self.toaster.push(iced_toaster::toast("Disconnected from the SpaceNav daemon.")
                            .title("Success")
                            .duration(3)
                            .level(ToastLevel::Success));
                    }
                    Err(_) => {
                        self.toaster.push(iced_toaster::toast("Failed to disconnect from the SpaceNav daemon!")
                            .title("Failure")
                            .duration(3)
                            .level(ToastLevel::Error));
                    }
                }
                Task::none()
            }
            Message::ClientSubscriptionEvent(_) => {
                Task::none()
            }
            Message::ClientGetDeviceEvent(result) => {
                match result {
                    Ok(device) => {
                        self.toaster.push(iced_toaster::toast(device.ty.to_string())
                            .title("Device")
                            .duration(3)
                            .level(ToastLevel::Success));
                        self.device = Some(device);
                        let (event_sender, event_receiver) = mpsc::channel(1024);
                        let subscribe = self.client.as_ref()
                            .expect("Client should be created before.")
                            .subscribe(event_sender);
                        Task::batch(vec![
                            Task::run(ReceiverStream::new(event_receiver), Message::ClientEvent),
                            Task::perform(subscribe, Message::ClientSubscriptionEvent)
                        ])
                    }
                    Err(_) => {
                        self.toaster.push(iced_toaster::toast("No device found!")
                            .title("Device")
                            .duration(3)
                            .level(ToastLevel::Warning));
                        Task::none()
                    }
                }
            }
            Message::ClientSetAxesSpeedEvent(_) => {
                Task::none()
            }
            Message::ClientEvent(event) => {
                self.handle_client_event(event)
            }
            Message::ProfileSelected(profile_id) => {
                if self.selected_profile.as_ref().is_none_or(|p| p != &profile_id) {
                    self.selected_profile = Some(profile_id);
                    Task::batch(vec![
                        self.update_axes_speed(),
                        self.update_axes_threshold(),
                        self.update_axes_inverted(),
                        self.update_axes_mapping(),
                    ])
                }
                else {
                    Task::none()
                }
            }
            Message::ProfileMotionsConfigurationViewActivated(_profile_id) => {
                self.active_profile_configuration_view = ProfileConfigurationView::Motions;
                Task::none()
            }
            Message::ProfileKeybindingsConfigurationViewActivated(_profile_id) => {
                self.active_profile_configuration_view = ProfileConfigurationView::Keybindings;
                Task::none()
            }
            Message::PushToast(toast) => {
                self.toaster.push(toast);
                Task::none()
            }
            Message::DismissToast(id) => {
                self.toaster.dismiss(id);
                Task::none()
            }
            Message::SetHoveredToast(id, hovered) => {
                self.toaster.set_hovered(id, hovered);
                Task::none()
            }
            Message::AxisSpeedChanged { profile_id, function_name, speed } => {
                if let Some(profile) = self.profiles.profiles.get_mut(&profile_id) {
                    if let Some(function_settings) = profile.motions.get_mut(&function_name) {
                        let speed = speed.max(0_f32).min(2_f32);
                        let speed = (speed * 100_f32).round() / 100_f32;
                        function_settings.speed = speed;
                    }
                }
                Task::none()
            }
            Message::UpdateAxisSpeed { profile_id: _profile_id, function_name: _function_name } => {
                self.update_axes_speed()
            }
            Message::AxisThresholdChanged { profile_id, function_name, threshold } => {
                if let Some(profile) = self.profiles.profiles.get_mut(&profile_id) {
                    if let Some(function_settings) = profile.motions.get_mut(&function_name) {
                        function_settings.threshold = threshold;
                    }
                }
                Task::none()
            }
            Message::UpdateAxisThreshold { profile_id: _profile_id, function_name: _axis } => {
                self.update_axes_threshold()
            }
            Message::AxisInvertedChanged { profile_id, function_name, inverted: invert } => {
                if let Some(profile) = self.profiles.profiles.get_mut(&profile_id) {
                    if let Some(function_settings) = profile.motions.get_mut(&function_name) {
                        function_settings.inverted = invert;
                    }
                }
                self.update_axes_inverted()
            }
            Message::AxisDisabledChanged { profile_id, function_name, disabled } => {
                if let Some(profile) = self.profiles.profiles.get_mut(&profile_id) {
                    if let Some(function_settings) = profile.motions.get_mut(&function_name) {
                        function_settings.disabled = disabled;
                    }
                }
                self.update_axes_speed()
            }
            Message::AxisMappingChanged { profile_id, function_name, axis } => {
                if let Some(profile) = self.profiles.profiles.get_mut(&profile_id) {
                    if let Some(function_settings) = profile.motions.get_mut(&function_name) {
                        function_settings.axis = axis;
                    }
                }
                self.update_axes_mapping()
            }
            Message::KeybindingSelectProfileChanged { profile_id, keybinding, select } => {
                if let Some(profile) = self.profiles.profiles.get_mut(&profile_id) {
                    match profile.keybindings.get_mut(keybinding) {
                        Some(Keybinding::SelectProfile { profile, .. }) => *profile = select,
                        _ => {}
                    }
                }
                Task::none()
            }
            Message::KeybindingButtonChanged { profile_id, keybinding, button } => {
                if let Some(profile) = self.profiles.profiles.get_mut(&profile_id) {
                    if let Some(keybinding) = profile.keybindings.get_mut(keybinding) {
                        let new_button = button.map(|number| KeybindingButton::new(number, KeybindingButtonState::Released));
                        match keybinding {
                            Keybinding::SelectProfile { button, .. } => *button = new_button,
                            Keybinding::PreviousProfile { button } => *button = new_button,
                            Keybinding::NextProfile { button } => *button = new_button,
                        }
                    }
                }
                Task::none()
            }
            Message::FireKeyEvent { profile_id, keybinding } => {
                if let Some(profile) = self.profiles.profiles.get(&profile_id) {
                    match profile.keybindings.get(keybinding) {
                        None => {
                            error!("There is no keybinding '{keybinding}' for profile '{profile_id}'!");
                            Task::none()
                        },
                        Some(Keybinding::SelectProfile { profile: Some(profile_id), .. }) => {
                            Task::done(Message::ProfileSelected(Clone::clone(&profile_id)))
                        }
                        _ => Task::none()
                    }
                }
                else {
                    Task::none()
                }
            }
            Message::Tick => {
                self.toaster.dismiss_expired();
                Task::none()
            }
        }
    }

    fn update_axes_speed(&mut self) -> Task<Message> {

        if !matches!(self.state, State::Connected) {
            return Task::none();
        }

        let profile = &self.selected_profile.as_ref()
            .and_then(|id| self.profiles.profiles.get(id))
            .expect("Selected profile must exist");

        // TODO: This will fail if there are more or less than 6 axes.
        // TODO: Verify the ordering of the axes (tx, ty, tz, rx, ry, rz).
        let speed: [f32; 6] = profile.motions.values()
            .map(|function_settings| if function_settings.disabled { 0_f32 } else { function_settings.speed })
            .collect::<Vec<_>>()
            .try_into()
            .expect("There should be exactly six axes.");

        let set_axes_speed = self.client.as_ref()
            .expect("Client should be created before")
            .set_axes_speed(speed);

        Task::perform(set_axes_speed, Message::ClientSetAxesSpeedEvent)
    }

    fn update_axes_threshold(&mut self) -> Task<Message> {

        if !matches!(self.state, State::Connected) {
            return Task::none();
        }

        let profile = &self.selected_profile.as_ref()
            .and_then(|id| self.profiles.profiles.get(id))
            .expect("Selected profile must exist");

        // TODO: This will fail if there are more or less than 6 axes.
        // TODO: Verify the ordering of the axes (tx, ty, tz, rx, ry, rz).
        let threshold: [u8; 6] = profile.motions.values()
            .map(|function_settings| function_settings.threshold)
            .collect::<Vec<_>>()
            .try_into()
            .expect("There should be exactly six axes.");

        let set_axes_threshold = self.client.as_ref()
            .expect("Client should be created before")
            .set_axes_threshold(threshold);

        Task::perform(set_axes_threshold, Message::ClientSetAxesSpeedEvent)
    }

    fn update_axes_inverted(&mut self) -> Task<Message> {

        if !matches!(self.state, State::Connected) {
            return Task::none();
        }

        let profile = &self.selected_profile.as_ref()
            .and_then(|id| self.profiles.profiles.get(id))
            .expect("Selected profile must exist");

        // TODO: This will fail if there are more or less than 6 axes.
        // TODO: Verify the ordering of the axes (tx, ty, tz, rx, ry, rz).
        let inverted: [bool; 6] = profile.motions.values()
            .map(|function_settings| function_settings.inverted)
            .collect::<Vec<_>>()
            .try_into()
            .expect("There should be exactly six axes.");

        let set_axes_inverted = self.client.as_ref()
            .expect("Client should be created before")
            .set_axes_inverted(inverted);

        Task::perform(set_axes_inverted, Message::ClientSetAxesSpeedEvent)
    }

    fn update_axes_mapping(&mut self) -> Task<Message> {

        if !matches!(self.state, State::Connected) {
            return Task::none();
        }

        let profile = &self.selected_profile.as_ref()
            .and_then(|id| self.profiles.profiles.get(id))
            .expect("Selected profile must exist");

        // TODO: This will fail if there are more or less than 6 axes.
        // TODO: Verify the ordering of the axes (tx, ty, tz, rx, ry, rz).
        let mapping: [u8; 6] = profile.motions.values()
            .map(|function_settings| function_settings.axis)
            .collect::<Vec<_>>()
            .try_into()
            .expect("There should be exactly six axes.");

        let set_axes_mapping = self.client.as_ref()
            .expect("Client should be created before")
            .set_axes_mapping(mapping);

        Task::perform(set_axes_mapping, Message::ClientSetAxesSpeedEvent)
    }

    fn handle_client_event(&mut self, event: libspnav::Event) -> Task<Message> {

        match event {
            libspnav::Event::Motion(event) =>
                self.handle_client_motion_event(event),
            libspnav::Event::Button(event) =>
                self.handle_client_button_event(event),
            _ => Task::none(),
        }
    }

    fn handle_client_motion_event(&mut self, event: libspnav::MotionEvent) -> Task<Message> {
        // TODO: Verify the ordering of the axes (tx, ty, tz, rx, ry, rz).
        if let Some(profile) = self.selected_profile.as_ref().and_then(|profile_id| self.profiles.profiles.get(profile_id)) {
            let values = [event.x as f32, event.y as f32, event.z as f32, event.rx as f32, event.ry as f32, event.rz as f32];
            MotionFunctionName::MOTION_FUNCTION_NAMES.iter()
                .flat_map(|function_name| profile.motions.get(function_name))
                .map(|function_settings| function_settings.axis as usize)
                .zip(values.into_iter())
                .for_each(|(axis, value)| {
                    self.axes_values[axis] = value;
                });
        }

        Task::none()
    }

    fn handle_client_button_event(&mut self, event: libspnav::ButtonEvent) -> Task<Message> {

        // Filter consecutive events for the same button with the same state.
        if let Some(previous_button_event) = self.last_button_event.as_ref() {
            if previous_button_event == &event {
                return Task::none();
            }
        }

        let libspnav::ButtonEvent { button: event_button_number, state: event_button_state } = event;
        self.last_button_event = Some(event);

        if let Some(selected_profile_id) = self.selected_profile.as_ref() {
            if let Some(selected_profile) = self.profiles.profiles.get(selected_profile_id) {
                let tasks = selected_profile.keybindings.iter()
                    .filter(|keybinding| keybinding.button().is_some_and(|button| {
                        button.number as i32 == event_button_number &&
                        (
                            matches!(button.state, KeybindingButtonState::Released) && matches!(event_button_state, libspnav::ButtonState::Released) ||
                            matches!(button.state, KeybindingButtonState::Pressed) && matches!(event_button_state, libspnav::ButtonState::Pressed)
                        )
                    }))
                    .enumerate()
                    .map(|(keybinding, _)| Task::done(Message::FireKeyEvent { profile_id: Clone::clone(&selected_profile_id), keybinding }))
                    .collect::<Vec<_>>();
                return Task::batch(tasks);
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {

        let content = container(
            widget::Column::new()
                .push(header_view(self))
                .push(profiles_view(self))
                .push(footer_view(self))
            );

            self.toaster
                .view(content, Message::DismissToast, Message::SetHoveredToast)
    }
}
