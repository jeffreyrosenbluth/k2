use directories::UserDirs;
use eframe::egui;
use std::path::PathBuf;
use std::sync::atomic::Ordering;

mod art;
mod background;
mod color;
mod common;
mod dot;
mod extrude;
mod field;
mod fractal;
mod gradient;
mod gui;
mod location;
mod noise;
mod presets;
mod sine;
mod size;

use crate::art::draw;
use crate::background::Background;
use crate::common::*;
use crate::gui::{color_picker, numeric, pick_list};
use crate::location::Location;
use crate::noise::NoiseFunction;
use crate::presets::*;

pub fn main() -> eframe::Result {
    env_logger::init();
    if std::env::var("K2_BENCH").is_ok() {
        bench();
        return Ok(());
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1500.0, 1100.0])
            .with_title("K2"),
        ..Default::default()
    };
    eframe::run_native(
        "K2",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::new(K2::new()))
        }),
    )
}

pub fn print(controls: Controls) {
    let canvas = draw(&controls, true);
    let dirs = UserDirs::new().unwrap();
    let dir = dirs.download_dir().unwrap();
    let path = format!(r"{}/{}", dir.to_string_lossy(), "k2");
    let mut num = 0;
    let mut sketch = PathBuf::from(format!(r"{path}_{num}"));
    sketch.set_extension("png");
    while sketch.exists() {
        num += 1;
        sketch = PathBuf::from(format!(r"{path}_{num}"));
        sketch.set_extension("png");
    }
    canvas.save_png(&sketch);
}

fn load_preset(p: Preset) -> Controls {
    use Preset::*;
    let mut controls = match p {
        Ribbons => ribbons(),
        Solar => solar(),
        RiverStones => river_stones(),
        Vortex => vortex(),
        Canyon => canyon(),
        Fence => fence(),
        Splat => splat(),
        Tubes => tubes(),
        Ducts => ducts(),
        Symmetry => symmetry(),
        PomPom => pompom(),
        RedDwarf => red_dwarf(),
        Ridges => ridges(),
    };
    controls.preset = Some(p);
    controls
}

impl K2 {
    fn left_panel(&mut self, ui: &mut egui::Ui) {
        use crate::common::CurveDirection::*;
        use crate::CurveStyle::*;
        use Background::*;
        use NoiseFunction::*;
        use Preset::*;

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Width");
                ui.add(
                    egui::TextEdit::singleline(&mut self.controls.width)
                        .desired_width(70.0)
                        .hint_text("1000"),
                );
            });
            ui.vertical(|ui| {
                ui.label("Height");
                ui.add(
                    egui::TextEdit::singleline(&mut self.controls.height)
                        .desired_width(70.0)
                        .hint_text("1000"),
                );
            });
        });

        let mut preset = self.controls.preset;
        if pick_list(
            ui,
            "Preset",
            &[
                Ribbons, Solar, RiverStones, Vortex, Canyon, Fence, Splat, Tubes, Ducts, Symmetry,
                PomPom, RedDwarf, Ridges,
            ],
            &mut preset,
        ) {
            if let Some(p) = preset {
                self.controls = load_preset(p);
            }
        }

        pick_list(
            ui,
            "Curve Style",
            &[Line, Dots, Extrusion],
            &mut self.controls.curve_style,
        );
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.controls.curve_direction, Some(OneSided), "One Sided");
            ui.radio_value(&mut self.controls.curve_direction, Some(TwoSided), "Two Sided");
        });
        pick_list(
            ui,
            "Flow Field",
            &[
                Fbm, Billow, Ridged, Value, Cylinders, Worley, Curl, Magnet, Gravity, Sinusoidal,
            ],
            &mut self.controls.noise_controls.noise_function,
        );
        pick_list(
            ui,
            "Curve Locations",
            &[
                Location::Grid,
                Location::Rand,
                Location::Halton,
                Location::Poisson,
                Location::Circle,
                Location::Lissajous,
            ],
            &mut self.controls.location,
        );
        pick_list(
            ui,
            "Background Style",
            &[LightGrain, LightFiber, DarkGrain, DarkFiber, ColorGrain],
            &mut self.controls.background,
        );
        numeric(ui, "Density", &mut self.controls.density, 5.0..=100.0, 5.0, 0);
        numeric(
            ui,
            "Point Spacing",
            &mut self.controls.spacing,
            1.0..=100.0,
            1.0,
            0,
        );
        numeric(
            ui,
            "Curve Length",
            &mut self.controls.curve_length,
            0..=400,
            1.0,
            0,
        );
        numeric(
            ui,
            "Noise Scale",
            &mut self.controls.noise_controls.noise_scale,
            0.5..=20.0,
            0.1,
            1,
        );
        numeric(
            ui,
            "Noise Factor",
            &mut self.controls.noise_controls.noise_factor,
            0.5..=10.0,
            0.1,
            1,
        );
        numeric(
            ui,
            "Convergence Speed",
            &mut self.controls.speed,
            0.01..=1.0,
            0.01,
            2,
        );
        self.controls.color_mode_controls.ui(ui);
        numeric(
            ui,
            "Stroke Width",
            &mut self.controls.stroke_width,
            0.0..=25.0,
            0.5,
            1,
        );
        ui.checkbox(&mut self.controls.border, "Border");

        ui.add_space(5.0);
        let exporting = self.exporting.load(Ordering::Relaxed);
        if ui
            .add_enabled(!exporting, egui::Button::new("Export"))
            .clicked()
        {
            self.exporting.store(true, Ordering::Relaxed);
            let controls = self.controls.clone();
            let flag = self.exporting.clone();
            std::thread::spawn(move || {
                print(controls);
                flag.store(false, Ordering::Relaxed);
            });
        }
        if exporting {
            ui.spinner();
        }
    }

    fn right_panel(&mut self, ui: &mut egui::Ui) {
        if self.controls.curve_style == Some(CurveStyle::Extrusion) {
            self.controls.extrude_controls.ui(ui);
        } else if self.controls.curve_style == Some(CurveStyle::Dots) {
            self.controls.dot_controls.ui(ui);
        }
        if matches!(
            self.controls.noise_controls.noise_function,
            Some(NoiseFunction::Fbm)
                | Some(NoiseFunction::Billow)
                | Some(NoiseFunction::Ridged)
                | Some(NoiseFunction::Curl)
        ) {
            self.controls.fractal_controls.ui(ui);
        }
        if self.controls.noise_controls.noise_function == Some(NoiseFunction::Sinusoidal) {
            self.controls.sin_controls.ui(ui);
        }
        if self.controls.background == Some(Background::ColorGrain) {
            ui.separator();
            color_picker(ui, "Grain Color", &mut self.controls.grain_color);
        }
    }
}

impl eframe::App for K2 {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        egui::Panel::left("controls")
            .exact_size(250.0)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 10.0;
                ui.spacing_mut().slider_width = 150.0;
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(5.0);
                    self.left_panel(ui);
                });
            });
        egui::Panel::right("style_controls")
            .exact_size(250.0)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 10.0;
                ui.spacing_mut().slider_width = 150.0;
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(5.0);
                    self.right_panel(ui);
                });
            });
        egui::CentralPanel::default().show(ui, |ui| {
            if let Some(texture) = &self.texture {
                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    ui.add(
                        egui::Image::new(texture)
                            .max_size(ui.available_size())
                            .maintain_aspect_ratio(true),
                    );
                });
            }
        });

        // Regenerate the artwork once the user finishes an interaction:
        // no mouse button held (slider drags) and no focused widget (text
        // edits commit on Enter or focus loss).
        let interacting = ctx.input(|i| i.pointer.any_down())
            || ctx.memory(|m| m.focused().is_some());
        if self.texture.is_none() {
            self.regenerate(&ctx);
        } else if !interacting && self.controls != self.last_drawn {
            // Manual edits clear the preset selection; picking a preset keeps it.
            if self.controls.preset == self.last_drawn.preset {
                self.controls.preset = None;
            }
            self.regenerate(&ctx);
        }

        if self.exporting.load(Ordering::Relaxed) {
            ctx.request_repaint_after(std::time::Duration::from_millis(250));
        }
    }
}

fn bench() {
    use crate::background::Background as BgKind;
    for bg in [
        BgKind::LightFiber,
        BgKind::LightGrain,
        BgKind::DarkGrain,
        BgKind::ColorGrain,
    ] {
        let mut controls = ribbons();
        controls.background = Some(bg);
        let t = std::time::Instant::now();
        let canvas = draw(&controls, false);
        println!(
            "{bg}: {:?} ({}x{})",
            t.elapsed(),
            canvas.width(),
            canvas.height()
        );
    }
}
