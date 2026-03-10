use std::fmt::Formatter;
use crate::libspnav;

#[derive(Debug)]
pub struct SetGlobalAxesSpeedError;

impl std::error::Error for SetGlobalAxesSpeedError {}

impl std::fmt::Display for SetGlobalAxesSpeedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to set global axes speed")
    }
}

pub fn set_global_axes_speed(value: f32) -> Result<(), SetGlobalAxesSpeedError> {

    let result = unsafe {
        libspnav::spnav_cfg_set_sens(value) as i32
    };

    if result < 0 {
        return Err(SetGlobalAxesSpeedError)
    }

    Ok(())
}

#[derive(Debug)]
pub struct SetIndividualAxesSpeedError;

impl std::error::Error for SetIndividualAxesSpeedError {}

impl std::fmt::Display for SetIndividualAxesSpeedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to set individual axes speed")
    }
}

pub fn set_individual_axes_speed(values: [f32; 6]) -> Result<(), SetIndividualAxesSpeedError> {

    let result = unsafe {
        libspnav::spnav_cfg_set_axis_sens(values.as_ptr()) as i32
    };

    if result != 0 {
        return Err(SetIndividualAxesSpeedError)
    }

    Ok(())
}
