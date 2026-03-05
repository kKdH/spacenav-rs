use crate::app::widgets::axis_bar;
use crate::spnav;
use crate::spnav::client::Client;
use iced::{widget, Fill};
use iced::{Element, Subscription};
use iced_aw::{TabBar, TabLabel};
use libspnav::Device;

pub struct App {
    state: State,
    client: Option<Client>,
    device: Option<Device>,
    tx: f32,
    ty: f32,
    tz: f32,
    rx: f32,
    ry: f32,
    rz: f32,
}

#[derive(Debug, Clone)]
enum State {
    Disconnected,
    Connecting,
    Connected,
}

#[derive(Debug, Clone)]
pub enum Message {
    Connect,
    Disconnect,
    ClientEvent(spnav::client::Event),
    TabSelected(usize),
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: State::Disconnected,
            client: None,
            device: None,
            tx: 0_f32,
            ty: 0_f32,
            tz: 0_f32,
            rx: 0_f32,
            ry: 0_f32,
            rz: 0_f32,
        }
    }
}

impl App {

    pub fn update(&mut self, message: Message) {
        match message {
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
            Message::TabSelected(index) => {
                println!("Tab selected: {}", index);
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::run(spnav::client::create)
            .map(Message::ClientEvent)
    }

    pub fn view(&self) -> Element<'_, Message> {

        let connect_button = match self.state {
            State::Disconnected => {
                widget::button("Connect")
                    .on_press(Message::Connect)
            }
            State::Connecting => {
                widget::button("Connecting...")
            }
            State::Connected => {
                widget::button("Disconnect")
                    .on_press(Message::Disconnect)
            }
        };

        widget::Column::new()
            .push(TabBar::new(Message::TabSelected)
                .push(0, TabLabel::Text(String::from("Tab 1")))
                .push(2, TabLabel::Text(String::from("Tab 2")))
                .push(3, TabLabel::Text(String::from("Tab 3")))
            )
            .push(
                widget::Row::new()
                    .spacing(10)
                    .push(widget::text("State:"))
                    .push(widget::text(format!("{:?}", self.state)))
            )
            .push(connect_button)
            .push(
                widget::text(format!("Device: {:?}", self.device))
            )
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
            .into()
    }
}
