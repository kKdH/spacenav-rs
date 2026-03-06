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
    pub name: String,
}

impl Profile {

    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }
}
