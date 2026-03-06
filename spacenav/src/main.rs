use crate::app::SpaceNavCockpit;
use iced::window;
use iced::window::settings::PlatformSpecific;
use image::ImageFormat;
use shadow_rs::shadow;

mod app;
mod spnav;
mod util;

shadow!(build);

fn main() -> Result<(), iced::Error> {
    let app_icon = include_bytes!("../assets/app-icon.webp");
    iced::application(SpaceNavCockpit::default, SpaceNavCockpit::update, SpaceNavCockpit::view)
        .subscription(SpaceNavCockpit::subscription)
        .title("SpaceNav Cockpit")
        .window(window::Settings {
            platform_specific: PlatformSpecific {
                application_id: String::from("spacenav"),
                ..Default::default()
            },
            icon: Some(window::icon::from_file_data(app_icon, Some(ImageFormat::WebP))
                .expect("Failed to load embedded app icon")
            ),
            ..Default::default()
        })
        .run()
}
