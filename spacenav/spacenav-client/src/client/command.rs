use tokio::sync::{mpsc, oneshot};

pub use libspnav::{CloseError, OpenError};
use libspnav::{GetDeviceError, SetAxesInvertedError, SetAxesSpeedError, SetAxesThresholdError};

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
    SetAxesSpeed {
        speed: [f32; 6],
        reply: oneshot::Sender<Result<(), SetAxesSpeedError>>,
    },
    SetAxesThreshold {
        threshold: [i32; 6],
        reply: oneshot::Sender<Result<(), SetAxesThresholdError>>,
    },
    SetAxesInverted {
        inverted: [bool; 6],
        reply: oneshot::Sender<Result<(), SetAxesInvertedError>>,
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

    pub fn new_command_set_axes_speed(speed: [f32; 6], reply: oneshot::Sender<Result<(), SetAxesSpeedError>>) -> Self {
        Self::SetAxesSpeed {
            speed,
            reply,
        }
    }

    pub fn new_command_set_axes_threshold(threshold: [i32; 6], reply: oneshot::Sender<Result<(), SetAxesThresholdError>>) -> Self {
        Self::SetAxesThreshold {
            threshold,
            reply,
        }
    }

    pub fn new_command_set_axes_inverted(inverted: [bool; 6], reply: oneshot::Sender<Result<(), SetAxesInvertedError>>) -> Self {
        Self::SetAxesInverted {
            inverted,
            reply,
        }
    }
}
