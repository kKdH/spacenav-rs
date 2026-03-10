use crate::app::views;
use crate::app::views::{configuration_view, header_view};
use crate::app::widgets::axis_bar;
use crate::assets::ImageHandles;
use crate::util::{load_profiles, store_profiles};
use iced::widget::container;
use iced::{widget, Fill, Task};
use iced::{Element, Subscription};
use iced_toaster::{Toast, ToastId, ToastLevel, Toaster};
use spacenav_client::SpaceNavClient;
use spacenav_settings::{NavigationFunctionName, Profile, Profiles};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use views::footer_view;

pub struct SpaceNavCockpit {
    pub state: State,
    pub profiles: Profiles,
    pub selected_profile: Option<String>,
    pub client: Option<SpaceNavClient>,
    pub device: Option<libspnav::Device>,
    pub tx: f32,
    pub ty: f32,
    pub tz: f32,
    pub rx: f32,
    pub ry: f32,
    pub rz: f32,
    pub toaster: Toaster<Message>,
    pub image_handles: ImageHandles,
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
    ClientSetIndividualAxesSpeedEvent(Result<(), ()>),
    TabSelected(String),
    PushToast(Toast<Message>),
    DismissToast(ToastId),
    SetHoveredToast(ToastId, bool),
    AxisSpeedChanged { profile: String, axis: NavigationFunctionName, speed: f32 },
    Tick,
}

impl SpaceNavCockpit {

    pub fn create() -> Self {
        Self {
            client: Some(SpaceNavClient::create()),
            state: State::Disconnected,
            profiles: Profiles::default(),
            selected_profile: None,
            device: None,
            tx: 0_f32,
            ty: 0_f32,
            tz: 0_f32,
            rx: 0_f32,
            ry: 0_f32,
            rz: 0_f32,
            toaster: iced_toaster::toaster(),
            image_handles: ImageHandles::new(),
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
                            profiles.profiles.insert(String::from("default"), Profile::new(String::from("Default")));
                        }
                        self.profiles = profiles;
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
                Task::none()
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
            Message::ClientSetIndividualAxesSpeedEvent(_) => {
                Task::none()
            }
            Message::ClientEvent(event) => {
                match event {
                    libspnav::Event::Axis(event) => {
                        match event.index {
                            0 => self.tx = event.value as f32,
                            1 => self.ty = event.value as f32,
                            2 => self.tz = event.value as f32,
                            3 => self.rx = event.value as f32,
                            4 => self.ry = event.value as f32,
                            5 => self.rz = event.value as f32,
                            _ => {}
                        }
                    }
                    _ => {}
                }
                Task::none()
            }
            Message::TabSelected(profile_id) => {
                self.selected_profile = Some(profile_id);
                self.update_individual_axes_speed()
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
            Message::AxisSpeedChanged { profile: profile_id, axis, speed} => {
                if let Some(profile) = self.profiles.profiles.get_mut(&profile_id) {
                    if let Some(axis) = profile.navigation.get_mut(&axis) {
                        let speed = speed.max(0_f32).min(2_f32);
                        let speed = (speed * 100_f32).round() / 100_f32;
                        axis.speed = speed;
                    }
                    self.update_individual_axes_speed()
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

    fn update_individual_axes_speed(&mut self) -> Task<Message> {

        let profile = &self.selected_profile.as_ref()
            .and_then(|id| self.profiles.profiles.get(id))
            .expect("Selected profile must exist");

        // TODO: This will fail if there are more or less than 6 axes.
        // TODO: Verify the ordering of the axes (tx, ty, tz, rx, ry, rz).
        let speed: [f32; 6] = profile.navigation.values()
            .map(|axis| axis.speed)
            .collect::<Vec<_>>()
            .try_into()
            .expect("There should be exactly six axes.");

        let set_individual_axes_speed = self.client.as_ref()
            .expect("Client should be created before")
            .set_individual_axes_speed(speed);

        Task::perform(set_individual_axes_speed, Message::ClientSetIndividualAxesSpeedEvent)
    }

    pub fn view(&self) -> Element<'_, Message> {

        let content = container(
            widget::Column::new()
                .push(header_view(self))
                .push(configuration_view(self))
                .push(widget::Row::new()
                    .push(widget::Column::new()
                        .push(widget::Row::new()
                            .spacing(10)
                            .push(widget::text("tx"))
                            .push(widget::canvas(axis_bar(-500_f32..=500_f32, self.tx)).width(Fill).height(30)).padding(10))
                        .push(widget::Row::new()
                            .spacing(10)
                            .push(widget::text("ty"))
                            .push(widget::canvas(axis_bar(-500_f32..=500_f32, self.ty)).width(Fill).height(30)).padding(10))
                        .push(widget::Row::new()
                            .spacing(10)
                            .push(widget::text("tz"))
                            .push(widget::canvas(axis_bar(-500_f32..=500_f32, self.tz)).width(Fill).height(30)).padding(10))
                    )
                    .push(widget::Column::new()
                        .push(widget::Row::new()
                            .spacing(10)
                            .push(widget::text("rx"))
                            .push(widget::canvas(axis_bar(-500_f32..=500_f32, self.rx)).width(Fill).height(30)).padding(10))
                        .push(widget::Row::new()
                            .spacing(10)
                            .push(widget::text("ry"))
                            .push(widget::canvas(axis_bar(-500_f32..=500_f32, self.ry)).width(Fill).height(30)).padding(10))
                        .push(widget::Row::new()
                            .spacing(10)
                            .push(widget::text("rz"))
                            .push(widget::canvas(axis_bar(-500_f32..=500_f32, self.rz)).width(Fill).height(30)).padding(10))
                    )
                )
                .push(widget::Space::new().height(Fill))
                .push(footer_view(self))
            );

            self.toaster
                .view(content, Message::DismissToast, Message::SetHoveredToast)
    }
}
