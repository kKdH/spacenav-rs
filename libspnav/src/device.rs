use std::fmt::Formatter;

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
