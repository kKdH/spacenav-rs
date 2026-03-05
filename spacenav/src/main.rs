use crate::app::App;

mod spnav;
mod app;

fn main() -> Result<(), iced::Error> {
    iced::application(App::default, App::update, App::view)
        .subscription(App::subscription)
        .title("SpaceNav")
        .run()
}
