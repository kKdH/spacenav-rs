use iced::widget::image;

pub const APP_ICON: &[u8] = include_bytes!("../assets/app-icon.webp");

pub const MOTION_FWD_BWD_IMAGE: &[u8] = include_bytes!("../assets/fwd-bwd.webp");
pub const MOTION_LEFT_RIGHT_IMAGE: &[u8] = include_bytes!("../assets/left-right.webp");
pub const MOTION_UP_DOWN_IMAGE: &[u8] = include_bytes!("../assets/up-down.webp");
pub const MOTION_ROLL_IMAGE: &[u8] = include_bytes!("../assets/roll.webp");
pub const MOTION_PITCH_IMAGE: &[u8] = include_bytes!("../assets/pitch.webp");
pub const MOTION_YAW_IMAGE: &[u8] = include_bytes!("../assets/yaw.webp");

pub struct ImageHandles {
    motion_fwd_bwd: image::Handle,
    motion_left_right: image::Handle,
    motion_up_down: image::Handle,
    motion_roll: image::Handle,
    motion_pitch: image::Handle,
    motion_yaw: image::Handle,
}

impl ImageHandles {

    pub fn new() -> Self {
        Self {
            motion_fwd_bwd: image::Handle::from_bytes(MOTION_FWD_BWD_IMAGE),
            motion_left_right: image::Handle::from_bytes(MOTION_LEFT_RIGHT_IMAGE),
            motion_up_down: image::Handle::from_bytes(MOTION_UP_DOWN_IMAGE),
            motion_roll: image::Handle::from_bytes(MOTION_ROLL_IMAGE),
            motion_pitch: image::Handle::from_bytes(MOTION_PITCH_IMAGE),
            motion_yaw: image::Handle::from_bytes(MOTION_YAW_IMAGE),
        }
    }

    pub fn motion_fwd_bwd(&self) -> image::Handle {
        Clone::clone(&self.motion_fwd_bwd)
    }

    pub fn motion_left_right(&self) -> image::Handle {
        Clone::clone(&self.motion_left_right)
    }

    pub fn motion_up_down(&self) -> image::Handle {
        Clone::clone(&self.motion_up_down)
    }

    pub fn motion_roll(&self) -> image::Handle {
        Clone::clone(&self.motion_roll)
    }

    pub fn motion_pitch(&self) -> image::Handle {
        Clone::clone(&self.motion_pitch)
    }

    pub fn motion_yaw(&self) -> image::Handle {
        Clone::clone(&self.motion_yaw)
    }

    pub fn all_axes(&self) -> Vec<image::Handle> {
        vec![
            self.motion_left_right(),
            self.motion_up_down(),
            self.motion_fwd_bwd(),
            self.motion_pitch(),
            self.motion_yaw(),
            self.motion_roll(),
        ]
    }
}
