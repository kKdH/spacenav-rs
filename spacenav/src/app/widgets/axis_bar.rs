use std::ops::RangeInclusive;
use iced::{Color, Point, Rectangle, Renderer, Size, Theme};
use iced::border::Radius;
use iced::mouse::Cursor;
use iced::widget::canvas;
use iced::widget::canvas::Geometry;

pub fn axis_bar(range: RangeInclusive<f32>, value: f32) -> AxisBar {
    AxisBar::new(range, value)
}

pub struct AxisBar {
    range: RangeInclusive<f32>,
    value: f32,
}

impl AxisBar {

    pub fn new(range: RangeInclusive<f32>, value: f32) -> Self {
        Self {
            range,
            value,
        }
    }
}

impl<Message> canvas::Program<Message> for AxisBar {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor
    ) -> Vec<Geometry<Renderer>> {

        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let bar = {
            let (range_start, range_end) = Clone::clone(&self.range).into_inner();
            let scaled_value = (-self.value - range_start) / (range_end - range_start) * bounds.width;
            let (x, w) = if self.value > 0_f32 {
                let x = bounds.width / 2_f32;
                let w = x - scaled_value;
                (x, w)
            }
            else {
                let x = bounds.width - scaled_value;
                let w = bounds.width / 2_f32 - x;
                (x, w)
            };
            let y = 6_f32;
            let h = bounds.height - 12_f32;

            canvas::Path::rounded_rectangle(Point::new(x, y), Size::new(w, h), Radius::new(2_f32))
        };

        let bar_box = canvas::Path::rounded_rectangle(Point::new(0_f32, 4_f32), Size::new(bounds.width, bounds.height - 8_f32), Radius::new(2_f32));

        let mid_line = {
            let line_width = 4_f32;
            let x = bounds.width / 2_f32 - line_width / 2_f32;
            let y = 0_f32;
            canvas::Path::rectangle(Point::new(x, y), Size::new(line_width, bounds.size().height))
        };

        frame.fill(&bar_box, theme.extended_palette().background.strong.color);
        frame.fill(&bar, theme.extended_palette().primary.base.color);
        frame.fill(&mid_line, theme.palette().text);

        vec![frame.into_geometry()]
    }

}
