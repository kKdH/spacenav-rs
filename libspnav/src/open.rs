use crate::device::Device;
use crate::libspnav;
use std::ffi::{c_char, c_int};

#[derive(Debug)]
pub enum OpenError {
    Connect,
    RetrieveDeviceName,
    RetrieveDeviceAxes,
    RetrieveDeviceButtons,
}

impl std::error::Error for OpenError {}

impl std::fmt::Display for OpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OpenError::Connect => write!(f, "Failed to connect to the daemon!"),
            OpenError::RetrieveDeviceName => write!(f, "Failed to retrieve the device's name!"),
            OpenError::RetrieveDeviceAxes => write!(f, "Failed to retrieve the device's number of axes!"),
            OpenError::RetrieveDeviceButtons => write!(f, "Failed to retrieve the device's number of buttons!")
        }
    }
}

pub fn open() -> Result<Device, OpenError> {

    let result = unsafe {
        libspnav::spnav_open() as i32
    };

    if result != 0 {
        return Err(OpenError::Connect)
    }

    unsafe {
        libspnav::spnav_evmask(libspnav::SPNAV_EVMASK_ALL);
    }

    Ok(
        Device {
            name: get_device_name()?,
            axes: get_device_axes()?,
            buttons: get_device_buttons()?,
        }
    )
}

fn get_device_name() -> Result<String, OpenError> {
    let buffer = [0_u8; 128];

    let result = unsafe {
        libspnav::spnav_dev_name(buffer.as_ptr() as *mut c_char, buffer.len() as c_int) as i32
    };

    if result < 0 {
        return Err(OpenError::RetrieveDeviceName)
    }

    Ok(String::from_utf8_lossy(&buffer[..result as usize]).to_string())
}

fn get_device_axes() -> Result<u32, OpenError> {

    let result = unsafe {
        libspnav::spnav_dev_axes() as u32
    };

    if result == 0 {
        return Err(OpenError::RetrieveDeviceAxes)
    }

    Ok(result)
}

fn get_device_buttons() -> Result<u32, OpenError> {

    let result = unsafe {
        libspnav::spnav_dev_buttons() as u32
    };

    if result == 0 {
        return Err(OpenError::RetrieveDeviceButtons)
    }

    Ok(result)
}
