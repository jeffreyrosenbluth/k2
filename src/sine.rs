#![allow(dead_code)]

use crate::gui::numeric;
use eframe::egui;

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
        ui.separator();
        ui.label("Sine Noise");
        numeric(ui, "X Frequency", &mut self.xfreq, 0.1..=10.0, 0.1, 1);
        numeric(ui, "Y Frequency", &mut self.yfreq, 0.1..=10.0, 0.1, 1);
        numeric(ui, "X Exponent", &mut self.xexp, 1.0..=4.0, 1.0, 0);
        numeric(ui, "Y Exponent", &mut self.yexp, 1.0..=4.0, 1.0, 0);
    }
}
