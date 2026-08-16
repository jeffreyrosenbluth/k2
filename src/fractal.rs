#![allow(dead_code)]

use crate::gui::{numeric, section};
use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
            // The noise crate's DEFAULT_LACUNARITY: pi * 2/3
            lacunarity: std::f32::consts::PI * 2.0 / 3.0,
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

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let d = Self::default();
        section(ui, "Fractal Noise");
        egui::Grid::new("fractal")
            .spacing((15.0, 10.0))
            .min_col_width(90.0)
            .show(ui, |ui| {
                numeric(ui, "Octaves", &mut self.octaves, d.octaves, 1..=8, 1.0, 0);
                if self.octaves > 1 {
                    numeric(
                        ui,
                        "Persistence",
                        &mut self.persistence,
                        d.persistence,
                        0.05..=0.95,
                        0.05,
                        2,
                    );
                    numeric(
                        ui,
                        "Lacunarity",
                        &mut self.lacunarity,
                        d.lacunarity,
                        0.1..=4.0,
                        0.1,
                        1,
                    );
                    numeric(
                        ui,
                        "Frequency",
                        &mut self.frequency,
                        d.frequency,
                        0.1..=4.0,
                        0.1,
                        1,
                    );
                }
            });
    }
}
