#![allow(dead_code)]

use wassily::prelude::*;
use serde::{Deserialize, Serialize};

/// A Worley generator configured from the Worley controls.
pub fn build_worley(c: &WorleyControls) -> wassily::prelude::Worley {
    use noise::core::worley::distance_functions;
    use wassily::prelude::Worley;
    let distance_fn = match c.distance.unwrap_or(WorleyDistance::Euclidean) {
        WorleyDistance::Euclidean => distance_functions::euclidean as fn(&[f64], &[f64]) -> f64,
        WorleyDistance::EuclideanSquared => distance_functions::euclidean_squared,
        WorleyDistance::Manhattan => distance_functions::manhattan,
        WorleyDistance::Chebyshev => distance_functions::chebyshev,
    };
    let return_type = match c.return_type.unwrap_or(WorleyReturn::Distance) {
        WorleyReturn::Distance => noise::core::worley::ReturnType::Distance,
        WorleyReturn::Value => noise::core::worley::ReturnType::Value,
    };
    Worley::default()
        .set_frequency(c.frequency as f64)
        .set_distance_function(distance_fn)
        .set_return_type(return_type)
}

thread_local! {
    static SOURCE_WORLEY_CONFIG: std::cell::Cell<WorleyControls> =
        std::cell::Cell::new(WorleyControls {
            frequency: 1.0,
            distance: Some(WorleyDistance::Euclidean),
            return_type: Some(WorleyReturn::Distance),
        });
}

/// Set the configuration the next `SourceWorley::default()` calls pick up.
/// The fractal generators build their sources through `Default`, which is
/// the only hook available for configuring them.
pub fn set_source_worley_config(c: WorleyControls) {
    SOURCE_WORLEY_CONFIG.with(|cell| cell.set(c));
}

/// Worley as a fractal source noise, configured via the thread local above.
pub struct SourceWorley(wassily::prelude::Worley);

impl Default for SourceWorley {
    fn default() -> Self {
        SourceWorley(SOURCE_WORLEY_CONFIG.with(|cell| build_worley(&cell.get())))
    }
}

impl wassily::prelude::Seedable for SourceWorley {
    fn set_seed(self, seed: u32) -> Self {
        SourceWorley(self.0.set_seed(seed))
    }
    fn seed(&self) -> u32 {
        self.0.seed()
    }
}

impl NoiseFn<f64, 2> for SourceWorley {
    fn get(&self, point: [f64; 2]) -> f64 {
        self.0.get(point)
    }
}

/// The base noise the fractal generators (Fbm, Billow, ...) are built on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NoiseSource {
    Perlin,
    Worley,
}

impl std::fmt::Display for NoiseSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NoiseSource::Perlin => "Perlin",
                NoiseSource::Worley => "Worley",
            }
        )
    }
}

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
    /// Flow along the level contours of the field (its curl) instead of
    /// using its value as the flow angle directly.
    #[serde(default)]
    pub curl: bool,
}

impl NoiseControls {
    pub fn new(noise_function: NoiseFunction, noise_scale: f32, noise_factor: f32) -> Self {
        Self {
            noise_function: Some(noise_function),
            noise_factor,
            noise_scale,
            curl: false,
        }
    }

    pub fn set_curl(mut self, curl: bool) -> Self {
        self.curl = curl;
        self
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
            curl: false,
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
        // Normalized to [-1, 1] like every other noise, so Noise Factor and
        // the value-based color modes behave consistently. powi keeps
        // negative sine bases well-defined for the integer exponents.
        0.5 * ((self.x_freq * point[0]).sin().powi(self.x_exp.round() as i32)
            + (self.y_freq * point[1]).sin().powi(self.y_exp.round() as i32))
    }
}
