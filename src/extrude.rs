use crate::gradient::{GradStyle, GradStyle::Plain};
use crate::gui::{numeric, pick_list, SPACE};
use crate::size::SizeControls;
use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExtrudeDirection {
    Vertical,
    Horizontal,
    Normal,
}

impl std::fmt::Display for ExtrudeDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExtrudeDirection::Vertical => "Vertical",
                ExtrudeDirection::Horizontal => "Horizontal",
                ExtrudeDirection::Normal => "Normal",
            }
        )
    }
}

fn default_noise_scale() -> f32 {
    4.0
}

fn default_noise_strength() -> f32 {
    25.0
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ExtrudeControls {
    pub size_controls: SizeControls,
    pub grad_style: Option<GradStyle>,
    pub direction: Option<ExtrudeDirection>,
    /// Extrude between two edge curves, each the flow curve displaced along
    /// the extrude direction by an independent Perlin field.
    #[serde(default)]
    pub noisy: bool,
    #[serde(default = "default_noise_scale")]
    pub noise_scale: f32,
    #[serde(default = "default_noise_strength")]
    pub noise_strength: f32,
}

impl Default for ExtrudeControls {
    fn default() -> Self {
        Self {
            size_controls: SizeControls::default(),
            grad_style: Some(Plain),
            direction: Some(ExtrudeDirection::Vertical),
            noisy: false,
            noise_scale: default_noise_scale(),
            noise_strength: default_noise_strength(),
        }
    }
}

impl ExtrudeControls {
    pub fn new(size_controls: SizeControls, grad_style: Option<GradStyle>) -> Self {
        Self {
            size_controls,
            grad_style,
            ..Self::default()
        }
    }

    pub fn set_direction(mut self, direction: ExtrudeDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        use ExtrudeDirection::*;
        use GradStyle::{Dark, DarkFiber, Double, Fiber, Light, LightFiber, Plain};
        self.size_controls.ui(ui);
        ui.add_space(2.0 * SPACE);
        egui::Grid::new("extrude")
            .spacing((15.0, 10.0))
            .min_col_width(90.0)
            .show(ui, |ui| {
                pick_list(
                    ui,
                    "Extrude Dir",
                    &[Vertical, Horizontal, Normal],
                    &mut self.direction,
                );
                pick_list(
                    ui,
                    "Gradient Style",
                    &[GradStyle::None, Plain, Double, Light, Dark, Fiber, LightFiber, DarkFiber],
                    &mut self.grad_style,
                );
                ui.label("Noisy Edges").on_hover_ui(|ui| {
                    ui.colored_label(
                        egui::Color32::ORANGE,
                        "Extrude between two curves, each the",
                    );
                    ui.colored_label(
                        egui::Color32::ORANGE,
                        "flow curve perturbed by Perlin noise",
                    );
                    ui.colored_label(
                        egui::Color32::ORANGE,
                        "along the extrude direction.",
                    );
                });
                ui.checkbox(&mut self.noisy, "");
                ui.end_row();
                if self.noisy {
                    numeric(
                        ui,
                        "Edge Scale",
                        &mut self.noise_scale,
                        default_noise_scale(),
                        0.5..=20.0,
                        0.1,
                        1,
                    );
                    numeric(
                        ui,
                        "Edge Strength",
                        &mut self.noise_strength,
                        default_noise_strength(),
                        0.0..=200.0,
                        1.0,
                        0,
                    );
                }
            });
    }
}
