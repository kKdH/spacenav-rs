use tokio::sync::{mpsc, oneshot};

pub use libspnav::{OpenError, CloseError};
use libspnav::{GetDeviceError, SetIndividualAxesSpeedError, SetGlobalAxesSpeedError};

#[derive(Debug)]
pub enum Command {
    Open {
        name: String,
        reply: oneshot::Sender<Result<(), OpenError>>,
    },
    Close {
        reply: oneshot::Sender<Result<(), CloseError>>,
    },
    Subscribe {
        subscriber: mpsc::Sender<libspnav::Event>,
        reply: oneshot::Sender<Result<(), ()>>,
    },
    GetDevice {
        reply: oneshot::Sender<Result<libspnav::Device, GetDeviceError>>,
    },
    SetIndividualAxesSpeed {
        speed: [f32; 6],
        reply: oneshot::Sender<Result<(), SetIndividualAxesSpeedError>>,
    },
}

impl Command {

    pub fn new_command_open(name: impl Into<String>, reply: oneshot::Sender<Result<(), OpenError>>) -> Self {
        Self::Open {
            name: name.into(),
            reply,
        }
    }

    pub fn new_command_close(reply: oneshot::Sender<Result<(), CloseError>>) -> Self {
        Self::Close {
            reply,
        }
    }

    pub fn new_command_subscribe(subscriber: mpsc::Sender<libspnav::Event>, reply: oneshot::Sender<Result<(), ()>>) -> Self {
        Self::Subscribe {
            subscriber,
            reply,
        }
    }

    pub fn new_command_get_device(reply: oneshot::Sender<Result<libspnav::Device, GetDeviceError>>) -> Self {
        Self::GetDevice {
            reply,
        }
    }

    pub fn new_command_set_individual_axes_speed(speed: [f32; 6], reply: oneshot::Sender<Result<(), SetIndividualAxesSpeedError>>) -> Self {
        Self::SetIndividualAxesSpeed {
            speed,
            reply,
        }
    }
}
