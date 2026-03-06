use crate::app::app::{Message, State};
use crate::app::SpaceNavCockpit;
use iced::widget::button::Status;
use iced::{color, widget, Theme};
use iced::{Element, Fill};
use iced_font_awesome::fa_icon_solid;


pub fn header_view(app: &SpaceNavCockpit) -> Element<'_, Message> {

    const ICON_SIZE: f32 = 40.0;

    fn rounded_button_style(theme: &Theme, status: Status) -> widget::button::Style {
        let mut style = widget::button::primary(theme, status);
        style.border = iced::border::rounded(5.0);
        style
    };

    let connect_button = match app.state {
        State::Disconnected => {
            widget::button(
                fa_icon_solid("plug")
                    .size(ICON_SIZE)
                    .color(color!(255, 255, 255))
            )
            .style(rounded_button_style)
            .on_press(Message::Connect)
        }
        State::Connecting => {
            widget::button(
                fa_icon_solid("plug-circle-bolt")
                    .size(ICON_SIZE)
                    .color(color!(255, 255, 255))
            )
            .style(rounded_button_style)
        }
        State::Connected => {
            widget::button(
                fa_icon_solid("plug-circle-xmark")
                    .size(ICON_SIZE)
                    .color(color!(255, 255, 255))
            )
            .style(rounded_button_style)
            .on_press(Message::Disconnect)
        }
    };

    let content = widget::Column::new()
        .padding(10)
        .spacing(10)
        .push(
            widget::Row::new()
                .spacing(10)
                .push(connect_button)
                .push(
                    widget::button(
                        fa_icon_solid("file-export")
                            .size(ICON_SIZE)
                            .color(color!(255, 255, 255))
                    )
                    .style(rounded_button_style)
                    .on_press(Message::LoadSettings)
                )
                .push(
                    widget::button(
                        fa_icon_solid("file-import")
                            .size(ICON_SIZE)
                            .color(color!(255, 255, 255))
                    )
                    .style(rounded_button_style)
                    .on_press(Message::StoreSettings)
                )
                .push(widget::Space::new().width(Fill))
                .push(widget::text("State:"))
                .push(widget::text(format!("{:?}", app.state)))
        )
        .push(
            widget::text(format!("Device: {:?}", app.device))
        );

    widget::container(content).into()
}
