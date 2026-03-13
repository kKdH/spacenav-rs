use std::collections::btree_map::Iter;
use directories::ProjectDirs;
use iced::widget::image;
use spacenav_settings::{Profile, ProfileIcon, ProfileId, Profiles};
use std::collections::BTreeMap;
use std::path::PathBuf;

pub fn load_profiles() -> Result<Profiles, ()> {
    let path = profile_toml_path()?;

    if !path.exists() {
        return Ok(Profiles::default());
    }

    let profiles =
        spacenav_settings::read_profiles_toml(path).map_err(|cause| println!("Cause: {cause}"))?;

    Ok(profiles)
}

pub fn store_profiles(profiles: &Profiles) -> Result<(), ()> {
    spacenav_settings::write_profiles_toml(profiles, profile_toml_path()?).map_err(|_| ())?;

    Ok(())
}

pub fn load_profile_icons<'a>(profiles: impl Iterator<Item = (&'a ProfileId, &'a Profile)>) -> Result<BTreeMap<ProfileId, image::Handle>, ()> {

    let config_dir = config_dir()?;
    let mut loaded_icons = BTreeMap::<ProfileId, image::Handle>::new();

    for (profile_id, profile) in profiles {
        match &profile.icon {
            ProfileIcon::None => {}
            ProfileIcon::Path { path } => {
                let path = PathBuf::from(path);
                let path = if path.is_relative() {
                    config_dir.join(path)
                }
                else {
                    path
                };
                let icon = image::Handle::from_path(path);
                loaded_icons.insert(Clone::clone(profile_id), icon);
            }
        }
    }

    Ok(loaded_icons)
}

fn config_dir() -> Result<PathBuf, ()> {
    let proj_dirs = ProjectDirs::from("", "", "spacenav").ok_or(())?;
    Ok(proj_dirs.config_dir().to_path_buf())
}

fn profile_toml_path() -> Result<PathBuf, ()> {
    Ok(config_dir()?.join("profiles.toml"))
}
