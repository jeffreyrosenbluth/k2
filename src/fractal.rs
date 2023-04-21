#![allow(dead_code)]

use crate::gui::lslider::LSlider;
use iced::widget::{Column, Rule};
use iced::Element;
#[derive(Debug, Clone)]
pub enum FractalMessage {
    Octaves(u8),
    Persistence(f32),
    Lacunarity(f32),
    Frequency(f32),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FractalControls {
    pub octaves: u8,
    pub persistence: f32,
    pub lacunarity: f32,
    pub frequency: f32,
    pub dirty: bool,
}

impl Default for FractalControls {
    fn default() -> Self {
        Self {
            octaves: 4,
            persistence: 0.5,
            lacunarity: 2.094395,
            frequency: 1.0,
            dirty: false,
        }
    }
}

impl<'a> FractalControls {
    pub fn new(
        octaves: u8,
        persistence: f32,
        lacunarity: f32,
        frequency: f32,
        dirty: bool,
    ) -> Self {
        Self {
            octaves,
            persistence,
            lacunarity,
            frequency,
            dirty,
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

    pub fn update(&mut self, message: FractalMessage) {
        use FractalMessage::*;
        match message {
            Octaves(octaves) => {
                self.octaves = octaves;
                self.dirty = false;
            }
            Persistence(persistence) => {
                self.persistence = persistence;
                self.dirty = false;
            }
            Lacunarity(lacunarity) => {
                self.lacunarity = lacunarity;
                self.dirty = false;
            }
            Frequency(frequency) => {
                self.frequency = frequency;
                self.dirty = false;
            }
            Null => {
                self.dirty = true;
            }
        }
    }

    pub fn view(&self) -> Element<'a, FractalMessage> {
        use FractalMessage::*;
        let mut col = Column::new()
            .push(Rule::horizontal(10))
            .push("Fractal Noise")
            .push(
                LSlider::new("Octaves".to_string(), self.octaves, 1..=8, 1, Octaves, Null)
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
                        Null,
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
                        Null,
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
                        Null,
                    )
                    .decimals(2),
                )
        }
        col.into()
    }
}
