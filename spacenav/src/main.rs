mod spnav;

use crate::spnav::client::Client;
use iced::widget;
use iced::{Element, Subscription};
use libspnav::Device;

fn main() -> Result<(), iced::Error> {
    iced::application(App::default, App::update, App::view)
        .subscription(App::subscription)
        .title("SpaceNav")
        .run()
}

struct App {
    state: State,
    client: Option<Client>,
    device: Option<Device>,
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
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: State::Disconnected,
            client: None,
            device: None,
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
                    spnav::client::Event::Error => {
                        self.state = State::Disconnected;
                    },
                }
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
            .into()
    }
}
