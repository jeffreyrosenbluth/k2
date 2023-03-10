#![allow(dead_code)]
use std::ops::RangeInclusive;

use iced::{
    widget::{pick_list, row, slider, text, Column},
    Alignment, Color, Length,
};

pub fn wslider<'a, M, T, F>(
    title: String,
    message: M,
    release: F,
    range: RangeInclusive<T>,
    value: T,
    step: T,
) -> Column<'a, F>
where
    T: 'a
        + num_traits::cast::FromPrimitive
        + Copy
        + From<u8>
        + PartialOrd<T>
        + std::fmt::Display
        + num_traits::One,
    F: 'a + Clone,
    M: 'a + Fn(T) -> F + Clone,
    f64: From<T>,
{
    let n = if step >= T::one() {
        format!("{:5.0}", value)
    } else {
        format!("{:5.2}", value)
    };
    iced::widget::column![
        text(title).size(15),
        row![
            slider(range, value, message)
                .on_release(release)
                .step(step)
                .width(Length::Fixed(150.0)),
            text(n).size(15).style(Color::from_rgb8(0x5E, 0x7C, 0xE2))
        ]
        .align_items(Alignment::Center)
    ]
    .spacing(5)
}

pub struct SliderBuilder<T, M, F>
where
    F: Clone,
    M: Fn(T) -> F + Clone,
{
    label: String,
    value: T,
    message: M,
    release: F,
    range: RangeInclusive<T>,
    step: T,
    text_size: u16,
    width: u16,
    spacing: u16,
    decimals: u8,
}

impl<'a, T, M, F> SliderBuilder<T, M, F>
where
    T: 'a
        + Copy
        + From<u8>
        + std::cmp::PartialOrd
        + num_traits::One
        + num_traits::Zero
        + std::fmt::Display
        + num_traits::FromPrimitive,
    F: 'a + Clone,
    M: 'a + Fn(T) -> F + Clone,
    f64: From<T>,
{
    pub fn new(label: String, message: M, release: F, value: T) -> Self {
        Self {
            label,
            value,
            message,
            release,
            range: T::zero()..=T::one(),
            step: T::one(),
            text_size: 15,
            width: 150,
            spacing: 5,
            decimals: 1,
        }
    }

    pub fn step(self, step: T) -> Self {
        SliderBuilder { step, ..self }
    }

    pub fn range(self, range: RangeInclusive<T>) -> Self {
        SliderBuilder { range, ..self }
    }

    pub fn text_size(self, size: u16) -> Self {
        SliderBuilder {
            text_size: size,
            ..self
        }
    }

    pub fn width(self, width: u16) -> Self {
        SliderBuilder { width, ..self }
    }

    pub fn spacing(self, spacing: u16) -> Self {
        SliderBuilder { spacing, ..self }
    }

    pub fn decimals(self, decimals: u8) -> Self {
        SliderBuilder { decimals, ..self }
    }

    pub fn build(self) -> Column<'a, F> {
        let n = match self.decimals {
            0 => format!("{:7.0}", self.value),
            1 => format!("{:7.1}", self.value),
            _ => format!("{:7.2}", self.value),
        };
        iced::widget::column![
            text(self.label).size(self.text_size),
            row![
                slider(self.range, self.value, self.message)
                    .on_release(self.release)
                    .step(self.step)
                    .width(Length::Fixed(self.width as f32)),
                text(n)
                    .size(self.text_size)
                    .style(Color::from_rgb8(0x5E, 0x7C, 0xE2))
            ]
            .align_items(Alignment::Center)
        ]
        .spacing(self.spacing)
    }
}

pub struct PickListBuilder<T, M, F>
where
    T: 'static + Copy + std::fmt::Display + Clone + Eq,
    F: Fn(T) -> M,
{
    label: String,
    choices: Vec<T>,
    value: Option<T>,
    message: F,
    text_size: u16,
    width: u16,
    spacing: u16,
}

impl<T, M, F> PickListBuilder<T, M, F>
where
    T: 'static + Copy + std::fmt::Display + Clone + Eq,
    M: 'static,
    F: Fn(T) -> M + 'static,
{
    pub fn new(label: String, choices: Vec<T>, value: Option<T>, message: F) -> Self {
        Self {
            label,
            choices,
            value,
            message,
            text_size: 15,
            width: 175,
            spacing: 5,
        }
    }

    pub fn build(self) -> Column<'static, M> {
        iced::widget::column![
            text(self.label).size(self.text_size),
            pick_list(self.choices, self.value, self.message)
                .text_size(self.text_size)
                .width(Length::Fixed(self.width as f32)),
        ]
        .spacing(self.spacing)
    }

    pub fn text_size(self, size: u16) -> Self {
        PickListBuilder {
            text_size: size,
            ..self
        }
    }

    pub fn width(self, width: u16) -> Self {
        PickListBuilder { width, ..self }
    }

    pub fn spacing(self, spacing: u16) -> Self {
        PickListBuilder { spacing, ..self }
    }
}
