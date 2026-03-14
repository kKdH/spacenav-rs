use crate::app::app::{Message, ProfileConfigurationView};
use crate::app::views::BORDER_RADIUS;
use crate::app::SpaceNavCockpit;
use iced::widget::image;
use iced::{border, color, widget, Alignment, Border, Element, Theme};
use iced_font_awesome::fa_icon_solid;
use spacenav_settings::{Profile, ProfileId};

pub fn profile_list_view(app: &SpaceNavCockpit) -> Element<'_, Message> {

    const LIST_WIDTH: f32 = 260.0;

    let profiles = app.profiles.profiles.iter()
        .fold(widget::Column::new(), |column, (profile_id, profile)| {
            let selected = app.selected_profile.as_ref()
                .map_or(false, |selected_id| selected_id == profile_id);
            let profile_icon = app.profiles_icon_handles.get(profile_id).cloned();
            column.push(profiles_list_item(profile_id, profile, profile_icon, selected, app.active_profile_configuration_view))
        })
        .spacing(10);

    let content = widget::Column::new()
        .push(widget::Scrollable::new(profiles));

    widget::Container::new(content)
        .width(LIST_WIDTH)
        .height(iced::Fill)
        .style(|_theme| widget::container::Style {
            border: border::rounded(border::bottom_left(BORDER_RADIUS)),
            ..Default::default()
        })
        .into()
}

fn profiles_list_item(
    profile_id: &ProfileId,
    profile: &Profile,
    icon: Option<image::Handle>,
    selected: bool,
    active_configuration_view: ProfileConfigurationView
) -> Element<'static, Message> {

    let mut column = widget::Column::new()
        .push(widget::Button::new(
            widget::Container::new(
                widget::Row::new()
                    .spacing(16)
                    .push(profile_icon(icon))
                    .push(widget::Text::new(Clone::clone(&profile.name))
                        .font(iced::Font {
                            weight: if selected { iced::font::Weight::Semibold } else { iced::font::Weight::Normal },
                            ..Default::default()
                        })
                    )
            )
            .width(iced::Fill)
            .padding(6))
            .style(move |theme, status| {
                let base_style = widget::button::Style {
                    border: border::rounded(BORDER_RADIUS),
                    text_color: theme.palette().text,
                    ..Default::default()
                };
                match (status, selected) {
                    (_, true) => widget::button::Style {
                        border: border::rounded(border::top(BORDER_RADIUS)),
                        background: Some(theme.palette().primary.into()),
                        ..base_style
                    },
                    (widget::button::Status::Hovered, false) => widget::button::Style {
                        background: Some(theme.extended_palette().secondary.weak.color.into()),
                        ..base_style
                    },
                    (widget::button::Status::Active, false) => widget::button::Style {
                        background: Some(theme.extended_palette().secondary.base.color.into()),
                        ..base_style
                    },
                    _ => base_style
                }
            })
        .on_press(Message::ProfileSelected(Clone::clone(&profile_id))));

    if selected {
        column = column
            .push(
                widget::Row::new()
                    .width(iced::Fill)
                    .push(profile_navigation_configuration_button(profile_id, active_configuration_view))
                    .push(profile_keybindings_configuration_button(profile_id, active_configuration_view))
            );
    }

    column.into()
}

fn profile_icon(icon: Option<image::Handle>) -> Element<'static, Message> {
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
        .padding(8)
        .style(|theme| widget::container::Style {
            background: Some(theme.palette().background.into()),
            border: border::rounded(BORDER_RADIUS),
            ..Default::default()
        })
        .into()
}

fn profile_navigation_configuration_button(profile_id: &ProfileId, active_configuration_view: ProfileConfigurationView) -> Element<'static, Message> {
    let active = matches!(active_configuration_view, ProfileConfigurationView::Navigation);
    profile_button(String::from("Navigation"), 0, active, || Message::ProfileNavigationConfigurationViewActivated(Clone::clone(profile_id)))
}

fn profile_keybindings_configuration_button(profile_id: &ProfileId, active_configuration_view: ProfileConfigurationView) -> Element<'static, Message> {
    let active = matches!(active_configuration_view, ProfileConfigurationView::Keybindings);
    profile_button(String::from("Keybindings"), 1, active, || Message::ProfileKeybindingsConfigurationViewActivated(Clone::clone(profile_id)))
}

fn profile_button(text: String, index: usize, active: bool, on_press: impl Fn() -> Message) -> Element<'static, Message> {
    widget::Button::new(
        widget::Text::new(text)
            .align_x(Alignment::Center)
            .width(iced::Fill))
        .on_press(on_press())
        .style(move |theme: &Theme, status| {
            let base_style = widget::button::Style {
                background: Some(theme.extended_palette().secondary.weak.color.into()),
                border: match index {
                    0 => border::rounded(border::bottom_left(BORDER_RADIUS)),
                    1 => border::rounded(border::bottom_right(BORDER_RADIUS)),
                    _ => Border::default(),
                },
                text_color: theme.palette().text,
                ..Default::default()
            };
            match (status, active) {
                (widget::button::Status::Hovered, false) => widget::button::Style {
                    background: Some(theme.extended_palette().secondary.strong.color.into()),
                    ..base_style
                },
                (_, true) => widget::button::Style {
                    background: Some(theme.palette().primary.into()),
                    ..base_style
                },
                _ => widget::button::Style {
                    background: Some(theme.extended_palette().secondary.weak.color.into()),
                    ..base_style
                },
            }
        })
        .into()
}
