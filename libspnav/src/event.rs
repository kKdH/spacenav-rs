use std::ffi::{c_int, c_uint};
use crate::libspnav;

#[derive(Debug, Clone)]
pub enum Event {
    Motion(MotionEvent),
    Button(ButtonEvent),
    Device(DeviceEvent),
    Configuration(ConfigurationEvent),
    Axis(AxisEvent),
}

#[derive(Debug, Clone)]
pub struct MotionEvent {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rx: i32,
    pub ry: i32,
    pub rz: i32,
    pub period: u32,
}

#[derive(Copy, Clone, Debug)]
pub struct ButtonEvent {
    pub pressed: bool,
    pub button: i32,
}

#[derive(Debug, Clone)]
pub struct DeviceEvent {}

#[derive(Debug, Clone)]
pub struct ConfigurationEvent {}

#[derive(Debug, Clone)]
pub struct AxisEvent {
    pub index: i32,
    pub value: i32,
}

#[derive(Debug, Clone)]
pub struct PollError;

pub fn poll() -> Result<Event, PollError> {
    let mut event = libspnav::spnav_event::default();
    let event_type = unsafe {
        libspnav::spnav_wait_event(&mut event) as c_uint
    };
    let event = match event_type {
        libspnav::SPNAV_EVENT_MOTION => Event::Motion(motion_event_from_c(&event)),
        libspnav::SPNAV_EVENT_BUTTON | libspnav::SPNAV_EVENT_RAWBUTTON => Event::Button(button_event_from_c(&event)),
        libspnav::SPNAV_EVENT_DEV => Event::Device(device_event_from_c(&event)),
        libspnav::SPNAV_EVENT_CFG => Event::Configuration(configuration_event_from_c(&event)),
        libspnav::SPNAV_EVENT_RAWAXIS => Event::Axis(axis_event_from_c(&event)),
        _ => return Err(PollError),
    };
    Ok(event)
}

fn motion_event_from_c(event: &libspnav::spnav_event) -> MotionEvent {
    unsafe {
        assert_eq!(event.type_ as u32, libspnav::SPNAV_EVENT_MOTION);
        MotionEvent {
            x: event.motion.x,
            y: event.motion.y,
            z: event.motion.z,
            rx: event.motion.rx,
            ry: event.motion.ry,
            rz: event.motion.rz,
            period: event.motion.period,
        }
    }
}

fn button_event_from_c(event: &libspnav::spnav_event) -> ButtonEvent {
    unsafe {
        assert!(event.type_ as u32 == libspnav::SPNAV_EVENT_BUTTON || event.type_ as u32 == libspnav::SPNAV_EVENT_RAWBUTTON);
        ButtonEvent {
            pressed: c_int_to_bool(event.button.press),
            button: event.button.bnum,
        }
    }
}

fn device_event_from_c(event: &libspnav::spnav_event) -> DeviceEvent {
    unsafe {
        assert_eq!(event.type_ as u32, libspnav::SPNAV_EVENT_DEV);
        DeviceEvent {}
    }
}

fn configuration_event_from_c(event: &libspnav::spnav_event) -> ConfigurationEvent {
    unsafe {
        assert_eq!(event.type_ as u32, libspnav::SPNAV_EVENT_CFG);
        ConfigurationEvent {}
    }
}

fn axis_event_from_c(event: &libspnav::spnav_event) -> AxisEvent {
    unsafe {
        assert_eq!(event.type_ as u32, libspnav::SPNAV_EVENT_RAWAXIS);
        AxisEvent {
            index: event.axis.idx,
            value: event.axis.value,
        }
    }
}

#[inline]
fn c_int_to_bool(value: c_int) -> bool {
    value != 0
}
