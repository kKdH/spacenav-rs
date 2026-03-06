use crate::app::App;
use iced::window;
use iced::window::settings::PlatformSpecific;
use image::ImageFormat;

mod spnav;
mod app;

fn main() -> Result<(), iced::Error> {
    let app_icon = include_bytes!("../assets/app-icon.webp");
    iced::application(App::default, App::update, App::view)
        .subscription(App::subscription)
        .title("SpaceNav")
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
