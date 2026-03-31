use crate::app::app::Message;
use crate::app::views::BORDER_RADIUS;
use crate::app::widgets::axis_bar;
use crate::app::SpaceNavCockpit;
use iced::alignment::Vertical;
use iced::widget::image;
use iced::widget::text::Wrapping;
use iced::{border, widget, Alignment, Element, Fill, Padding};
use spacenav_settings::{MotionFunctionName, MotionFunctionSettings, ProfileId};
use std::ops::RangeInclusive;
use iced::widget::text_input::Catalog;

pub fn profile_motion_configuration_view(app: &SpaceNavCockpit) -> Element<'_, Message> {

    let motion_function_images = app.image_handles.all_axes();

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
                        profile.motions.iter()
                            .map(|(function_name, function_settings)| (function_name, function_settings, Clone::clone(&motion_function_images[function_settings.axis as usize])))
                            .fold(column, |column, (function_name, function_settings, function_image)| {
                                let motion_settings_row = motion_settings_row(
                                    Clone::clone(&profile_id),
                                    *function_name,
                                    function_image,
                                    function_settings,
                                    app.axes_values[function_settings.axis as usize]
                                );
                                column.push(motion_settings_row)
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

fn motion_settings_row(
    profile_id: ProfileId,
    function_name: MotionFunctionName,
    function_image: image::Handle,
    function_settings: &MotionFunctionSettings,
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
                        .push(widget::Text::new(motion_display_name(function_name))
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
        .push(motion_settings_speed_row(Clone::clone(&profile_id), function_name, function_settings.speed))
        .push(motion_settings_threshold_row(Clone::clone(&profile_id), function_name, function_settings.threshold))
        .push(motion_settings_inverted_and_disabled_row(profile_id, function_name, function_settings.inverted, function_settings.disabled));

    widget::Container::new(
        widget::Row::new()
            .push(axis_information_column)
            .push(speed_and_threshold_column)
            .padding(Padding::from([5, 10]))
            .align_y(Vertical::Center))
        .style(|theme| widget::container::Style {
            background: Some(theme.extended_palette().background.strong.color.into()),
            border: border::rounded(BORDER_RADIUS),
            ..Default::default()
        })
        .into()
}

fn motion_settings_speed_row(profile: ProfileId, axis: MotionFunctionName, speed: f32) -> Element<'static, Message> {
    let on_release = {
        let profile = Clone::clone(&profile);
        move || Message::UpdateAxisSpeed { profile_id: Clone::clone(&profile), function_name: axis }
    };
    widget::Row::new()
        .spacing(10)
        .align_y(Vertical::Center)
        .push(widget::text("Speed:")
            .width(100)
            .align_x(Alignment::End)
        )
        .push(slider(
            0_f32..=5_f32,
            speed,
            0.01,
            move |speed| Message::AxisSpeedChanged { profile_id: Clone::clone(&profile), function_name: axis, speed },
            on_release
        ))
        .push(input(speed.to_string()))
        .into()
}

fn motion_settings_threshold_row(profile_id: ProfileId, axis: MotionFunctionName, threshold: u8) -> Element<'static, Message> {
    let on_release = {
        let profile_id = Clone::clone(&profile_id);
        move || Message::UpdateAxisThreshold { profile_id: Clone::clone(&profile_id), function_name: axis }
    };
    widget::Row::new()
        .spacing(10)
        .align_y(Vertical::Center)
        .push(widget::text("Threshold:")
            .width(100)
            .align_x(Alignment::End)
        )
        .push(slider(
            u8::MIN..=u8::MAX,
            threshold,
            1_u8,
            move |threshold| Message::AxisThresholdChanged { profile_id: Clone::clone(&profile_id), function_name: axis, threshold },
            on_release
        ))
        .push(input(threshold.to_string()))
        .into()
}

fn motion_settings_inverted_and_disabled_row(profile_id: ProfileId, function_name: MotionFunctionName, inverted: bool, disabled: bool) -> Element<'static, Message> {

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

fn slider<'a, T>(
    range: RangeInclusive<T>,
    value: T,
    step: T,
    on_change: impl Fn(T) -> Message + 'static,
    on_release: impl Fn() -> Message + 'static
) -> Element<'static, Message>
where
    T: 'a + Copy + From<u8> + PartialOrd + Into<f64> + num_traits::FromPrimitive + 'static,
{
    widget::Slider::new(range, value, on_change)
        .on_release(on_release())
        .step(step)
        .style(|theme, status| {
            let mut style = widget::slider::default(theme, status);
            style.rail.backgrounds = (theme.palette().background.into(), theme.palette().background.into());
            style
        })
        .width(iced::Fill)
        .into()
}

fn input(
    value: String,
) -> Element<'static, Message>
{
    widget::TextInput::new(&value, &value)
        .align_x(Alignment::Center)
        .style(|theme: &iced::Theme, status| widget::text_input::Style {
            border: iced::Border {
                radius: 20.0.into(),
                width: 1.0,
                color: theme.extended_palette().background.strong.color,
            },
            ..widget::text_input::default(theme, status)
        })
        .width(54)
        .into()
}


fn motion_display_name(name: MotionFunctionName) -> String {
    match name {
        MotionFunctionName::LeftRight => String::from("Left / Right"),
        MotionFunctionName::UpDown => String::from("Up / Down"),
        MotionFunctionName::FwdBwd => String::from("Forward / Backward"),
        MotionFunctionName::Pitch => String::from("Pitch"),
        MotionFunctionName::Yaw => String::from("Yaw"),
        MotionFunctionName::Roll => String::from("Roll"),
    }
}
