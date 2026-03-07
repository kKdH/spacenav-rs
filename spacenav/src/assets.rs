use iced::widget::image;

pub const APP_ICON: &[u8] = include_bytes!("../assets/app-icon.webp");

pub const NAVIGATION_FWD_BWD_IMAGE: &[u8] = include_bytes!("../assets/fwd-bwd.webp");
pub const NAVIGATION_LEFT_RIGHT_IMAGE: &[u8] = include_bytes!("../assets/left-right.webp");
pub const NAVIGATION_UP_DOWN_IMAGE: &[u8] = include_bytes!("../assets/up-down.webp");
pub const NAVIGATION_ROLL_IMAGE: &[u8] = include_bytes!("../assets/roll.webp");
pub const NAVIGATION_PITCH_IMAGE: &[u8] = include_bytes!("../assets/pitch.webp");
pub const NAVIGATION_YAW_IMAGE: &[u8] = include_bytes!("../assets/yaw.webp");

pub struct ImageHandles {
    navigation_fwd_bwd: image::Handle,
    navigation_left_right: image::Handle,
    navigation_up_down: image::Handle,
    navigation_roll: image::Handle,
    navigation_pitch: image::Handle,
    navigation_yaw: image::Handle,
}

impl ImageHandles {

    pub fn new() -> Self {
        Self {
            navigation_fwd_bwd: image::Handle::from_bytes(NAVIGATION_FWD_BWD_IMAGE),
            navigation_left_right: image::Handle::from_bytes(NAVIGATION_LEFT_RIGHT_IMAGE),
            navigation_up_down: image::Handle::from_bytes(NAVIGATION_UP_DOWN_IMAGE),
            navigation_roll: image::Handle::from_bytes(NAVIGATION_ROLL_IMAGE),
            navigation_pitch: image::Handle::from_bytes(NAVIGATION_PITCH_IMAGE),
            navigation_yaw: image::Handle::from_bytes(NAVIGATION_YAW_IMAGE),
        }
    }

    pub fn navigation_fwd_bwd(&self) -> image::Handle {
        Clone::clone(&self.navigation_fwd_bwd)
    }

    pub fn navigation_left_right(&self) -> image::Handle {
        Clone::clone(&self.navigation_left_right)
    }

    pub fn navigation_up_down(&self) -> image::Handle {
        Clone::clone(&self.navigation_up_down)
    }

    pub fn navigation_roll(&self) -> image::Handle {
        Clone::clone(&self.navigation_roll)
    }

    pub fn navigation_pitch(&self) -> image::Handle {
        Clone::clone(&self.navigation_pitch)
    }

    pub fn navigation_yaw(&self) -> image::Handle {
        Clone::clone(&self.navigation_yaw)
    }

    pub fn all_axes(&self) -> Vec<image::Handle> {
        vec![
            self.navigation_left_right(),
            self.navigation_up_down(),
            self.navigation_fwd_bwd(),
            self.navigation_pitch(),
            self.navigation_yaw(),
            self.navigation_roll(),
        ]
    }
}
