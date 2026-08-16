//! egui helpers shared by all control panels, styled after mixel: controls
//! live in two-column grids (label left, widget right), sliders get a
//! reset-to-default button and shift-for-coarse steps, and help text shows
//! as orange hover lines on the label.

use eframe::egui::{self, Color32, RichText};
use std::fmt::Display;
use std::ops::RangeInclusive;

pub const SPACE: f32 = 7.0;

/// A standard grid row: a label (optionally with orange hover help lines),
/// a slider with an editable value box, and a reset-to-default button.
pub struct SliderRow<'a, T: egui::emath::Numeric> {
    label: &'a str,
    hover: &'a [&'a str],
    value: &'a mut T,
    default: T,
    range: RangeInclusive<T>,
    steps: (f64, f64),
    decimals: usize,
    clamp: bool,
    logarithmic: bool,
}

impl<'a, T: egui::emath::Numeric> SliderRow<'a, T> {
    pub fn new(
        label: &'a str,
        value: &'a mut T,
        default: T,
        range: RangeInclusive<T>,
    ) -> Self {
        Self {
            label,
            hover: &[],
            value,
            default,
            range,
            steps: (0.0, 0.0),
            decimals: 0,
            clamp: true,
            logarithmic: false,
        }
    }

    pub fn hover(mut self, lines: &'a [&'a str]) -> Self {
        self.hover = lines;
        self
    }

    /// Slider steps: fine, and coarse while shift is held. Zero = continuous.
    pub fn steps(mut self, fine: f64, coarse: f64) -> Self {
        self.steps = (fine, coarse);
        self
    }

    pub fn decimals(mut self, decimals: usize) -> Self {
        self.decimals = decimals;
        self
    }

    /// Let typed values exceed the slider range.
    pub fn unclamped(mut self) -> Self {
        self.clamp = false;
        self
    }

    /// Spread the low end of the range across most of the track.
    pub fn logarithmic(mut self) -> Self {
        self.logarithmic = true;
        self
    }

    pub fn show(self, ui: &mut egui::Ui) {
        let Self {
            label,
            hover,
            value,
            default,
            range,
            steps,
            decimals,
            clamp,
            logarithmic,
        } = self;
        let shift_held = ui.ctx().input(|i| i.modifiers.shift);
        let response = ui.label(label);
        if !hover.is_empty() {
            response.on_hover_ui(|ui| {
                for line in hover {
                    ui.colored_label(Color32::ORANGE, *line);
                }
            });
        }
        ui.horizontal(|ui| {
            let step = if shift_held { steps.1 } else { steps.0 };
            let mut slider = egui::Slider::new(&mut *value, range)
                .trailing_fill(true)
                .logarithmic(logarithmic)
                .fixed_decimals(decimals);
            if step > 0.0 {
                slider = slider.step_by(step);
            }
            if !clamp {
                slider = slider.clamping(egui::SliderClamping::Never);
            }
            ui.add(slider);
            if ui.small_button("\u{21ba}").clicked() {
                *value = default;
            }
        });
        ui.end_row();
    }
}

/// A grid row shorthand for the common slider case.
pub fn numeric<T: egui::emath::Numeric>(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut T,
    default: T,
    range: RangeInclusive<T>,
    step: f64,
    decimals: usize,
) {
    SliderRow::new(label, value, default, range)
        .steps(step, 5.0 * step)
        .decimals(decimals)
        .show(ui);
}

/// A grid row with a combo box over an optional value. Returns true if the
/// selection changed this frame.
pub fn pick_list<T: Copy + PartialEq + Display>(
    ui: &mut egui::Ui,
    label: &str,
    choices: &[T],
    value: &mut Option<T>,
) -> bool {
    ui.label(label);
    let mut changed = false;
    egui::ComboBox::from_id_salt(label)
        .width(150.0)
        .selected_text(value.map_or("None".to_string(), |v| v.to_string()))
        .show_ui(ui, |ui| {
            ui.set_min_width(60.0);
            for choice in choices {
                if ui
                    .selectable_value(value, Some(*choice), choice.to_string())
                    .changed()
                {
                    changed = true;
                }
            }
        });
    ui.end_row();
    changed
}

/// A grid row with a color swatch button and its rgb values.
pub fn color_picker(ui: &mut egui::Ui, label: &str, color: &mut egui::Color32) {
    ui.label(label);
    ui.horizontal(|ui| {
        ui.color_edit_button_srgba(color);
        ui.label(format!("{:3} {:3} {:3}", color.r(), color.g(), color.b()));
    });
    ui.end_row();
}

/// A section break: separator plus a centered bold title.
pub fn section(ui: &mut egui::Ui, title: &str) {
    ui.add_space(SPACE);
    ui.separator();
    ui.add_space(SPACE);
    ui.vertical_centered(|ui| {
        ui.label(RichText::new(title).strong().size(14.0));
    });
    ui.add_space(SPACE);
}

