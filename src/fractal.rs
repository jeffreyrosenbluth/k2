#![allow(dead_code)]

use crate::common::PresetState::NotSet;
use crate::gui::lslider::LSlider;
use crate::Message::{self, *};
use iced::widget::{Column, Rule};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FractalControls {
    pub octaves: u8,
    pub persistence: f32,
    pub lacunarity: f32,
    pub frequency: f32,
}

impl Default for FractalControls {
    fn default() -> Self {
        Self {
            octaves: 4,
            persistence: 0.5,
            lacunarity: 2.094395,
            frequency: 1.0,
        }
    }
}

impl FractalControls {
    pub fn new(octaves: u8, persistence: f32, lacunarity: f32, frequency: f32) -> Self {
        Self {
            octaves,
            persistence,
            lacunarity,
            frequency,
        }
    }

    pub fn set_octaves(mut self, octaves: u8) -> Self {
        self.octaves = octaves;
        self
    }

    pub fn set_persistence(mut self, persistence: f32) -> Self {
        self.persistence = persistence;
        self
    }

    pub fn set_lacunarity(mut self, lacunarity: f32) -> Self {
        self.lacunarity = lacunarity;
        self
    }

    pub fn set_frequency(mut self, frequency: f32) -> Self {
        self.frequency = frequency;
        self
    }
}

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
            .push(Rule::horizontal(10))
            .push("Fractal Noise")
            .push(
                LSlider::new(
                    "Octaves".to_string(),
                    self.octaves,
                    1..=8,
                    1,
                    Octaves,
                    Draw(NotSet),
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
                        Draw(NotSet),
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
                        Draw(NotSet),
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
                        Draw(NotSet),
                    )
                    .decimals(2),
                )
        }
        col
    }
}
