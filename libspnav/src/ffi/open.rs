use crate::libspnav;
use std::ffi::CString;
use std::os::fd::{FromRawFd, OwnedFd};

#[derive(Debug)]
pub enum OpenError {
    Connect,
    UnsupportedVersion,
    SetClientName,
    SetEventMask,
    GetFileDescriptor,
}

impl std::error::Error for OpenError {}

impl std::fmt::Display for OpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OpenError::Connect => write!(f, "Failed to connect to the daemon!"),
            OpenError::UnsupportedVersion => write!(f, "Unsupported protocol version of daemon!"),
            OpenError::SetClientName => write!(f, "Failed to set client name!"),
            OpenError::SetEventMask => write!(f, "Failed to set event mask!"),
            OpenError::GetFileDescriptor => write!(f, "Failed to get a file descriptor for the daemon's unix socket!"),
        }
    }
}

pub fn open(name: &str) -> Result<OwnedFd, OpenError> {

    let result = unsafe {
        libspnav::spnav_open() as i32
    };

    if result != 0 {
        return Err(OpenError::Connect)
    }

    let result = unsafe {
        libspnav::spnav_protocol()
    };

    if result != 1 {
        return Err(OpenError::UnsupportedVersion)
    }

    let client_name = CString::new(name).unwrap();
    let result = unsafe {
        libspnav::spnav_client_name(client_name.as_ptr()) as i32
    };

    if result != 0 {
        return Err(OpenError::SetClientName)
    }

    let result = unsafe {
        libspnav::spnav_evmask(libspnav::SPNAV_EVMASK_ALL)
    };

    if result != 0 {
        return Err(OpenError::SetEventMask)
    }

    let result = unsafe {
        libspnav::spnav_fd() as i32
    };

    if result < 0 {
        return Err(OpenError::GetFileDescriptor)
    }

    let fd = unsafe {
        OwnedFd::from_raw_fd(result)
    };

    Ok(fd)
}
