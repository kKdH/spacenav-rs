use crate::app::app::{Message, ProfileConfigurationView};
use crate::app::views::profile_navigation_configuration_view::navigation_configuration_view;
use crate::app::SpaceNavCockpit;
use iced::{widget, Element};

pub fn profile_navigation_configuration_view(app: &SpaceNavCockpit) -> Element<'_, Message> {

    let content = widget::Row::new();
    let content = match app.active_profile_configuration_view {
        ProfileConfigurationView::Navigation => content.push(navigation_configuration_view(app)),
        ProfileConfigurationView::Keybindings => content
    };

    widget::container(content).into()
}
