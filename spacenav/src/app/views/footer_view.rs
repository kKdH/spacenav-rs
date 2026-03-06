use crate::app::app::{Message, State};
use crate::app::SpaceNavCockpit;
use iced::widget;
use iced::{Element, Fill};

pub fn footer_view(app: &SpaceNavCockpit) -> Element<'_, Message> {

    let content = widget::Row::new()
        .padding(10)
        .spacing(10)
        .push(
            widget::text(format!("Version: {} ({})", crate::build::PKG_VERSION, crate::build::SHORT_COMMIT))
                .width(Fill)
                .style(widget::text::secondary)
        );

    widget::container(content).into()
}
