use crate::gradient::{GradStyle, GradStyle::Plain};
use crate::gui::pick_list;
use crate::size::SizeControls;
use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtrudeControls {
    pub size_controls: SizeControls,
    pub grad_style: Option<GradStyle>,
}

impl Default for ExtrudeControls {
    fn default() -> Self {
        Self {
            size_controls: SizeControls::default(),
            grad_style: Some(Plain),
        }
    }
}

impl ExtrudeControls {
    pub fn new(size_controls: SizeControls, grad_style: Option<GradStyle>) -> Self {
        Self {
            size_controls,
            grad_style,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        use GradStyle::*;
        self.size_controls.ui(ui);
        pick_list(
            ui,
            "Gradient Style",
            &[Plain, Light, Dark, Fiber, LightFiber, DarkFiber],
            &mut self.grad_style,
        );
    }
}
