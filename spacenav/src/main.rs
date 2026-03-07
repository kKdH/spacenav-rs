use crate::app::SpaceNavCockpit;
use iced::window;
use iced::window::settings::PlatformSpecific;
use image::ImageFormat;
use shadow_rs::shadow;
use crate::assets::APP_ICON;

mod app;
mod spnav;
mod util;
mod assets;

shadow!(build);

fn main() -> Result<(), iced::Error> {
    iced::application(SpaceNavCockpit::default, SpaceNavCockpit::update, SpaceNavCockpit::view)
        .subscription(SpaceNavCockpit::subscription)
        .title("SpaceNav Cockpit")
        .window(window::Settings {
            platform_specific: PlatformSpecific {
                application_id: String::from("spacenav"),
                ..Default::default()
            },
            icon: Some(window::icon::from_file_data(APP_ICON, Some(ImageFormat::WebP))
                .expect("Failed to load embedded app icon")
            ),
            ..Default::default()
        })
        .run()
}
