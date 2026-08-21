#![allow(dead_code)]

use wassily::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorleyDistance {
    Euclidean,
    EuclideanSquared,
    Manhattan,
    Chebyshev,
}

impl std::fmt::Display for WorleyDistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WorleyDistance::Euclidean => "Euclidean",
                WorleyDistance::EuclideanSquared => "Euclidean Sq",
                WorleyDistance::Manhattan => "Manhattan",
                WorleyDistance::Chebyshev => "Chebyshev",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorleyReturn {
    Distance,
    Value,
}

impl std::fmt::Display for WorleyReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WorleyReturn::Distance => "Distance",
                WorleyReturn::Value => "Value",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WorleyControls {
    pub frequency: f32,
    pub distance: Option<WorleyDistance>,
    pub return_type: Option<WorleyReturn>,
}

impl Default for WorleyControls {
    fn default() -> Self {
        Self {
            frequency: 1.0,
            distance: Some(WorleyDistance::Euclidean),
            return_type: Some(WorleyReturn::Distance),
        }
    }
}

impl WorleyControls {
    pub fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        use crate::gui::{numeric, pick_list, section};
        section(ui, "Worley");
        eframe::egui::Grid::new("worley")
            .spacing((15.0, 10.0))
            .min_col_width(90.0)
            .show(ui, |ui| {
                numeric(ui, "Frequency", &mut self.frequency, 1.0, 0.1..=8.0, 0.1, 1);
                pick_list(
                    ui,
                    "Distance Fn",
                    &[
                        WorleyDistance::Euclidean,
                        WorleyDistance::EuclideanSquared,
                        WorleyDistance::Manhattan,
                        WorleyDistance::Chebyshev,
                    ],
                    &mut self.distance,
                );
                pick_list(
                    ui,
                    "Return",
                    &[WorleyReturn::Distance, WorleyReturn::Value],
                    &mut self.return_type,
                );
            });
    }
}

/// Optional turbulence wrapper distorting any flow field's input
/// coordinates with Perlin noise.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TurbulenceControls {
    pub enabled: bool,
    pub frequency: f32,
    pub power: f32,
    pub roughness: u8,
}

impl Default for TurbulenceControls {
    fn default() -> Self {
        Self {
            enabled: false,
            frequency: 1.0,
            power: 1.0,
            roughness: 3,
        }
    }
}

impl TurbulenceControls {
    pub fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        use crate::gui::{numeric, section};
        section(ui, "Turbulence");
        eframe::egui::Grid::new("turbulence")
            .spacing((15.0, 10.0))
            .min_col_width(90.0)
            .show(ui, |ui| {
                ui.label("Enabled").on_hover_ui(|ui| {
                    ui.colored_label(
                        eframe::egui::Color32::ORANGE,
                        "Distorts the flow field with",
                    );
                    ui.colored_label(eframe::egui::Color32::ORANGE, "Perlin turbulence.");
                });
                ui.checkbox(&mut self.enabled, "");
                ui.end_row();
                if self.enabled {
                    numeric(ui, "Frequency", &mut self.frequency, 1.0, 0.1..=8.0, 0.1, 1);
                    numeric(ui, "Power", &mut self.power, 1.0, 0.0..=10.0, 0.1, 1);
                    numeric(ui, "Roughness", &mut self.roughness, 3, 1..=8, 1.0, 0);
                }
            });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NoiseControls {
    pub noise_function: Option<NoiseFunction>,
    pub noise_factor: f32,
    pub noise_scale: f32,
}

impl NoiseControls {
    pub fn new(noise_function: NoiseFunction, noise_scale: f32, noise_factor: f32) -> Self {
        Self {
            noise_function: Some(noise_function),
            noise_factor,
            noise_scale,
        }
    }

    pub fn set_noise_function(mut self, noise_function: NoiseFunction) -> Self {
        self.noise_function = Some(noise_function);
        self
    }

    pub fn set_noise_factor(mut self, noise_factor: f32) -> Self {
        self.noise_factor = noise_factor;
        self
    }

    pub fn set_noise_scale(mut self, noise_scale: f32) -> Self {
        self.noise_scale = noise_scale;
        self
    }
}

impl Default for NoiseControls {
    fn default() -> Self {
        Self {
            noise_function: Some(NoiseFunction::Fbm),
            noise_factor: 1.0,
            noise_scale: 4.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NoiseFunction {
    Fbm,
    BasicMulti,
    HybridMulti,
    Billow,
    Ridged,
    Value,
    Cylinders,
    Worley,
    Curl,
    Sinusoidal,
    Image,
}

impl std::fmt::Display for NoiseFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NoiseFunction::Fbm => "Fbm",
                NoiseFunction::BasicMulti => "Basic Multi",
                NoiseFunction::HybridMulti => "Hybrid Multi",
                NoiseFunction::Billow => "Billow",
                NoiseFunction::Ridged => "Ridged",
                NoiseFunction::Cylinders => "Cylinders",
                NoiseFunction::Value => "Value",
                NoiseFunction::Worley => "Worley",
                NoiseFunction::Curl => "Curl",
                NoiseFunction::Sinusoidal => "Sinusoidal",
                NoiseFunction::Image => "Image",
            }
        )
    }
}

pub struct Sinusoidal {
    x_freq: f64,
    y_freq: f64,
    x_exp: f64,
    y_exp: f64,
}

impl Sinusoidal {
    pub fn new(x_freq: f64, y_freq: f64, x_exp: f64, y_exp: f64) -> Self {
        Self {
            x_freq,
            y_freq,
            x_exp,
            y_exp,
        }
    }
}

impl NoiseFn<f64, 2> for Sinusoidal {
    fn get(&self, point: [f64; 2]) -> f64 {
        std::f64::consts::PI
            * (2.0
                + (self.x_freq * point[0]).sin().powf(self.x_exp)
                + (self.y_freq * point[1]).sin().powf(self.y_exp))
    }
}
