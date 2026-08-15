use crate::gradient::{GradStyle, GradStyle::Plain};
use crate::gui::pick_list;
use crate::size::SizeControls;
use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtrudeControls {
    pub size_controls: SizeControls,
    pub grad_style: Option<GradStyle>,
    pub direction: Option<ExtrudeDirection>,
}

impl Default for ExtrudeControls {
    fn default() -> Self {
        Self {
            size_controls: SizeControls::default(),
            grad_style: Some(Plain),
            direction: Some(ExtrudeDirection::Vertical),
        }
    }
}

impl ExtrudeControls {
    pub fn new(size_controls: SizeControls, grad_style: Option<GradStyle>) -> Self {
        Self {
            size_controls,
            grad_style,
            direction: Some(ExtrudeDirection::Vertical),
        }
    }

    pub fn set_direction(mut self, direction: ExtrudeDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        use ExtrudeDirection::*;
        use GradStyle::*;
        self.size_controls.ui(ui);
        pick_list(
            ui,
            "Extrude Direction",
            &[Vertical, Horizontal, Normal],
            &mut self.direction,
        );
        pick_list(
            ui,
            "Gradient Style",
            &[Plain, Light, Dark, Fiber, LightFiber, DarkFiber],
            &mut self.grad_style,
        );
    }
}
