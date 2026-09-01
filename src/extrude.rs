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

fn default_noise_amount() -> f32 {
    0.5
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ExtrudeControls {
    pub size_controls: SizeControls,
    pub grad_style: Option<GradStyle>,
    pub direction: Option<ExtrudeDirection>,
    // The serde defaults keep saved settings from before these fields loadable.
    #[serde(default)]
    pub noisy_edges: bool,
    #[serde(default)]
    pub independent_edges: bool,
    #[serde(default = "default_noise_scale")]
    pub edge_noise_scale: f32,
    #[serde(default = "default_noise_amount")]
    pub edge_noise_amount: f32,
}

impl Default for ExtrudeControls {
    fn default() -> Self {
        Self {
            size_controls: SizeControls::default(),
            grad_style: Some(Plain),
            direction: Some(ExtrudeDirection::Vertical),
            noisy_edges: false,
            independent_edges: false,
            edge_noise_scale: default_noise_scale(),
            edge_noise_amount: default_noise_amount(),
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
                        "Extrude between two Perlin noise",
                    );
                    ui.colored_label(egui::Color32::ORANGE, "perturbations of the curve.");
                });
                ui.checkbox(&mut self.noisy_edges, "");
                ui.end_row();
                if self.noisy_edges {
                    ui.label("Independent").on_hover_ui(|ui| {
                        ui.colored_label(
                            egui::Color32::ORANGE,
                            "Each curve's edges undulate on their",
                        );
                        ui.colored_label(
                            egui::Color32::ORANGE,
                            "own noise instead of a shared field.",
                        );
                    });
                    ui.checkbox(&mut self.independent_edges, "");
                    ui.end_row();
                    numeric(
                        ui,
                        "Edge Noise Scale",
                        &mut self.edge_noise_scale,
                        default_noise_scale(),
                        0.5..=20.0,
                        0.5,
                        1,
                    );
                    numeric(
                        ui,
                        "Edge Noise Amount",
                        &mut self.edge_noise_amount,
                        default_noise_amount(),
                        0.0..=2.0,
                        0.05,
                        2,
                    );
                }
            });
    }
}
