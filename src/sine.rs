#![allow(dead_code)]

use crate::gui::lslider::LSlider;
use iced::{
    widget::{Column, Rule},
    Element,
};

#[derive(Debug, Clone, Copy)]
pub enum SineMessage {
    XFreq(f32),
    YFreq(f32),
    XExp(f32),
    YExp(f32),
    Draw,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SineControls {
    pub xfreq: f32,
    pub yfreq: f32,
    pub xexp: f32,
    pub yexp: f32,
}

impl Default for SineControls {
    fn default() -> Self {
        Self {
            xfreq: 1.0,
            yfreq: 1.0,
            xexp: 2.0,
            yexp: 2.0,
        }
    }
}

impl<'a> SineControls {
    pub fn new(xfreq: f32, yfreq: f32, xexp: f32, yexp: f32) -> Self {
        Self {
            xfreq,
            yfreq,
            xexp,
            yexp,
        }
    }

    pub fn set_xfreq(mut self, xfreq: f32) -> Self {
        self.xfreq = xfreq;
        self
    }

    pub fn set_yfreq(mut self, yfreq: f32) -> Self {
        self.yfreq = yfreq;
        self
    }

    pub fn set_xexp(mut self, xexp: f32) -> Self {
        self.xexp = xexp;
        self
    }

    pub fn set_yexp(mut self, yexp: f32) -> Self {
        self.yexp = yexp;
        self
    }

    pub fn update(&mut self, message: SineMessage) {
        use SineMessage::*;
        match message {
            XFreq(xfreq) => self.xfreq = xfreq,
            YFreq(yfreq) => self.yfreq = yfreq,
            XExp(xexp) => self.xexp = xexp,
            YExp(yexp) => self.yexp = yexp,
            Draw => (),
        }
    }

    pub fn view(&self) -> Element<'a, SineMessage> {
        use SineMessage::*;
        Column::new()
            .push(Rule::horizontal(10))
            .push("Sine Noise")
            .push(
                LSlider::new(
                    "X Frequency".to_string(),
                    self.xfreq,
                    0.1..=10.0,
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
                    0.1..=10.0,
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
            .into()
    }
}
