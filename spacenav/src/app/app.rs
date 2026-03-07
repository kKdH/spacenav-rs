use crate::app::views;
use crate::app::views::{configuration_view, header_view};
use crate::app::widgets::axis_bar;
use crate::spnav;
use crate::spnav::client::Client;
use crate::util::{load_profiles, store_profiles};
use iced::widget::container;
use iced::{widget, Fill};
use iced::{Element, Subscription};
use iced_toaster::{Toast, ToastId, ToastLevel, Toaster};
use libspnav::Device;
use spacenav_settings::{NavigationFunctionName, Profile, Profiles};
use views::footer_view;
use crate::assets::ImageHandles;

pub struct SpaceNavCockpit {
    pub state: State,
    pub profiles: Profiles,
    pub selected_profile: Option<String>,
    pub client: Option<Client>,
    pub device: Option<Device>,
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
    ClientEvent(spnav::client::Event),
    TabSelected(String),
    PushToast(Toast<Message>),
    DismissToast(ToastId),
    SetHoveredToast(ToastId, bool),
    AxisSpeedChanged { profile: String, axis: NavigationFunctionName, speed: f32 },
    Tick,
}

impl Default for SpaceNavCockpit {
    fn default() -> Self {
        Self {
            state: State::Disconnected,
            profiles: Profiles::default(),
            selected_profile: None,
            client: None,
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
}

impl SpaceNavCockpit {

    pub fn update(&mut self, message: Message) {
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
            }
            Message::Connect => {
                if matches!(self.state, State::Disconnected) {
                    self.state = State::Connecting;
                    self.client.as_mut()
                        .expect("Client should be created before connecting")
                        .send(spnav::client::Message::Connect);
                }
            }
            Message::Disconnect => {
                if matches!(self.state, State::Connected) {
                    self.client.as_mut()
                        .expect("Client should be created before connecting")
                        .send(spnav::client::Message::Disconnect);
                }
            }
            Message::ClientEvent(event) => {
                match event {
                    spnav::client::Event::Created(client) => {
                        self.client = Some(client);
                    }
                    spnav::client::Event::Connected(device) => {
                        self.device = Some(device);
                        self.state = State::Connected;
                    },
                    spnav::client::Event::Disconnected => {
                        self.device = None;
                        self.state = State::Disconnected;
                    },
                    spnav::client::Event::Axis(event) => {
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
                    spnav::client::Event::Error => {
                        self.state = State::Disconnected;
                    },
                }
            }
            Message::TabSelected(profile_id) => {
                self.selected_profile = Some(profile_id);
            }
            Message::PushToast(toast) => {
                self.toaster.push(toast);
            }
            Message::DismissToast(id) => {
                self.toaster.dismiss(id);
            }
            Message::SetHoveredToast(id, hovered) => {
                self.toaster.set_hovered(id, hovered);
            }
            Message::AxisSpeedChanged { profile: profile_id, axis, speed} => {
                if let Some(profile) = self.profiles.profiles.get_mut(&profile_id) {
                    if let Some(axis) = profile.navigation.get_mut(&axis) {
                        let speed = speed.max(0_f32).min(2_f32);
                        let speed = (speed * 100_f32).round() / 100_f32;
                        axis.speed = speed;
                    }
                }
            }
            Message::Tick => {
                self.toaster.dismiss_expired();
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            Subscription::run(spnav::client::create)
                .map(Message::ClientEvent),
            iced::time::every(std::time::Duration::from_millis(200))
                .map(|_| Message::Tick)
        ])
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
