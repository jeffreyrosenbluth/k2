use crate::gui::lslider::LSlider;
use crate::Message::{self, *};
use crate::RandomMessage::*;
use iced::widget::Column;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fractal {
    pub octaves: u8,
    pub persistence: f32,
    pub lacunarity: f32,
    pub frequency: f32,
}

impl<'a> Fractal {
    pub fn new(octaves: u8, persistence: f32, lacunarity: f32, frequency: f32) -> Self {
        Self {
            octaves,
            persistence,
            lacunarity,
            frequency,
        }
    }
    pub fn show(&self) -> Column<'a, Message> {
        let mut col = Column::new()
            .push(
                LSlider::new(
                    "Octaves".to_string(),
                    self.octaves,
                    1..=8,
                    1,
                    Octaves,
                    Some(Rand(RandomOctaves)),
                    Draw,
                )
                .decimals(0),
            )
            .spacing(15);
        if self.octaves > 1 {
            col = col
                .push(
                    LSlider::new(
                        "Persistence".to_string(),
                        self.persistence,
                        0.05..=0.95,
                        0.05,
                        Persistence,
                        None,
                        Draw,
                    )
                    .decimals(2),
                )
                .push(
                    LSlider::new(
                        "Lacunarity".to_string(),
                        self.lacunarity,
                        0.1..=4.00,
                        0.1,
                        Lacunarity,
                        None,
                        Draw,
                    )
                    .decimals(2),
                )
                .push(
                    LSlider::new(
                        "Frequency".to_string(),
                        self.frequency,
                        0.1..=4.00,
                        0.1,
                        Frequency,
                        None,
                        Draw,
                    )
                    .decimals(2),
                )
        }
        col
    }
}
