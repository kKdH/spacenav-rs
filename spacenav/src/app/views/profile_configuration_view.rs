use iced::{widget, Element};
use crate::app::app::Message;
use crate::app::SpaceNavCockpit;
use crate::app::views::profile_navigation_configuration_view::navigation_configuration_view;

pub fn profile_configuration_view(app: &SpaceNavCockpit) -> Element<'_, Message> {

    let content = widget::Row::new()
        .push(
            navigation_configuration_view(app)
        );

    widget::container(content).into()
}
