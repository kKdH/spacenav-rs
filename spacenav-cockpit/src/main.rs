use crate::app::SpaceNavCockpit;
use crate::assets::APP_ICON;
use iced::{window, Size};
use iced::window::settings::PlatformSpecific;
use image::ImageFormat;
use shadow_rs::shadow;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod app;
mod util;
mod assets;

shadow!(build);

fn main() -> Result<(), iced::Error> {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_writer(std::io::stdout)
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("iced=warn".parse().unwrap())
                .add_directive("spacenav=debug".parse().unwrap())
        )
        .init();

    info!("Starting SpaceNav Cockpit");

    iced::application(SpaceNavCockpit::create, SpaceNavCockpit::update, SpaceNavCockpit::view)
        .subscription(SpaceNavCockpit::subscription)
        .title("SpaceNav Cockpit")
        .window(window::Settings {
            platform_specific: PlatformSpecific {
                application_id: String::from("spacenav"),
                ..Default::default()
            },
            min_size: Some(Size::new(900_f32, 860_f32)),
            icon: Some(window::icon::from_file_data(APP_ICON, Some(ImageFormat::WebP))
                .expect("Failed to load embedded app icon")
            ),
            ..Default::default()
        })
        .run()
}
