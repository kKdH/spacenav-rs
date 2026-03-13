use crate::app::app::Message;
use crate::app::widgets::axis_bar;
use crate::app::SpaceNavCockpit;
use iced::alignment::Vertical;
use iced::widget::text::Wrapping;
use iced::widget::image;
use iced::{widget, Alignment, Element, Fill, Padding};
use spacenav_settings::{NavigationFunctionName, NavigationFunctionSettings, ProfileId};

pub fn navigation_configuration_view(app: &SpaceNavCockpit) -> Element<'_, Message> {

    let navigation_function_images = app.image_handles.all_axes();

    let content = {
        let content = widget::Column::new().spacing(10);
        match &app.selected_profile {
            None => content,
            Some(profile_id) => {
                let profile_id = Clone::clone(profile_id);
                let profile = app.profiles.profiles.get(&profile_id).expect("Selected profile should exist");
                let config_row = {
                    let row = widget::Row::new();
                    let column = {
                        let column = widget::Column::new()
                            .spacing(10);
                        profile.navigation.iter()
                            .map(|(function_name, function_settings)| (function_name, function_settings, Clone::clone(&navigation_function_images[function_settings.axis as usize])))
                            .fold(column, |column, (function_name, function_settings, function_image)| {
                                let navigation_settings_row = navigation_settings_row(
                                    Clone::clone(&profile_id),
                                    *function_name,
                                    function_image,
                                    function_settings,
                                    app.axes_values[function_settings.axis as usize]
                                );
                                column.push(navigation_settings_row)
                            })
                    };
                    row.push(column)
                };
                content.push(config_row)
            }
        }
    };

    widget::container(content).into()
}

fn navigation_settings_row(
    profile_id: ProfileId,
    function_name: NavigationFunctionName,
    function_image: image::Handle,
    function_settings: &NavigationFunctionSettings,
    axis_value: f32
) -> Element<'static, Message> {

    let axis_information_column = {
        let profile_id = Clone::clone(&profile_id);
        widget::Column::new()
            .width(250)
            .spacing(5)
            .push(
                widget::Row::new()
                    .push(image(function_image).height(64))
                    .push(widget::Column::new()
                        .spacing(10)
                        .push(widget::Text::new(navigation_display_name(function_name))
                            .font(iced::Font {
                                weight: iced::font::Weight::Semibold,
                                ..Default::default()
                            })
                            .wrapping(Wrapping::None)
                        )
                        .push(widget::Row::new()
                            .spacing(5)
                            .align_y(Vertical::Center)
                            .push(widget::text("Axis:"))
                            .push(widget::PickList::new(vec![0, 1, 2, 3, 4, 5], Some(function_settings.axis), move |axis| Message::AxisMappingChanged { profile_id: Clone::clone(&profile_id), function_name, axis })
                                .width(Fill))
                        )
                    )
                    .spacing(10)
                    .align_y(Vertical::Center)
            )
            .push(widget::Row::new()
                .push(widget::canvas(axis_bar(-500_f32..=500_f32, axis_value)).width(Fill).height(30))
            )
    };

    let speed_and_threshold_column = widget::Column::new()
        .spacing(10)
        .push(navigation_settings_speed_row(Clone::clone(&profile_id), function_name, function_settings.speed))
        .push(navigation_settings_threshold_row(Clone::clone(&profile_id), function_name, function_settings.threshold))
        .push(navigation_settings_inverted_and_disabled_row(profile_id, function_name, function_settings.inverted, function_settings.disabled));

    widget::Container::new(
        widget::Row::new()
            .push(axis_information_column)
            .push(speed_and_threshold_column)
            .padding(Padding::from([5, 10]))
            .align_y(Vertical::Center)
    )
        .style(|theme| widget::container::rounded_box(theme))
        .into()
}

fn navigation_settings_speed_row(profile: ProfileId, axis: NavigationFunctionName, speed: f32) -> Element<'static, Message> {
    let update_message = Message::UpdateAxisSpeed { profile_id: Clone::clone(&profile), function_name: axis };
    widget::Row::new()
        .spacing(10)
        .align_y(Vertical::Center)
        .push(widget::text("Speed:")
            .width(100)
            .align_x(Alignment::End)
        )
        .push(widget::Slider::new(0_f32..=2_f32, speed, move |speed| Message::AxisSpeedChanged { profile_id: Clone::clone(&profile), function_name: axis, speed })
            .on_release(update_message)
            .step(0.01))
        .into()
}

fn navigation_settings_threshold_row(profile_id: ProfileId, axis: NavigationFunctionName, threshold: u8) -> Element<'static, Message> {
    let update_message = Message::UpdateAxisThreshold { profile_id: Clone::clone(&profile_id), function_name: axis };
    widget::Row::new()
        .spacing(10)
        .align_y(Vertical::Center)
        .push(widget::text("Threshold:")
            .width(100)
            .align_x(Alignment::End)
        )
        .push(widget::Slider::new(u8::MIN..=u8::MAX, threshold, move |threshold| Message::AxisThresholdChanged { profile_id: Clone::clone(&profile_id), function_name: axis, threshold })
            .on_release(update_message)
            .step(1))
        .into()
}

fn navigation_settings_inverted_and_disabled_row(profile_id: ProfileId, function_name: NavigationFunctionName, inverted: bool, disabled: bool) -> Element<'static, Message> {

    let on_toggle_inverted = {
        let profile_id = Clone::clone(&profile_id);
        move |inverted| Message::AxisInvertedChanged { profile_id: Clone::clone(&profile_id), function_name, inverted }
    };

    let on_toggle_disabled = {
        let profile_id = Clone::clone(&profile_id);
        move |disabled| Message::AxisDisabledChanged { profile_id: Clone::clone(&profile_id), function_name, disabled }
    };

    widget::Row::new()
        .spacing(10)
        .align_y(Vertical::Center)
        .push(widget::text("Inverted:")
            .width(100)
            .align_x(Alignment::End)
        )
        .push(widget::Checkbox::new(inverted)
            .on_toggle(on_toggle_inverted))
        .push(widget::text("Disabled:")
            .width(100)
            .align_x(Alignment::End)
        )
        .push(widget::Checkbox::new(disabled)
            .on_toggle(on_toggle_disabled))
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
