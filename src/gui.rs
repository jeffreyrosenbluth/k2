use std::ops::RangeInclusive;

use iced::{
    widget::{column, row, slider, text, Column},
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
        format!("{:5.1}", value)
    };
    column![
        text(title).size(15),
        row![
            slider(range, value, message)
                .on_release(release)
                .step(step)
                .width(Length::Units(130)),
            text(n).size(15).style(Color::from_rgb8(0x5E, 0x7C, 0xE2))
        ]
        .align_items(Alignment::Center)
    ]
    .spacing(5)
}