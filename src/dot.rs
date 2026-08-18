use crate::gui::{color_picker, numeric, pick_list};
use crate::size::SizeControls;
use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DotStyle {
    Circle,
    Square,
    Pearl,
}

impl std::fmt::Display for DotStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DotStyle::Circle => "Circle",
                DotStyle::Square => "Square",
                DotStyle::Pearl => "Pearl",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DotControls {
    pub dot_style: Option<DotStyle>,
    pub size_controls: SizeControls,
    pub pearl_sides: u32,
    pub pearl_smoothness: u32,
    /// When false the dots are drawn without any stroke.
    #[serde(default = "default_stroke")]
    pub stroke: bool,
    pub dot_stroke_color: egui::Color32,
}

fn default_stroke() -> bool {
    true
}

impl Default for DotControls {
    fn default() -> Self {
        Self {
            dot_style: Some(DotStyle::Circle),
            size_controls: SizeControls::default(),
            pearl_sides: 4,
            pearl_smoothness: 3,
            stroke: true,
            dot_stroke_color: egui::Color32::WHITE,
        }
    }
}

impl DotControls {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        use DotStyle::*;
        let d = Self::default();
        egui::Grid::new("dot")
            .spacing((15.0, 10.0))
            .min_col_width(90.0)
            .show(ui, |ui| {
                pick_list(ui, "Dot Style", &[Circle, Square, Pearl], &mut self.dot_style);
                ui.label("Stroke").on_hover_ui(|ui| {
                    ui.colored_label(
                        egui::Color32::ORANGE,
                        "Uncheck to draw dots with",
                    );
                    ui.colored_label(egui::Color32::ORANGE, "no stroke at all.");
                });
                ui.checkbox(&mut self.stroke, "");
                ui.end_row();
                if self.stroke {
                    color_picker(ui, "Stroke Color", &mut self.dot_stroke_color);
                }
            });
        self.size_controls.ui(ui);
        if self.dot_style == Some(Pearl) {
            egui::Grid::new("pearl")
                .spacing((15.0, 10.0))
                .min_col_width(90.0)
                .show(ui, |ui| {
                    numeric(
                        ui,
                        "Pearl Sides",
                        &mut self.pearl_sides,
                        d.pearl_sides,
                        3..=8,
                        1.0,
                        0,
                    );
                    numeric(
                        ui,
                        "Smoothness",
                        &mut self.pearl_smoothness,
                        d.pearl_smoothness,
                        0..=5,
                        1.0,
                        0,
                    );
                });
        }
    }
}
