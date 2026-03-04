mod device;
mod open;
mod close;

pub use device::Device;
pub use open::{open, OpenError};
pub use close::{close, CloseError};

#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod libspnav {
    include!(concat!(env!("OUT_DIR"), "/libspnav.rs"));
}
