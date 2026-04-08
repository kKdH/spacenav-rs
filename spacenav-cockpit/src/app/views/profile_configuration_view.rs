use crate::app::app::{Message, ProfileConfigurationView};
use crate::app::views::profile_keybindings_configuration_view::profile_keybindings_configuration_view;
use crate::app::views::profile_motions_configuration_view::profile_motion_configuration_view;
use crate::app::SpaceNavCockpit;
use iced::{widget, Element};

pub fn profile_configuration_view(app: &SpaceNavCockpit) -> Element<'_, Message> {

    let content = widget::Row::new();
    let content = match app.active_profile_configuration_view {
        ProfileConfigurationView::Motions => content.push(profile_motion_configuration_view(app)),
        ProfileConfigurationView::Keybindings => content.push(profile_keybindings_configuration_view(app)),
    };

    widget::container(content).into()
}
