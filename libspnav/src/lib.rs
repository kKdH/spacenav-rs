mod device;
mod open;
mod close;
mod event;

pub use close::{close, CloseError};
pub use device::Device;
pub use event::{poll, AxisEvent, ButtonEvent, ConfigurationEvent, DeviceEvent, Event, MotionEvent, PollError};
pub use open::{open, OpenError};

#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod libspnav {
    include!(concat!(env!("OUT_DIR"), "/libspnav.rs"));
}
