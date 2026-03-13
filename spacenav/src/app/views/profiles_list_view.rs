use crate::app::app::Message;
use crate::app::SpaceNavCockpit;
use iced::advanced::image::Handle;
use iced::mouse::Interaction;
use iced::widget::image;
use iced::{color, widget, Border, Element};
use iced_font_awesome::fa_icon_solid;
use spacenav_settings::{Profile, ProfileId};

pub fn profile_list_view(app: &SpaceNavCockpit) -> Element<'_, Message> {

    const LIST_WIDTH: f32 = 240.0;

    let profiles = app.profiles.profiles.iter()
        .fold(widget::Column::new(), |column, (profile_id, profile)| {
            let selected = app.selected_profile.as_ref()
                .map_or(false, |selected_id| selected_id == profile_id);
            let profile_icon = app.profiles_icon_handles.get(profile_id).cloned();
            column.push(profiles_list_item(profile_id, profile, profile_icon, selected))
        })
        .spacing(10);

    let content = widget::Column::new()
        .spacing(10)
        .push(widget::Text::new("Profiles"))
        .push(widget::Scrollable::new(profiles));

    widget::Container::new(content)
        .padding(10)
        .width(LIST_WIDTH)
        .height(iced::Fill)
        .style(|theme| widget::container::rounded_box(theme))
        .into()
}

fn profiles_list_item(profile_id: &ProfileId, profile: &Profile, icon: Option<image::Handle>, selected: bool) -> Element<'static, Message> {
    widget::MouseArea::new(
        widget::Container::new(
            widget::Row::new()
                .spacing(10)
                .push(profile_icon(icon))
                .push(widget::Text::new(Clone::clone(&profile.name))
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
                widget::container::rounded_box(theme)
                    .background(theme.palette().primary)
            }
            else {
                widget::container::rounded_box(theme)
                    .background(theme.extended_palette().secondary.base.color)
            }
        }))
    .on_press(Message::ProfileSelected(Clone::clone(&profile_id)))
    .interaction(Interaction::Pointer)
    .into()
}

fn profile_icon(icon: Option<Handle>) -> Element<'static, Message> {
    const ICON_SIZE: f32 = 42.0;
    let icon: Element<'static, Message> = match icon {
        None => fa_icon_solid("sliders")
            .size(ICON_SIZE)
            .color(color!(255, 255, 255))
            .into(),
        Some(icon) => widget::Image::new(icon)
            .width(ICON_SIZE)
            .height(ICON_SIZE)
            .into(),
    };
    widget::Container::new(icon)
        .padding(10)
        .style(|theme| widget::container::rounded_box(theme))
        .into()
}
