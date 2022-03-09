use crate::color;
use iced::canvas::path::Arc;
use iced::canvas::{Cache, Cursor, Geometry, Path, Program, Stroke};
use iced::{Color, Rectangle};
use std::f32::consts;

const START_ANGLE: f32 = consts::PI / 2.0;

pub struct ProgressCircle {
    progress_bar: Cache,
    stroke_width: f32,
    pub value: f32,
}

impl ProgressCircle {
    pub fn new(stroke_width: f32) -> Self {
        Self {
            progress_bar: Default::default(),
            stroke_width,
            value: Default::default(),
        }
    }

    pub fn update(&mut self, value: f32) {
        self.value = value;
        self.progress_bar.clear();
    }
}

impl<Message> Program<Message> for ProgressCircle {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let progress_bar = self.progress_bar.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.height().min(frame.height()) / 2.0 - self.stroke_width;

            let path = Path::new(|builder| {
                builder.arc(Arc {
                    center,
                    radius,
                    start_angle: -START_ANGLE,
                    end_angle: 2.0 * self.value * consts::PI - START_ANGLE,
                })
            });

            frame.stroke(
                &path,
                Stroke {
                    color: color!(0x01B757),
                    width: self.stroke_width,
                    line_cap: Default::default(),
                    line_join: Default::default(),
                    line_dash: Default::default(),
                },
            );
        });

        vec![progress_bar]
    }
}
