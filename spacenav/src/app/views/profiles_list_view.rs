use crate::app::app::Message;
use crate::app::SpaceNavCockpit;
use iced::{widget, Element};
use iced::mouse::Interaction;
use spacenav_settings::Profile;

pub fn profile_list_view(app: &SpaceNavCockpit) -> Element<'_, Message> {

    let profiles = app.profiles.profiles.iter()
        .fold(widget::Column::new(), |column, (id, profile)| {
            let selected = app.selected_profile.as_ref()
                .map_or(false, |selected_id| selected_id == id);
            column.push(profiles_list_item(id, profile, selected))
        })
        .spacing(10);

    let content = widget::Column::new()
        .spacing(10)
        .push(widget::Text::new("Profiles"))
        .push(widget::Scrollable::new(profiles));

    widget::Container::new(content)
        .padding(10)
        .width(200)
        .height(iced::Fill)
        .style(|theme| widget::container::rounded_box(theme))
        .into()
}

fn profiles_list_item(id: &String, profile: &Profile, selected: bool) -> Element<'static, Message> {
    widget::MouseArea::new(
        widget::Container::new(
            widget::Row::new()
                // .push(widget::Image::new())
                .push(widget::Text::new(Clone::clone(&profile.title))
                    .font(iced::Font {
                        weight: if selected { iced::font::Weight::Semibold } else { iced::font::Weight::Normal },
                        ..Default::default()
                    })
                )
        )
        .width(iced::Fill)
        .padding(10)
        .style(move |theme| {
            if selected {
                widget::container::primary(theme)
            }
            else {
                widget::container::secondary(theme)
            }
        }))
    .on_press(Message::ProfileSelected(Clone::clone(&id)))
    .interaction(Interaction::Pointer)
    .into()
}
