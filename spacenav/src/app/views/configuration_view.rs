use crate::app::app::Message;
use crate::app::widgets::axis_bar;
use crate::app::SpaceNavCockpit;
use iced::alignment::Vertical;
use iced::widget::text::Wrapping;
use iced::widget::{container, image, text};
use iced::{widget, Alignment, Element, Fill};
use iced_aw::{TabBar, TabLabel};
use spacenav_settings::NavigationFunctionName;

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
                let config_row = {
                    let row = widget::Row::new();
                    let column =
                        profile.navigation.iter()
                            .zip(app.image_handles.all_axes())
                            .fold(widget::Column::new(), |column, ((navigation_name, navigation_settings), axis_image)| {
                                let profile_id = Clone::clone(&profile_id);
                                let axis_information_column = widget::Column::new()
                                    .width(220)
                                    .spacing(10)
                                    .push(
                                        widget::Row::new()
                                            .push(image(axis_image).height(64))
                                            .push(text(navigation_display_name(*navigation_name)).wrapping(Wrapping::None))
                                            .spacing(10)
                                            .align_y(Vertical::Center)
                                    )
                                    .push(widget::Row::new()
                                        .push(widget::canvas(axis_bar(-500_f32..=500_f32, app.axes_values[navigation_settings.axis])).width(Fill).height(30))
                                    );
                                let speed_and_threshold_column = widget::Column::new()
                                    .spacing(10)
                                    .push(speed_row(Clone::clone(&profile_id), *navigation_name, navigation_settings.speed))
                                    .push(threshold_row(Clone::clone(&profile_id), *navigation_name, navigation_settings.threshold))
                                    .push(invert_row(profile_id, *navigation_name, navigation_settings.invert));
                                column.push(widget::Row::new()
                                    .push(axis_information_column)
                                    .push(speed_and_threshold_column)
                                    .align_y(Vertical::Center)
                                )
                            });
                    row.push(column)
                };
                content.push(config_row)
            }
        }
    };

    container(content).into()
}

fn speed_row(profile: String, axis: NavigationFunctionName, speed: f32) -> Element<'static, Message> {
    let update_message = Message::UpdateAxisSpeed { profile: Clone::clone(&profile), axis };
    widget::Row::new()
        .spacing(10)
        .align_y(Vertical::Center)
        .push(text("Speed:")
            .width(100)
            .align_x(Alignment::End)
        )
        .push(widget::Slider::new(0_f32..=2_f32, speed, move |speed| Message::AxisSpeedChanged { profile: Clone::clone(&profile), axis, speed })
            .on_release(update_message)
            .step(0.01))
        .into()
}

fn threshold_row(profile: String, axis: NavigationFunctionName, threshold: i32) -> Element<'static, Message> {
    let update_message = Message::UpdateAxisThreshold { profile: Clone::clone(&profile), axis };
    widget::Row::new()
        .spacing(10)
        .align_y(Vertical::Center)
        .push(text("Threshold:")
            .width(100)
            .align_x(Alignment::End)
        )
        .push(widget::Slider::new(0_i32..=255_i32, threshold, move |threshold| Message::AxisThresholdChanged { profile: Clone::clone(&profile), axis, threshold })
            .on_release(update_message)
            .step(1))
        .into()
}

fn invert_row(profile: String, axis: NavigationFunctionName, inverted: bool) -> Element<'static, Message> {
    widget::Row::new()
        .spacing(10)
        .align_y(Vertical::Center)
        .push(text("Inverted:")
            .width(100)
            .align_x(Alignment::End)
        )
        .push(widget::Checkbox::new(inverted)
            .on_toggle(move |inverted| Message::AxisInvertedChanged { profile: Clone::clone(&profile), axis, inverted }))
        .into()
}

fn navigation_display_name(navigation_name: NavigationFunctionName) -> String {
    match navigation_name {
        NavigationFunctionName::LeftRight => String::from("Left / Right"),
        NavigationFunctionName::UpDown => String::from("Up / Down"),
        NavigationFunctionName::FwdBwd => String::from("Forward / Backward"),
        NavigationFunctionName::Pitch => String::from("Pitch"),
        NavigationFunctionName::Yaw => String::from("Yaw"),
        NavigationFunctionName::Roll => String::from("Roll"),
    }
}
