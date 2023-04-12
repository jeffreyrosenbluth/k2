use crate::gui::lslider::LSlider;
use crate::Message::{self, *};
use iced::widget::{Column, Rule};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sine {
    pub xfreq: f32,
    pub yfreq: f32,
    pub xexp: f32,
    pub yexp: f32,
}

impl<'a> Sine {
    pub fn new(xfreq: f32, yfreq: f32, xexp: f32, yexp: f32) -> Self {
        Self {
            xfreq,
            yfreq,
            xexp,
            yexp,
        }
    }
    pub fn show(&self) -> Column<'a, Message> {
        Column::new()
            .push(Rule::horizontal(10))
            .push("Sine Noise")
            .push(
                LSlider::new(
                    "X Frequency".to_string(),
                    self.xfreq,
                    0.1..=4.0,
                    0.1,
                    XFreq,
                    Draw,
                )
                .decimals(1),
            )
            .push(
                LSlider::new(
                    "Y Frequency".to_string(),
                    self.yfreq,
                    0.1..=4.0,
                    0.1,
                    YFreq,
                    Draw,
                )
                .decimals(1),
            )
            .push(
                LSlider::new(
                    "X Exponent".to_string(),
                    self.xexp,
                    1.0..=4.0,
                    1.0,
                    XExp,
                    Draw,
                )
                .decimals(0),
            )
            .push(
                LSlider::new(
                    "Y Exponent".to_string(),
                    self.yexp,
                    1.0..=4.0,
                    1.0,
                    YExp,
                    Draw,
                )
                .decimals(0),
            )
            .spacing(15)
    }
}
