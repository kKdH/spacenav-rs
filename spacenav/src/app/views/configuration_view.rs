use crate::app::app::Message;
use crate::app::SpaceNavCockpit;
use iced::widget::{container, image, slider, text};
use iced::{widget, Element};
use iced::alignment::Vertical;
use iced_aw::{TabBar, TabLabel};

pub fn configuration_view(app: &SpaceNavCockpit) -> Element<'_, Message> {
    let tab_bar = {
        let mut tab_bar = app.profiles.profiles.iter()
            .map(|(id, profile)| (id.to_owned(), TabLabel::Text(Clone::clone(&profile.title))))
            .fold(TabBar::new(Message::TabSelected), |tab_bar, (key, tab)| {
                tab_bar.push(key, tab)
            });
        if let Some(profile_id) = &app.selected_profile {
            tab_bar = tab_bar.set_active_tab(profile_id)
        };
        tab_bar
    };

    let content = {
        let content = widget::Column::new().spacing(10).padding(10).push(tab_bar);
        match &app.selected_profile {
            None => content,
            Some(profile_id) => {
                let profile_id = Clone::clone(profile_id);
                let profile = app.profiles.profiles.get(&profile_id).expect("Selected profile should exist");
                let header_row = widget::Row::new()
                    .spacing(10)
                    .push(text("Profile:"))
                    .push(text(Clone::clone(&profile.title)));
                let axes_rows = {
                    let row = widget::Row::new();
                    let column =
                        profile.navigation.iter()
                            .zip(app.image_handles.all_axes())
                            .fold(widget::Column::new(), |column, ((navigation_name, navigation_settings), axis_image)| {
                                let profile_id = Clone::clone(&profile_id);
                                column.push(widget::Row::new()
                                    .push(image(axis_image).height(72))
                                    .push(text(format!("{:?}", navigation_name)))
                                    .push(text(format!("Speed: {:.2}", navigation_settings.speed)))
                                    .push(slider(0_f32..=2_f32, navigation_settings.speed, move |value| Message::AxisSpeedChanged { profile: Clone::clone(&profile_id), axis: Clone::clone(&navigation_name), speed: value }).step(0.01))
                                    .push(text(format!("Deadzone: {}", navigation_settings.deadzone)))
                                    .spacing(10)
                                    .align_y(Vertical::Center)
                                )
                            });
                    row.push(column)
                };
                let bot = widget::Row::new();
                content.push(header_row).push(axes_rows).push(bot)
            }
        }
    };

    container(content).into()
}
