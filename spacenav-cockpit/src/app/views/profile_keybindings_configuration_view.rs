use crate::app::app::Message;
use crate::app::views::BORDER_RADIUS;
use crate::app::SpaceNavCockpit;
use iced::alignment::Vertical;
use iced::{border, widget, Alignment, Element, Fill, Theme};
use spacenav_settings::{Keybinding, KeybindingButton, Profile, ProfileId};
use std::fmt::Formatter;

pub fn profile_keybindings_configuration_view(app: &SpaceNavCockpit) -> Element<'_, Message> {
    let content = widget::Column::new().spacing(10);
    let content = match &app.selected_profile {
        None => content,
        Some(profile_id) => {
            let profile = app.profiles.profiles.get(&profile_id).expect("Selected profile should exist");
            content.push(keybindings_list(profile_id, &profile.keybindings, app))
        }
    };
    widget::container(content).into()
}

fn keybindings_list<'a>(
    profile_id: &'a ProfileId,
    bindings: &'a[Keybinding],
    app: &'a SpaceNavCockpit
) -> Element<'a, Message> {
    bindings.iter()
        .enumerate()
        .fold(widget::Column::new(), |column, binding| {
            column.push(keybindings_list_item(profile_id, binding, app))
        })
        .spacing(10)
        .into()
}

fn keybindings_list_item<'a>(
    profile_id: &'a ProfileId,
    keybinding: (usize, &'a Keybinding),
    app: &'a SpaceNavCockpit
) -> Element<'a, Message>
{
    let (header, view) = match keybinding {
        (keybinding, Keybinding::SelectProfile { profile, button }) =>
            keybinding_select_profile(profile_id, keybinding, profile, button, app),
        (keybinding, Keybinding::PreviousProfile { button }) =>
            keybinding_previous_profile(profile_id, keybinding, button),
        (keybinding, Keybinding::NextProfile { button }) =>
            keybinding_next_profile(profile_id, keybinding, button),
    };
    let header = widget::container::Container::new(header)
        .padding([4, 10])
        .width(Fill)
        .style(|theme: &Theme| widget::container::Style {
            background: Some(theme.extended_palette().background.weak.color.into()),
            border: border::rounded(BORDER_RADIUS),
            ..Default::default()
        });
    let content = widget::Column::new()
        .spacing(16)
        .push(header)
        .push(view);
    widget::Container::new(content)
        .width(Fill)
        .padding(10)
        .style(|theme| widget::container::Style {
            background: Some(theme.extended_palette().background.strong.color.into()),
            border: border::rounded(BORDER_RADIUS),
            ..Default::default()
        })
        .into()
}

fn keybinding_select_profile<'a>(
    profile_id: &'a ProfileId,
    keybinding: usize,
    selected_profile: &'a Option<ProfileId>,
    selected_button: &'a Option<KeybindingButton>,
    app: &'a SpaceNavCockpit
) -> (Element<'a, Message>, Element<'a, Message>)
{
    let profiles = app.profiles.profiles.iter()
        .filter(|(id, _)| profile_id != *id)
        .map(ProfilePickerItem)
        .collect::<Vec<_>>();
    let selected_profile = selected_profile.as_ref()
        .and_then(|id| app.profiles.profiles.get(id).map(|profile| (id, profile)))
        .map(ProfilePickerItem);

    let view = widget::Column::new()
        .spacing(10)
        .push(widget::Row::new()
            .spacing(10)
            .align_y(Vertical::Center)
            .push(widget::Text::new("Select:")
                .width(100)
                .align_x(Alignment::End))
            .push(widget::PickList::new(profiles, selected_profile, move |item| Message::KeybindingSelectProfileChanged { profile_id: Clone::clone(&profile_id), keybinding, select: Some(Clone::clone(&item.0.0)) })
                .width(Fill))
        )
        .push(keybinding_button_row(profile_id, keybinding, selected_button));
    let header = widget::Text::new("Select Profile");
    (header.into(), view.into())
}

fn keybinding_previous_profile<'a>(profile_id: &'a ProfileId, keybinding: usize, button: &'a Option<KeybindingButton>) -> (Element<'a, Message>, Element<'a, Message>) {
    let header = widget::Text::new("Select Previous Profile");
    let view = keybinding_button_row(profile_id, keybinding, button);
    (header.into(), view.into())
}

fn keybinding_next_profile<'a>(profile_id: &'a ProfileId, keybinding: usize, button: &'a Option<KeybindingButton>) -> (Element<'a, Message>, Element<'a, Message>) {
    let header = widget::Text::new("Select Next Profile");
    let view = keybinding_button_row(profile_id, keybinding, button);
    (header.into(), view.into())
}

fn keybinding_button_row<'a>(profile_id: &'a ProfileId, keybinding: usize, button: &'a Option<KeybindingButton>) -> Element<'a, Message> {
    let buttons = vec![0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    widget::Row::new()
        .spacing(10)
        .align_y(Vertical::Center)
        .push(widget::Text::new("Button:")
            .width(100)
            .align_x(Alignment::End))
        .push(widget::PickList::new(buttons, button.as_ref().map(|button| button.number), move |button| Message::KeybindingButtonChanged { profile_id: Clone::clone(&profile_id.clone()), keybinding, button: Some(button) }))
        .into()
}

#[derive(Clone, Debug)]
struct ProfilePickerItem<'a>((&'a ProfileId, &'a Profile));

impl <'a> PartialEq for ProfilePickerItem<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.0 == other.0.0
    }
}

impl <'a> std::fmt::Display for ProfilePickerItem<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0.1.variant.as_ref() {
            None => write!(f, "{}", self.0.1.name),
            Some(variant) => write!(f, "{} ({})", self.0.1.name, variant),
        }
    }
}
