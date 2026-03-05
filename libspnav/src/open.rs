use crate::device::{Device, DeviceType};
use crate::libspnav;
use std::ffi::{c_char, c_int, c_uint};

#[derive(Debug)]
pub enum OpenError {
    Connect,
    RetrieveDeviceName,
    RetrieveDeviceType,
    RetrieveDeviceAxes,
    RetrieveDeviceButtons,
}

impl std::error::Error for OpenError {}

impl std::fmt::Display for OpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OpenError::Connect => write!(f, "Failed to connect to the daemon!"),
            OpenError::RetrieveDeviceName => write!(f, "Failed to retrieve the device's name!"),
            OpenError::RetrieveDeviceType => write!(f, "Failed to retrieve the device's type!"),
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
            ty: get_device_type()?,
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

fn get_device_type() -> Result<DeviceType, OpenError> {

    let result = unsafe {
        libspnav::spnav_dev_type() as c_uint
    };

    match result {
        libspnav::SPNAV_DEV_UNKNOWN => Ok(DeviceType::Unknown),
        libspnav::SPNAV_DEV_SB2003 => Ok(DeviceType::Spaceball2003),
        libspnav::SPNAV_DEV_SB3003 => Ok(DeviceType::Spaceball3003),
        libspnav::SPNAV_DEV_SB4000 => Ok(DeviceType::Spaceball4000),
        libspnav::SPNAV_DEV_SM => Ok(DeviceType::MagellanSpaceMouse),
        libspnav::SPNAV_DEV_SM5000 => Ok(DeviceType::Spaceball5000Serial),
        libspnav::SPNAV_DEV_SMCADMAN => Ok(DeviceType::ConnexionCadManSerial),
        libspnav::SPNAV_DEV_PLUSXT => Ok(DeviceType::SpaceMousePlusXT),
        libspnav::SPNAV_DEV_CADMAN => Ok(DeviceType::ConnexionCadManUsb),
        libspnav::SPNAV_DEV_SMCLASSIC => Ok(DeviceType::SpaceMouseClassic),
        libspnav::SPNAV_DEV_SB5000 => Ok(DeviceType::Spaceball5000Usb),
        libspnav::SPNAV_DEV_STRAVEL => Ok(DeviceType::SpaceTraveller),
        libspnav::SPNAV_DEV_SPILOT => Ok(DeviceType::SpacePilot),
        libspnav::SPNAV_DEV_SNAV => Ok(DeviceType::SpaceNavigator),
        libspnav::SPNAV_DEV_SEXP => Ok(DeviceType::SpaceExplorer),
        libspnav::SPNAV_DEV_SNAVNB => Ok(DeviceType::SpaceNavigatorForNotebooks),
        libspnav::SPNAV_DEV_SPILOTPRO => Ok(DeviceType::SpacePilotPro),
        libspnav::SPNAV_DEV_SMPRO => Ok(DeviceType::SpaceMousePro),
        libspnav::SPNAV_DEV_NULOOQ => Ok(DeviceType::NuLOOQ),
        libspnav::SPNAV_DEV_SMW => Ok(DeviceType::SpaceMouseWireless),
        libspnav::SPNAV_DEV_SMPROW => Ok(DeviceType::SpaceMouseProWireless),
        libspnav::SPNAV_DEV_SMENT => Ok(DeviceType::SpaceMouseEnterprise),
        libspnav::SPNAV_DEV_SMCOMP => Ok(DeviceType::SpaceMouseCompact),
        libspnav::SPNAV_DEV_SMMOD => Ok(DeviceType::SpaceMouseModule),
        _ => Err(OpenError::RetrieveDeviceType)
    }
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
