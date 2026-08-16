#![allow(dead_code)]

use crate::gui::{numeric, section};
use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

impl SineControls {
    pub fn new(xfreq: f32, yfreq: f32, xexp: f32, yexp: f32) -> Self {
        Self {
            xfreq,
            yfreq,
            xexp,
            yexp,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let d = Self::default();
        section(ui, "Sine Noise");
        egui::Grid::new("sine")
            .spacing((15.0, 10.0))
            .min_col_width(90.0)
            .show(ui, |ui| {
                numeric(ui, "X Frequency", &mut self.xfreq, d.xfreq, 0.1..=10.0, 0.1, 1);
                numeric(ui, "Y Frequency", &mut self.yfreq, d.yfreq, 0.1..=10.0, 0.1, 1);
                numeric(ui, "X Exponent", &mut self.xexp, d.xexp, 1.0..=4.0, 1.0, 0);
                numeric(ui, "Y Exponent", &mut self.yexp, d.yexp, 1.0..=4.0, 1.0, 0);
            });
    }
}
