use crate::gui::{color_picker, numeric, pick_list};
use crate::size::SizeControls;
use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DotControls {
    pub dot_style: Option<DotStyle>,
    pub size_controls: SizeControls,
    pub pearl_sides: u32,
    pub pearl_smoothness: u32,
    pub dot_stroke_color: egui::Color32,
}

impl Default for DotControls {
    fn default() -> Self {
        Self {
            dot_style: Some(DotStyle::Circle),
            size_controls: SizeControls::default(),
            pearl_sides: 4,
            pearl_smoothness: 3,
            dot_stroke_color: egui::Color32::WHITE,
        }
    }
}

impl DotControls {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        use DotStyle::*;
        pick_list(ui, "Dot Style", &[Circle, Square, Pearl], &mut self.dot_style);
        color_picker(ui, "Dot Stroke Color", &mut self.dot_stroke_color);
        self.size_controls.ui(ui);
        if self.dot_style == Some(Pearl) {
            numeric(ui, "Pearl Sides", &mut self.pearl_sides, 3..=8, 1.0, 0);
            numeric(
                ui,
                "Pearl Smoothness",
                &mut self.pearl_smoothness,
                0..=5,
                1.0,
                0,
            );
        }
    }
}
