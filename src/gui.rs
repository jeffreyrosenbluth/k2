//! Small egui helpers shared by all control panels.

use eframe::egui;
use std::fmt::Display;
use std::ops::RangeInclusive;

/// A labeled combo box over an optional value. Returns true if the selection
/// changed this frame.
pub fn pick_list<T: Copy + PartialEq + Display>(
    ui: &mut egui::Ui,
    label: &str,
    choices: &[T],
    value: &mut Option<T>,
) -> bool {
    ui.label(label);
    let mut changed = false;
    egui::ComboBox::from_id_salt(label)
        .width(175.0)
        .selected_text(value.map_or("None".to_string(), |v| v.to_string()))
        .show_ui(ui, |ui| {
            for choice in choices {
                if ui
                    .selectable_value(value, Some(*choice), choice.to_string())
                    .changed()
                {
                    changed = true;
                }
            }
        });
    changed
}

/// A labeled slider with an editable value box.
pub fn numeric<T: eframe::emath::Numeric>(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut T,
    range: RangeInclusive<T>,
    step: f64,
    decimals: usize,
) {
    ui.label(label);
    ui.add(
        egui::Slider::new(value, range)
            .step_by(step)
            .fixed_decimals(decimals)
            .trailing_fill(true),
    );
}

/// A color swatch button with its rgb values printed beside it.
pub fn color_picker(ui: &mut egui::Ui, label: &str, color: &mut egui::Color32) {
    ui.horizontal(|ui| {
        ui.color_edit_button_srgba(color);
        ui.label(format!(
            "{label}  {:3} {:3} {:3}",
            color.r(),
            color.g(),
            color.b()
        ));
    });
}
