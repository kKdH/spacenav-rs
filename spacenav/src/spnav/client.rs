use futures::channel::mpsc;
use futures::channel::mpsc::Sender;
use iced::task::{sipper, Sipper};
use iced::Never;
use libspnav::Device;

#[derive(Debug, Clone)]
pub enum Message {
    Connect,
    Disconnect,
}

#[derive(Debug, Clone)]
pub enum Event {
    Created(Client),
    Connected(Device),
    Disconnected,
    Error
}

#[derive(Debug, Clone)]
pub struct Client {
    sender: Sender<Message>
}

impl Client {

    fn new(sender: Sender<Message>) -> Self {
        Self {
            sender
        }
    }

    pub fn send(&mut self, message: Message) {
        self.sender
            .try_send(message)
            .expect("Failed to send a message to the client");
    }
}

pub fn create() -> impl Sipper<Never, Event> {
    sipper(async |mut output| {
        loop {
            let (sender, mut receiver) = mpsc::channel::<Message>(64);
            let client = Client::new(sender);

            output.send(Event::Created(Clone::clone(&client))).await;

            loop {
                let Ok(message) = receiver.recv().await else {
                    todo!()
                };
                match message {
                    Message::Connect => {
                        match libspnav::open() {
                            Ok(device) => {
                                output.send(Event::Connected(device)).await;
                            }
                            Err(_) => {
                                todo!()
                            }
                        }
                    }
                    Message::Disconnect => {
                        match libspnav::close() {
                            Ok(_) => {
                                output.send(Event::Disconnected).await;
                            }
                            Err(_) => {
                                todo!()
                            }
                        }
                    }
                }
            }
        }
    })
}
