use std::ffi::c_int;
use std::fmt::Formatter;
use crate::libspnav;

#[derive(Debug)]
pub struct SetAxesSpeedError;

impl std::error::Error for SetAxesSpeedError {}

impl std::fmt::Display for SetAxesSpeedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to set axes speed")
    }
}

pub fn set_axes_speed(values: [f32; 6]) -> Result<(), SetAxesSpeedError> {

    let result = unsafe {
        libspnav::spnav_cfg_set_axis_sens(values.as_ptr()) as i32
    };

    if result != 0 {
        return Err(SetAxesSpeedError)
    }

    Ok(())
}

#[derive(Debug)]
pub struct SetAxesThresholdError;

impl std::error::Error for SetAxesThresholdError {}

impl std::fmt::Display for SetAxesThresholdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to set axes threshold")
    }
}

pub fn set_axes_threshold(values: [i32; 6]) -> Result<(), SetAxesThresholdError> {

    for (axis, value) in values.into_iter().enumerate() {
        let result = unsafe {
            libspnav::spnav_cfg_set_deadzone(axis as c_int, value as c_int) as i32
        };
        if result != 0 {
            return Err(SetAxesThresholdError)
        }
    }

    Ok(())
}


#[derive(Debug)]
pub struct SetAxesInvertedError;

impl std::error::Error for SetAxesInvertedError {}

impl std::fmt::Display for SetAxesInvertedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to set axes inverted")
    }
}

pub fn set_axes_inverted(values: [bool; 6]) -> Result<(), SetAxesInvertedError> {

    let mut invert_bits = 0_i32;
    for (shift, value) in values.into_iter().enumerate() {
        invert_bits |= (value as i32) << shift
    }

    let result = unsafe {
        libspnav::spnav_cfg_set_invert(invert_bits as c_int) as i32
    };
    if result != 0 {
        return Err(SetAxesInvertedError)
    }

    Ok(())
}
