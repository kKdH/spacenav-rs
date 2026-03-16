use std::collections::BTreeMap;
use std::fmt::Formatter;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Profiles {
    pub profiles: BTreeMap<ProfileId, Profile>,
}

impl Profiles {
    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct ProfileId(String);

impl ProfileId {

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for ProfileId {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(ProfileId(value))
    }
}

impl TryFrom<&str> for ProfileId {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ProfileId::try_from(value.to_string())
    }
}

impl From<ProfileId> for String {
    fn from(value: ProfileId) -> Self {
        value.0
    }
}

impl std::fmt::Display for ProfileId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Profile {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub icon: ProfileIcon,
    #[cfg_attr(feature = "serde", serde(default))]
    pub motions: BTreeMap<MotionFunctionName, MotionFunctionSettings>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub keybindings: Vec<Keybinding>,
}

impl Profile {
    pub fn new(name: String) -> Self {
        Self {
            name,
            icon: ProfileIcon::None,
            motions: BTreeMap::new(),
            keybindings: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ProfileIcon {
    #[default]
    None,
    Path { path: String },
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MotionFunctionName {
    LeftRight,
    UpDown,
    FwdBwd,
    Pitch,
    Yaw,
    Roll,
}

impl MotionFunctionName {

    pub const MOTION_FUNCTION_NAMES: &'static [MotionFunctionName] = &[
        MotionFunctionName::LeftRight,
        MotionFunctionName::UpDown,
        MotionFunctionName::FwdBwd,
        MotionFunctionName::Pitch,
        MotionFunctionName::Yaw,
        MotionFunctionName::Roll,
    ];
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MotionFunctionSettings {
    pub axis: u8,
    pub speed: f32,
    pub threshold: u8,
    #[cfg_attr(feature = "serde", serde(default))]
    pub inverted: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub disabled: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum Keybinding {
    SelectProfile { profile: Option<ProfileId>, button: Option<u8> },
    PreviousProfile { button: Option<u8> },
    NextProfile { button: Option<u8> },
}
