use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Profiles {
    pub profiles: BTreeMap<String, Profile>,
}

impl Profiles {
    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Profile {
    pub title: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub navigation: BTreeMap<NavigationFunctionName, NavigationFunctionSettings>,
}

impl Profile {
    pub fn new(name: String) -> Self {
        Self {
            title: name,
            navigation: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NavigationFunctionName {
    LeftRight,
    UpDown,
    FwdBwd,
    Pitch,
    Yaw,
    Roll,
}

impl NavigationFunctionName {

    pub const FUNCTION_NAMES: &'static [NavigationFunctionName] = &[
        NavigationFunctionName::LeftRight,
        NavigationFunctionName::UpDown,
        NavigationFunctionName::FwdBwd,
        NavigationFunctionName::Pitch,
        NavigationFunctionName::Yaw,
        NavigationFunctionName::Roll,
    ];
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NavigationFunctionSettings {
    pub axis: u8,
    pub speed: f32,
    pub threshold: u8,
    pub invert: bool,
}
