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
mod imgnoise;
mod location;
mod noise;
mod presets;
mod sine;
mod size;

use crate::art::draw;
use crate::background::Background;
use crate::common::*;
use crate::gui::{action_button, color_picker, numeric, pick_list, section, SliderRow, SPACE};
use crate::location::Location;
use crate::noise::NoiseFunction;
use crate::presets::*;

pub fn main() -> eframe::Result {
    env_logger::init();
    if std::env::var("K2_BENCH").is_ok() {
        bench();
        return Ok(());
    }
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../assets/icons/icon_256.png"))
        .expect("embedded icon is a valid png");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1500.0, 950.0])
            .with_title("K2")
            .with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        "K2",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            let mut app = K2::new();
            // Restore the controls from the previous session.
            if let Some(storage) = cc.storage {
                if let Some(controls) = eframe::get_value::<Controls>(storage, eframe::APP_KEY) {
                    app.last_drawn = controls.clone();
                    app.controls = controls;
                }
            }
            Ok(Box::new(app))
        }),
    )
}

/// Render the artwork at full resolution and save it to `path`, along with
/// a json file of the parameters that produced it.
pub fn print(controls: Controls, mut path: PathBuf) {
    if path.extension().is_none() {
        path.set_extension("png");
    }
    let scale = std::cmp::max(controls.width, controls.height).max(1) as f32 / 1000.0;
    let canvas = draw(&controls, scale);
    canvas.save_png(&path);
    let params = path.with_extension("json");
    match serde_json::to_string_pretty(&controls) {
        Ok(json) => {
            if let Err(e) = std::fs::write(&params, json) {
                eprintln!("failed to write {}: {e}", params.display());
            }
        }
        Err(e) => eprintln!("failed to serialize parameters: {e}"),
    }
}

/// The first `k2_N.png` name not already present in `dir`.
fn next_sketch_name(dir: &std::path::Path) -> String {
    let mut num = 0;
    while dir.join(format!("k2_{num}.png")).exists() {
        num += 1;
    }
    format!("k2_{num}.png")
}

fn load_preset(p: Preset) -> Controls {
    use Preset::*;
    let mut controls = match p {
        Ribbons => ribbons(),
        Worms => worms(),
        Solar => solar(),
        Vortex => vortex(),
        Canyon => canyon(),
        Splat => splat(),
        Tubes => tubes(),
        Ducts => ducts(),
        RedDwarf => red_dwarf(),
    };
    controls.preset = Some(p);
    controls
}

/// Fresh controls with every creative parameter randomized; size stays
/// 1080 x 1080, no preset is selected, and the noise image is kept.
fn random_controls(image_noise: crate::imgnoise::ImageNoiseControls) -> Controls {
    use crate::color::{ColorBy, ColorMode, Palettes};
    use crate::dot::DotStyle;
    use crate::extrude::ExtrudeDirection;
    use crate::gradient::GradStyle;
    use crate::noise::{WorleyDistance, WorleyReturn};
    use crate::size::{Dir, SizeFn};
    use rand::Rng;

    let mut rng = rand::rng();
    let color =
        |rng: &mut rand::rngs::ThreadRng| egui::Color32::from_rgb(rng.random(), rng.random(), rng.random());

    let styles = [
        CurveStyle::Line,
        CurveStyle::Dots,
        CurveStyle::Extrusion,
        CurveStyle::Strips,
    ];
    let locations = [
        Location::Grid,
        Location::Rand,
        Location::Halton,
        Location::Poisson,
        Location::Circle,
        Location::Lissajous,
        Location::Box,
        Location::Line,
        Location::Even,
    ];
    let mut noises = vec![
        NoiseFunction::Fbm,
        NoiseFunction::BasicMulti,
        NoiseFunction::HybridMulti,
        NoiseFunction::Billow,
        NoiseFunction::Ridged,
        NoiseFunction::Value,
        NoiseFunction::Cylinders,
        NoiseFunction::Worley,
        NoiseFunction::Curl,
        NoiseFunction::Sinusoidal,
    ];
    if image_noise.path.is_some() {
        noises.push(NoiseFunction::Image);
    }
    let palettes = [
        Palettes::Royalty,
        Palettes::DeltaBlues,
        Palettes::PinotNoir,
        Palettes::Emerald,
        Palettes::Scepter,
        Palettes::Fire,
        Palettes::Perfume,
        Palettes::Rose,
        Palettes::GrayScale,
        Palettes::PorcoRosso,
        Palettes::SpiritedAway,
        Palettes::MonoRed,
        Palettes::MonoGreen,
        Palettes::MonoBlue,
    ];
    let color_bys = [
        ColorBy::Random,
        ColorBy::AlongCurve,
        ColorBy::Cycle,
        ColorBy::Region,
        ColorBy::Order,
        ColorBy::PositionX,
        ColorBy::PositionY,
        ColorBy::Radial,
        ColorBy::FlowAngle,
        ColorBy::NoiseValue,
    ];
    let grads = [
        GradStyle::Plain,
        GradStyle::Double,
        GradStyle::Light,
        GradStyle::Dark,
        GradStyle::Fiber,
        GradStyle::LightFiber,
        GradStyle::DarkFiber,
    ];
    let size_fns = [
        SizeFn::Constant,
        SizeFn::Expanding,
        SizeFn::Contracting,
        SizeFn::Periodic,
    ];
    let dirs = [Dir::Both, Dir::Horizontal, Dir::Vertical];

    let mut c = Controls {
        preset: None,
        width: 1080,
        height: 1080,
        image_noise,
        ..Default::default()
    };
    c.curve_style = Some(styles[rng.random_range(0..styles.len())]);
    c.curve_direction = Some(if rng.random_bool(0.5) {
        CurveDirection::OneSided
    } else {
        CurveDirection::TwoSided
    });
    c.location = Some(locations[rng.random_range(0..locations.len())]);
    c.spacing = rng.random_range(1.0..=10.0f32).round();
    c.curve_length = rng.random_range(30..=500);
    c.hide_ends = rng.random_bool(0.3);
    c.density = rng.random_range(20.0..=100.0f32).round();
    c.noise_controls.noise_function = Some(noises[rng.random_range(0..noises.len())]);
    c.noise_controls.noise_scale = rng.random_range(0.5..=8.0);
    c.noise_controls.noise_factor = rng.random_range(0.3..=4.0);
    c.speed = 10.0f32.powf(rng.random_range(-1.3..=0.0f32));
    c.fractal_controls.octaves = rng.random_range(1..=6);
    c.fractal_controls.persistence = rng.random_range(0.2..=0.8);
    c.fractal_controls.lacunarity = rng.random_range(1.5..=3.0);
    c.fractal_controls.frequency = rng.random_range(0.5..=2.0);
    c.sin_controls.xfreq = rng.random_range(0.5..=5.0);
    c.sin_controls.yfreq = rng.random_range(0.5..=5.0);
    c.sin_controls.xexp = rng.random_range(1.0..=4.0f32).round();
    c.sin_controls.yexp = rng.random_range(1.0..=4.0f32).round();
    c.worley.frequency = rng.random_range(0.5..=4.0);
    c.worley.distance = Some(
        [
            WorleyDistance::Euclidean,
            WorleyDistance::EuclideanSquared,
            WorleyDistance::Manhattan,
            WorleyDistance::Chebyshev,
        ][rng.random_range(0..4)],
    );
    c.worley.return_type = Some(if rng.random_bool(0.5) {
        WorleyReturn::Distance
    } else {
        WorleyReturn::Value
    });
    c.turbulence.enabled = rng.random_bool(0.3);
    c.turbulence.frequency = rng.random_range(0.5..=4.0);
    c.turbulence.power = rng.random_range(0.2..=3.0);
    c.turbulence.roughness = rng.random_range(1..=6);
    c.stroke_width = rng.random_range(0.0..=8.0f32);
    c.opacity = if rng.random_bool(0.3) {
        rng.random_range(0.1..=0.6)
    } else {
        1.0
    };
    let backgrounds = [Background::White, Background::Black, Background::Solid];
    c.background = Some(backgrounds[rng.random_range(0..backgrounds.len())]);
    c.grain_color = color(&mut rng);
    c.solid_color = color(&mut rng);
    c.grain_amount = rng.random_range(0.1..=1.0);
    c.grain_size = rng.random_range(0.5..=4.0);
    c.dot_controls.dot_style = Some(
        [DotStyle::Circle, DotStyle::Square, DotStyle::Pearl][rng.random_range(0..3)],
    );
    c.dot_controls.pearl_sides = rng.random_range(3..=8);
    c.dot_controls.pearl_smoothness = rng.random_range(0..=5);
    c.dot_controls.stroke = rng.random_bool(0.6);
    c.dot_controls.dot_stroke_color = color(&mut rng);
    for sc in [
        &mut c.dot_controls.size_controls,
        &mut c.extrude_controls.size_controls,
    ] {
        sc.size_fn = Some(size_fns[rng.random_range(0..size_fns.len())]);
        sc.size = rng.random_range(20.0..=250.0f32).round();
        sc.direction = Some(dirs[rng.random_range(0..dirs.len())]);
        sc.size_scale = rng.random_range(1.0..=20.0f32).round();
        sc.min_size = rng.random_range(1.0..=30.0f32).round();
    }
    c.extrude_controls.grad_style = Some(grads[rng.random_range(0..grads.len())]);
    c.extrude_controls.direction = Some(
        [
            ExtrudeDirection::Vertical,
            ExtrudeDirection::Horizontal,
            ExtrudeDirection::Normal,
        ][rng.random_range(0..3)],
    );
    c.color_mode_controls.mode = Some(if rng.random_bool(0.5) {
        ColorMode::Palette
    } else {
        ColorMode::Scale
    });
    c.color_mode_controls.anchor1 = color(&mut rng);
    c.color_mode_controls.anchor2 = color(&mut rng);
    c.color_mode_controls.palette_choice = Some(palettes[rng.random_range(0..palettes.len())]);
    c.color_mode_controls.color_by = Some(color_bys[rng.random_range(0..color_bys.len())]);
    c.color_mode_controls.along_cycles = rng.random_range(1.0..=5.0f32).round();
    c.color_mode_controls.along_mirror = rng.random_bool(0.5);
    c.color_mode_controls.along_phase = rng.random_bool(0.5);
    c.color_mode_controls.region_scale = rng.random_range(0.5..=5.0);
    c.color_mode_controls.region_colors = rng.random_range(2..=8);
    c.color_mode_controls.reverse = rng.random_bool(0.5);
    c.strip_gap = rng.random_range(0.0..=0.3);
    c.column_angle = rng.random_range(0.0..=180.0f32).round();
    c.line_shift = rng.random_range(-35.0..=35.0f32).round();
    c.normalize();
    c
}

impl K2 {
    fn left_panel(&mut self, ui: &mut egui::Ui) {
        use crate::common::CurveDirection::*;
        use crate::CurveStyle::*;
        use Background::*;
        use NoiseFunction::*;
        use Preset::*;

        let d = Controls::default();
        egui::Grid::new("main")
            .spacing((15.0, 10.0))
            .min_col_width(90.0)
            .show(ui, |ui| {
                SliderRow::new(
                    "Width",
                    &mut self.controls.width,
                    d.width,
                    0..=28800,
                )
                .hover(&[
                    "Set the width of the output",
                    "image in pixels. Values under",
                    "180 are treated as inches",
                    "at 300 DPI.",
                ])
                .unclamped()
                .show(ui);
                // Small values are treated as inches at 300 DPI.
                if self.controls.width < 180 {
                    self.controls.width *= 300;
                }
                SliderRow::new(
                    "Height",
                    &mut self.controls.height,
                    d.height,
                    0..=28800,
                )
                .hover(&[
                    "Set the height of the output",
                    "image in pixels. Values under",
                    "180 are treated as inches",
                    "at 300 DPI.",
                ])
                .unclamped()
                .show(ui);
                // Small values are treated as inches at 300 DPI.
                if self.controls.height < 180 {
                    self.controls.height *= 300;
                }

                let mut preset = self.controls.preset;
                if pick_list(
                    ui,
                    "Preset",
                    &[
                        Ribbons, Worms, Solar, Vortex, Canyon, Splat, Tubes, Ducts, RedDwarf,
                    ],
                    &mut preset,
                ) {
                    if let Some(p) = preset {
                        // Keep the chosen noise image across preset loads.
                        let image_noise = self.controls.image_noise.clone();
                        self.controls = load_preset(p);
                        self.controls.image_noise = image_noise;
                        self.pending_draw = true;
                    }
                }
            });

        ui.add_space(SPACE);
        ui.separator();
        ui.add_space(SPACE);

        egui::Grid::new("style")
            .spacing((15.0, 10.0))
            .min_col_width(90.0)
            .show(ui, |ui| {
                pick_list(
                    ui,
                    "Curve Style",
                    &[Line, Dots, Extrusion, Strips],
                    &mut self.controls.curve_style,
                );
                ui.label("Direction");
                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.controls.curve_direction, Some(OneSided), "One");
                    ui.radio_value(&mut self.controls.curve_direction, Some(TwoSided), "Two");
                });
                ui.end_row();
                pick_list(
                    ui,
                    "Flow Field",
                    &[
                        Fbm, BasicMulti, HybridMulti, Billow, Ridged, Value, Cylinders,
                        Worley, Curl, Sinusoidal, Image,
                    ],
                    &mut self.controls.noise_controls.noise_function,
                );
                if self.controls.curve_style == Some(CurveStyle::Strips) {
                    // Strips pair neighboring curves, which only makes sense
                    // from an ordered column of seeds.
                    self.controls.location = Some(Location::Line);
                    ui.label("Locations");
                    ui.add_enabled(
                        false,
                        egui::Button::new("Line").min_size(egui::vec2(150.0, 0.0)),
                    )
                    .on_disabled_hover_ui(|ui| {
                        ui.colored_label(
                            egui::Color32::ORANGE,
                            "Locked to Line while the",
                        );
                        ui.colored_label(egui::Color32::ORANGE, "Strips style is selected.");
                    });
                    ui.end_row();
                } else {
                    pick_list(
                        ui,
                        "Locations",
                        &[
                            Location::Grid,
                            Location::Rand,
                            Location::Halton,
                            Location::Poisson,
                            Location::Circle,
                            Location::Lissajous,
                            Location::Box,
                            Location::Line,
                            Location::Even,
                        ],
                        &mut self.controls.location,
                    );
                }
                if self.controls.location == Some(Location::Line) {
                    SliderRow::new(
                        "Angle",
                        &mut self.controls.column_angle,
                        0.0,
                        0.0..=180.0,
                    )
                    .hover(&[
                        "Rotation of the seed line:",
                        "0 is vertical, 90 horizontal.",
                    ])
                    .steps(5.0, 15.0)
                    .show(ui);
                    SliderRow::new(
                        "Shift",
                        &mut self.controls.line_shift,
                        0.0,
                        -50.0..=50.0,
                    )
                    .hover(&[
                        "Moves the seed line parallel to",
                        "itself; percent of the canvas,",
                        "0 is centered.",
                    ])
                    .steps(1.0, 5.0)
                    .show(ui);
                }
                pick_list(
                    ui,
                    "Background",
                    &[
                        LightGrain, LightFiber, DarkGrain, DarkFiber, ColorGrain, White, Black,
                        Solid,
                    ],
                    &mut self.controls.background,
                );
            });

        ui.add_space(SPACE);
        ui.separator();
        ui.add_space(SPACE);

        egui::Grid::new("curves")
            .spacing((15.0, 10.0))
            .min_col_width(90.0)
            .show(ui, |ui| {
                numeric(
                    ui,
                    "Density",
                    &mut self.controls.density,
                    d.density,
                    5.0..=100.0,
                    5.0,
                    0,
                );
                numeric(
                    ui,
                    "Point Spacing",
                    &mut self.controls.spacing,
                    d.spacing,
                    1.0..=100.0,
                    1.0,
                    0,
                );
                numeric(
                    ui,
                    "Curve Length",
                    &mut self.controls.curve_length,
                    d.curve_length,
                    0..=1000,
                    1.0,
                    0,
                );
                ui.label("Endless").on_hover_ui(|ui| {
                    ui.colored_label(
                        egui::Color32::ORANGE,
                        "Extend both ends of every curve",
                    );
                    ui.colored_label(
                        egui::Color32::ORANGE,
                        "along the flow field until they",
                    );
                    ui.colored_label(
                        egui::Color32::ORANGE,
                        "leave the canvas, so no curve",
                    );
                    ui.colored_label(
                        egui::Color32::ORANGE,
                        "endpoints are visible.",
                    );
                });
                ui.checkbox(&mut self.controls.hide_ends, "");
                ui.end_row();
                numeric(
                    ui,
                    "Noise Scale",
                    &mut self.controls.noise_controls.noise_scale,
                    d.noise_controls.noise_scale,
                    0.1..=20.0,
                    0.1,
                    1,
                );
                numeric(
                    ui,
                    "Noise Factor",
                    &mut self.controls.noise_controls.noise_factor,
                    d.noise_controls.noise_factor,
                    0.1..=10.0,
                    0.1,
                    1,
                );
                SliderRow::new(
                    "Turning Speed",
                    &mut self.controls.speed,
                    d.speed,
                    0.01..=1.0,
                )
                .hover(&[
                    "How quickly a curve turns toward",
                    "the flow field direction. Low",
                    "values give long, smooth strands.",
                ])
                .logarithmic()
                .decimals(2)
                .show(ui);
            });

        self.controls.color_mode_controls.ui(ui);

        ui.add_space(2.0 * SPACE);
        egui::Grid::new("stroke")
            .spacing((15.0, 10.0))
            .min_col_width(90.0)
            .show(ui, |ui| {
                numeric(
                    ui,
                    "Stroke Width",
                    &mut self.controls.stroke_width,
                    d.stroke_width,
                    0.0..=25.0,
                    0.5,
                    1,
                );
                SliderRow::new("Opacity", &mut self.controls.opacity, 1.0, 0.02..=1.0)
                    .hover(&[
                        "Curve opacity; low values let",
                        "overlapping curves build up color.",
                    ])
                    .steps(0.02, 0.1)
                    .decimals(2)
                    .show(ui);
            });

        ui.add_space(SPACE);
        ui.separator();
        ui.add_space(SPACE);
        if action_button(
            ui,
            "Draw",
            true,
            &["Render the artwork with the", "current settings."],
        ) {
            self.pending_draw = true;
        }
        if self.exporting.load(Ordering::Relaxed) || self.rendering {
            ui.add_space(SPACE);
            ui.vertical_centered(|ui| ui.spinner());
        }
    }

    /// Save the current artwork and its parameters on a background thread.
    fn save_png(&mut self) {
        let mut dialog = rfd::FileDialog::new().add_filter("PNG image", &["png"]);
        if let Some(download_dir) = UserDirs::new().and_then(|d| d.download_dir().map(PathBuf::from))
        {
            dialog = dialog
                .set_file_name(next_sketch_name(&download_dir))
                .set_directory(download_dir);
        }
        let Some(path) = dialog.save_file() else {
            return;
        };
        self.exporting.store(true, Ordering::Relaxed);
        let controls = self.controls.clone();
        let flag = self.exporting.clone();
        std::thread::spawn(move || {
            print(controls, path);
            flag.store(false, Ordering::Relaxed);
        });
    }

    fn menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open...").clicked() {
                    let mut dialog = rfd::FileDialog::new().add_filter("K2 params", &["json"]);
                    if let Some(download_dir) =
                        UserDirs::new().and_then(|d| d.download_dir().map(PathBuf::from))
                    {
                        dialog = dialog.set_directory(download_dir);
                    }
                    if let Some(path) = dialog.pick_file() {
                        match std::fs::read_to_string(&path)
                            .map_err(|e| e.to_string())
                            .and_then(|s| {
                                serde_json::from_str::<Controls>(&s).map_err(|e| e.to_string())
                            }) {
                            Ok(controls) => {
                                self.last_drawn = controls.clone();
                                self.controls = controls;
                                self.pending_draw = true;
                            }
                            Err(e) => eprintln!("could not load {}: {e}", path.display()),
                        }
                    }
                }
                let exporting = self.exporting.load(Ordering::Relaxed);
                if ui
                    .add_enabled(!exporting, egui::Button::new("Save PNG"))
                    .clicked()
                {
                    self.save_png();
                }
                if ui.button("Reset").clicked() {
                    let image_noise = self.controls.image_noise.clone();
                    self.controls = ribbons();
                    self.controls.image_noise = image_noise;
                    self.pending_draw = true;
                }
                ui.separator();
                if ui.button("Quit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }

    fn right_panel(&mut self, ui: &mut egui::Ui) {
        if action_button(
            ui,
            "Random",
            true,
            &["Randomize every control except", "the size and preset."],
        ) {
            self.controls = random_controls(self.controls.image_noise.clone());
            self.pending_draw = true;
        }
        ui.add_space(SPACE);
        ui.separator();

        if self.controls.curve_style == Some(CurveStyle::Strips) {
            section(ui, "Strips");
            egui::Grid::new("strips")
                .spacing((15.0, 10.0))
                .min_col_width(90.0)
                .show(ui, |ui| {
                    SliderRow::new("Gap", &mut self.controls.strip_gap, 0.08, 0.0..=0.6)
                        .hover(&[
                            "Fraction of the channel between",
                            "neighboring curves left open.",
                        ])
                        .steps(0.02, 0.1)
                        .decimals(2)
                        .show(ui);
                });
        }
        if self.controls.curve_style == Some(CurveStyle::Extrusion) {
            self.controls.extrude_controls.ui(ui);
        } else if self.controls.curve_style == Some(CurveStyle::Dots) {
            self.controls.dot_controls.ui(ui);
        }
        if matches!(
            self.controls.noise_controls.noise_function,
            Some(NoiseFunction::Fbm)
                | Some(NoiseFunction::BasicMulti)
                | Some(NoiseFunction::HybridMulti)
                | Some(NoiseFunction::Billow)
                | Some(NoiseFunction::Ridged)
                | Some(NoiseFunction::Curl)
        ) {
            self.controls.fractal_controls.ui(ui);
        }
        if self.controls.noise_controls.noise_function == Some(NoiseFunction::Sinusoidal) {
            self.controls.sin_controls.ui(ui);
        }
        if self.controls.noise_controls.noise_function == Some(NoiseFunction::Image) {
            self.controls.image_noise.ui(ui, &mut self.image_thumb);
        }
        if self.controls.noise_controls.noise_function == Some(NoiseFunction::Worley) {
            self.controls.worley.ui(ui);
        }
        self.controls.turbulence.ui(ui);
        if matches!(
            self.controls.background,
            Some(Background::LightGrain) | Some(Background::DarkGrain) | Some(Background::ColorGrain)
        ) {
            section(ui, "Grain");
            egui::Grid::new("grain")
                .spacing((15.0, 10.0))
                .min_col_width(90.0)
                .show(ui, |ui| {
                    if self.controls.background == Some(Background::ColorGrain) {
                        color_picker(ui, "Grain Color", &mut self.controls.grain_color);
                    }
                    numeric(
                        ui,
                        "Amount",
                        &mut self.controls.grain_amount,
                        0.3,
                        0.0..=3.0,
                        0.1,
                        1,
                    );
                    numeric(
                        ui,
                        "Size",
                        &mut self.controls.grain_size,
                        2.0,
                        0.1..=6.0,
                        0.1,
                        1,
                    );
                });
        }
        if self.controls.background == Some(Background::Solid) {
            section(ui, "Background");
            egui::Grid::new("solid_bg")
                .spacing((15.0, 10.0))
                .min_col_width(90.0)
                .show(ui, |ui| {
                    color_picker(ui, "Color", &mut self.controls.solid_color);
                });
        }

    }
}

impl eframe::App for K2 {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.controls);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        egui::Panel::top("menu_bar").show(ui, |ui| {
            self.menu_bar(ui);
        });
        // Panel width: 90px label column + 15px grid spacing + ~185px of
        // slider/value/reset widgets, a 10px scrollbar strip, and 2x10 margins.
        egui::Panel::left("controls")
            .exact_size(330.0)
            .resizable(false)
            .frame(egui::Frame::default().inner_margin(10.0))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.add_space(SPACE);
                        self.left_panel(ui);
                    });
            });
        egui::Panel::right("style_controls")
            .exact_size(330.0)
            .resizable(false)
            .frame(egui::Frame::default().inner_margin(10.0))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.add_space(SPACE);
                        self.right_panel(ui);
                    });
            });
        self.poll_renders(&ctx);
        egui::CentralPanel::default().show(ui, |ui| {
            if let Some(texture) = &self.texture {
                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    // Size by the logical image dimensions, so the layout is
                    // identical while previews and full renders swap in.
                    let avail = ui.available_size();
                    let s = (avail.x / self.image_logical.x)
                        .min(avail.y / self.image_logical.y)
                        .min(1.0);
                    ui.add(egui::Image::new(texture).fit_to_exact_size(self.image_logical * s));
                });
            }
        });

        // Rendering happens only when asked: the Draw button, a preset load,
        // or Reset set `pending_draw`, so any number of controls can be
        // changed before committing. Work runs on a worker thread; a fast
        // preview lands first and the full image follows.
        if self.texture.is_none() && !self.rendering {
            self.start_render(&ctx);
        } else if self.pending_draw {
            self.pending_draw = false;
            // Drawing with unchanged controls is a no-op: the render is
            // fully deterministic, so redrawing would only flash the
            // lower-resolution preview before landing on the same image.
            if self.controls != self.last_drawn || self.rendering {
                // Manual edits clear the preset selection; a preset load keeps it.
                if self.controls.preset == self.last_drawn.preset
                    && self.controls != self.last_drawn
                {
                    self.controls.preset = None;
                }
                self.start_render(&ctx);
            }
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
        let canvas = draw(&controls, 1.0);
        println!(
            "{bg}: {:?} ({}x{})",
            t.elapsed(),
            canvas.width(),
            canvas.height()
        );
    }
}



































































#[test]
fn find_ui_mutation() {
    let ctx = egui::Context::default();
    let mut app = K2::new();
    for roll in 0..30 {
        app.controls = random_controls(Default::default());
        let before = app.controls.clone();
        for _ in 0..3 {
            let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
                app.left_panel(ui);
                app.right_panel(ui);
            });
        }
        if app.controls != before {
            let a = serde_json::to_value(&before).unwrap();
            let b = serde_json::to_value(&app.controls).unwrap();
            for (k, va) in a.as_object().unwrap() {
                let vb = &b[k];
                if va != vb {
                    println!("roll {roll}: field '{k}' changed: {va} -> {vb}");
                }
            }
        }
    }
    println!("scan done");
}

