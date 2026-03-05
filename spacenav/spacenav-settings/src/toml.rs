use crate::profiles::Profiles;
use crate::{ReadProfileError, WriteProfileError};
use std::path::Path;

pub fn read_profiles_toml<P: AsRef<Path>>(path: P) -> Result<Profiles, ReadProfileError> {

    let content = std::fs::read_to_string(&path)
        .map_err(|cause| ReadProfileError::Io { message: cause.to_string(), path: path.as_ref().to_path_buf() })?;

    let profiles = deserialize_toml(&content)?;

    Ok(profiles)
}

pub fn write_profiles_toml<P: AsRef<Path>>(profile: &Profiles, path: P) -> Result<(), WriteProfileError> {

    let content = serialize_toml(profile)?;

    std::fs::write(&path, content)
        .map_err(|cause| WriteProfileError::Io { message: cause.to_string(), path: path.as_ref().to_path_buf() })?;

    Ok(())
}

fn deserialize_toml(content: &str) -> Result<Profiles, ReadProfileError> {
    let profiles: Profiles = toml::from_str(content)
        .map_err(|cause| ReadProfileError::Deserialize { message: cause.to_string() })?;
    Ok(profiles)
}


fn serialize_toml(profiles: &Profiles) -> Result<String, WriteProfileError> {
    let content = toml::to_string_pretty(&profiles)
        .map_err(|cause| WriteProfileError::Serialize { message: cause.to_string() })?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    pub use googletest::prelude::*;
    use indoc::indoc;

    #[test]
    fn test_read_and_write_toml() {

        let example = indoc! {r#"
            [profiles.p1]
            name = "Profile 1"

            [profiles.p2]
            name = "Profile 2"
        "#};

        let profiles = deserialize_toml(example).unwrap();

        assert_that!(profiles.profiles, len(eq(2)));
        assert_that!(profiles.profiles["p1"].name, eq("Profile 1"));
        assert_that!(profiles.profiles["p2"].name, eq("Profile 2"));

        let content = serialize_toml(&profiles).unwrap();

        assert_that!(content, eq(example));
    }
}
