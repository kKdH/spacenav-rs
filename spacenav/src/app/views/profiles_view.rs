use crate::app::app::Message;
use crate::app::views::profile_configuration_view::profile_configuration_view;
use crate::app::views::profiles_list_view::profile_list_view;
use crate::app::SpaceNavCockpit;
use iced::{widget, Element};

pub fn profiles_view(app: &SpaceNavCockpit) -> Element<'_, Message> {

    let content = widget::Row::new()
        .padding(10)
        .spacing(10)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .push(profile_list_view(app))
        .push(profile_configuration_view(app));

    widget::container(content).into()
}
