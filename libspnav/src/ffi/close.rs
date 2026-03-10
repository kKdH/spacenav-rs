use crate::libspnav;

#[derive(Debug)]
pub struct CloseError;

impl std::error::Error for CloseError {}

impl std::fmt::Display for CloseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CloseError")
    }
}

pub fn close() -> Result<(), CloseError> {
    let result = unsafe {
        libspnav::spnav_close() as i32
    };
    if result != 0 {
        Err(CloseError)
    }
    else {
        Ok(())
    }
}
