mod ffi;

pub use ffi::axes::{set_axes_inverted, set_axes_mapping, set_axes_speed, set_axes_threshold, SetAxesInvertedError, SetAxesMappingError, SetAxesSpeedError, SetAxesThresholdError};
pub use ffi::close::{close, CloseError};
pub use ffi::device::{get_device, Device, DeviceType, GetDeviceError};
pub use ffi::event::{poll, AxisEvent, ButtonEvent, ButtonState, ConfigurationEvent, DeviceEvent, Event, MotionEvent, PollError};
pub use ffi::open::{open, OpenError};

#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod libspnav {
    include!(concat!(env!("OUT_DIR"), "/libspnav.rs"));
}
