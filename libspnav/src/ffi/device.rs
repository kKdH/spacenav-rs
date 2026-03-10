use std::ffi::{c_char, c_int, c_uint};
use std::fmt::Formatter;
use crate::{libspnav, OpenError};

#[derive(Debug, Clone)]
pub struct Device {
    pub name: String,
    pub ty: DeviceType,
    pub axes: u32,
    pub buttons: u32,
}

#[derive(Debug, Clone)]
pub enum DeviceType {
    Unknown,
    /// Spaceball 1003/2003/2003C
    Spaceball2003,
    /// Spaceball 3003/3003C
    Spaceball3003,
    /// Spaceball 4000FLX/5000FLX
    Spaceball4000,
    MagellanSpaceMouse,
    /// Spaceball 5000 (spacemouse protocol)
    Spaceball5000Serial,
    /// 3Dconnexion CadMan (spacemouse protocol)
    ConnexionCadManSerial,
    SpaceMousePlusXT,
    /// 3Dconnexion CadMan (USB version)
    ConnexionCadManUsb,
    SpaceMouseClassic,
    /// Spaceball 5000 (USB version)
    Spaceball5000Usb,
    SpaceTraveller,
    SpacePilot,
    SpaceNavigator,
    SpaceExplorer,
    SpaceNavigatorForNotebooks,
    SpacePilotPro,
    SpaceMousePro,
    NuLOOQ,
    SpaceMouseWireless,
    SpaceMouseProWireless,
    SpaceMouseEnterprise,
    SpaceMouseCompact,
    SpaceMouseModule,
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceType::Unknown => write!(f, "Unknown"),
            DeviceType::Spaceball2003 => write!(f, "Spaceball 1003/2003/2003C"),
            DeviceType::Spaceball3003 => write!(f, "Spaceball 3003/3003C"),
            DeviceType::Spaceball4000 => write!(f, "Spaceball 4000FLX/5000FLX"),
            DeviceType::MagellanSpaceMouse => write!(f, "Magellan SpaceMouse"),
            DeviceType::Spaceball5000Serial => write!(f, "Spaceball 5000 (spacemouse protocol)"),
            DeviceType::ConnexionCadManSerial => write!(f, "3Dconnexion CadMan (spacemouse protocol)"),
            DeviceType::SpaceMousePlusXT => write!(f, "SpaceMouse Plus XT"),
            DeviceType::ConnexionCadManUsb => write!(f, "3Dconnexion CadMan (USB version)"),
            DeviceType::SpaceMouseClassic => write!(f, "SpaceMouse Classic"),
            DeviceType::Spaceball5000Usb => write!(f, "Spaceball 5000 (USB version)"),
            DeviceType::SpaceTraveller => write!(f, "Space Traveller"),
            DeviceType::SpacePilot => write!(f, "Space Pilot"),
            DeviceType::SpaceNavigator => write!(f, "Space Navigator"),
            DeviceType::SpaceExplorer => write!(f, "Space Explorer"),
            DeviceType::SpaceNavigatorForNotebooks => write!(f, "Space Navigator for Notebooks"),
            DeviceType::SpacePilotPro => write!(f, "Space Pilot Pro"),
            DeviceType::SpaceMousePro => write!(f, "SpaceMouse Pro"),
            DeviceType::NuLOOQ => write!(f, "NuLOOQ"),
            DeviceType::SpaceMouseWireless => write!(f, "SpaceMouse Wireless"),
            DeviceType::SpaceMouseProWireless => write!(f, "SpaceMouse Pro Wireless"),
            DeviceType::SpaceMouseEnterprise => write!(f, "SpaceMouse Enterprise"),
            DeviceType::SpaceMouseCompact => write!(f, "SpaceMouse Compact"),
            DeviceType::SpaceMouseModule => write!(f, "SpaceMouse Module"),
        }
    }
}

#[derive(Debug)]
pub enum GetDeviceError {
    RetrieveDeviceName,
    RetrieveDeviceType,
    RetrieveDeviceAxes,
    RetrieveDeviceButtons,
}

impl std::error::Error for GetDeviceError {}

impl std::fmt::Display for GetDeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GetDeviceError::RetrieveDeviceName => write!(f, "Failed to retrieve the device's name!"),
            GetDeviceError::RetrieveDeviceType => write!(f, "Failed to retrieve the device's type!"),
            GetDeviceError::RetrieveDeviceAxes => write!(f, "Failed to retrieve the device's number of axes!"),
            GetDeviceError::RetrieveDeviceButtons => write!(f, "Failed to retrieve the device's number of buttons!")
        }
    }
}

pub fn get_device() -> Result<Device, GetDeviceError> {
    Ok(
        Device {
            name: get_device_name()?,
            ty: get_device_type()?,
            axes: get_device_axes()?,
            buttons: get_device_buttons()?,
        }
    )
}

fn get_device_name() -> Result<String, GetDeviceError> {
    let buffer = [0_u8; 128];

    let result = unsafe {
        libspnav::spnav_dev_name(buffer.as_ptr() as *mut c_char, buffer.len() as c_int) as i32
    };

    if result < 0 {
        return Err(GetDeviceError::RetrieveDeviceName)
    }

    Ok(String::from_utf8_lossy(&buffer[..result as usize]).to_string())
}

fn get_device_type() -> Result<DeviceType, GetDeviceError> {

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
        _ => Err(GetDeviceError::RetrieveDeviceType)
    }
}

fn get_device_axes() -> Result<u32, GetDeviceError> {

    let result = unsafe {
        libspnav::spnav_dev_axes() as u32
    };

    if result == 0 {
        return Err(GetDeviceError::RetrieveDeviceAxes)
    }

    Ok(result)
}

fn get_device_buttons() -> Result<u32, GetDeviceError> {

    let result = unsafe {
        libspnav::spnav_dev_buttons() as u32
    };

    if result == 0 {
        return Err(GetDeviceError::RetrieveDeviceButtons)
    }

    Ok(result)
}

