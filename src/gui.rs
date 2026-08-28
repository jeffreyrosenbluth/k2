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
            // Edits (not Always): user input clamps and snaps to steps, but
            // programmatic values (Random, loaded params) are left intact —
            // Always would silently rewrite them on the next frame.
            slider = slider.clamping(if clamp {
                egui::SliderClamping::Edits
            } else {
                egui::SliderClamping::Never
            });
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

/// A grid row with a color swatch button and its rgb values. The button
/// opens egui's picker augmented with a hex field, recent colors, and
/// swatches of the named palettes.
pub fn color_picker(ui: &mut egui::Ui, label: &str, color: &mut egui::Color32) {
    ui.label(label);
    ui.horizontal(|ui| {
        color_edit_button(ui, color);
        ui.label(format!("{:3} {:3} {:3}", color.r(), color.g(), color.b()));
    });
    ui.end_row();
}

fn color_edit_button(ui: &mut egui::Ui, color: &mut Color32) {
    let popup_id = ui.auto_id_with("k2 color popup");
    let opened_key = popup_id.with("opened with");
    let recents_key = egui::Id::new("k2 recent colors");
    let button = ui.add(
        egui::Button::new("")
            .fill(*color)
            .min_size(egui::vec2(28.0, 16.0)),
    );

    egui::Popup::menu(&button)
        .id(popup_id)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            ui.spacing_mut().slider_width = 220.0;
            egui::color_picker::color_picker_color32(
                ui,
                color,
                egui::color_picker::Alpha::Opaque,
            );
            hex_row(ui, color);
            let recents: Vec<Color32> = ui
                .ctx()
                .data_mut(|d| d.get_temp(recents_key))
                .unwrap_or_default();
            if !recents.is_empty() {
                ui.separator();
                ui.label("Recent");
                swatch_grid(ui, &recents, color);
            }
            ui.separator();
            ui.label("Swatches");
            swatch_grid(ui, &crate::color::swatches(), color);
        });

    // Track the color the popup opened with; when it closes on a different
    // color, remember the result in the recent list.
    let open = egui::Popup::is_id_open(ui.ctx(), popup_id);
    let current = *color;
    ui.ctx().data_mut(|d| {
        let slot: &mut Option<Color32> = d.get_temp_mut_or_default(opened_key);
        if open {
            if slot.is_none() {
                *slot = Some(current);
            }
        } else if let Some(initial) = slot.take() {
            if initial != current {
                let recents: &mut Vec<Color32> = d.get_temp_mut_or_default(recents_key);
                recents.retain(|c| *c != current);
                recents.insert(0, current);
                recents.truncate(8);
            }
        }
    });
}

fn hex_row(ui: &mut egui::Ui, color: &mut Color32) {
    let id = ui.id().with("hex text");
    let mut text: String = ui
        .data_mut(|d| d.get_temp(id))
        .unwrap_or_else(|| hex_string(*color));
    ui.horizontal(|ui| {
        ui.label("Hex");
        let response = ui.add(
            egui::TextEdit::singleline(&mut text)
                .desired_width(80.0)
                .font(egui::TextStyle::Monospace),
        );
        if response.lost_focus() {
            if let Some(c) = parse_hex(&text) {
                *color = c;
            }
        }
        if !response.has_focus() {
            text = hex_string(*color);
        }
    });
    ui.data_mut(|d| d.insert_temp(id, text));
}

fn hex_string(c: Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", c.r(), c.g(), c.b())
}

fn parse_hex(s: &str) -> Option<Color32> {
    let s = s.trim().trim_start_matches('#');
    if s.len() != 6 {
        return None;
    }
    let n = u32::from_str_radix(s, 16).ok()?;
    Some(Color32::from_rgb((n >> 16) as u8, (n >> 8) as u8, n as u8))
}

fn swatch_grid(ui: &mut egui::Ui, colors: &[Color32], color: &mut Color32) {
    // Fixed 8-wide rows so the popup never grows past the picker itself.
    for row in colors.chunks(8) {
        ui.horizontal(|ui| {
            for &c in row {
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(18.0, 18.0), egui::Sense::click());
                let stroke = if c == *color {
                    egui::Stroke::new(2.0, ui.visuals().strong_text_color())
                } else {
                    egui::Stroke::new(1.0, ui.visuals().weak_text_color())
                };
                ui.painter()
                    .rect(rect, 3.0, c, stroke, egui::StrokeKind::Inside);
                if response.clicked() {
                    *color = c;
                }
                response.on_hover_text(hex_string(c));
            }
        });
    }
}

/// A centered mixel-style action button with orange hover help lines.
pub fn action_button(ui: &mut egui::Ui, label: &str, enabled: bool, hover: &[&str]) -> bool {
    let mut clicked = false;
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        let response = ui
            .add_enabled(
                enabled,
                egui::Button::new(RichText::new(label).strong().size(16.0))
                    .min_size(egui::Vec2::new(150.0, 25.0)),
            )
            .on_hover_ui(|ui| {
                for line in hover {
                    ui.colored_label(Color32::ORANGE, *line);
                }
            });
        clicked = response.clicked();
    });
    clicked
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

