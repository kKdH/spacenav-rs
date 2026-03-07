use directories::ProjectDirs;
use spacenav_settings::Profiles;
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

fn profile_toml_path() -> Result<PathBuf, ()> {
    let proj_dirs = ProjectDirs::from("", "", "spacenav").ok_or(())?;

    let path = proj_dirs.config_dir().join("profiles.toml");

    Ok(path)
}
