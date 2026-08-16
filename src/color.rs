#![allow(dead_code)]

use crate::gui::{color_picker, pick_list, section, SliderRow};
use eframe::egui;
use serde::{Deserialize, Serialize};
use wassily::prelude::palette::{Darken, Desaturate, OklabHue, Saturate};
use wassily::prelude::*;

pub fn color_scale(color1: Color, color2: Color, n: u8) -> Vec<Color> {
    let c1 = Okhsl::from_color(&color1);
    let c2 = Okhsl::from_color(&color2);
    let hsl1 = c1.desaturate(0.5).lighten(0.5);
    let hsl2 = c2.saturate(0.5).darken(0.5);
    (0..n)
        .map(|p| {
            let t = p as f32 * 1.0 / (n - 1) as f32;
            let h =
                (1.0 - t) * hsl1.hue.into_positive_radians() + t * hsl2.hue.into_positive_radians();
            let s = (1.0 - t) * hsl1.saturation + t * hsl2.saturation;
            let l = (1.0 - t) * hsl1.lightness + t * hsl2.lightness;
            Okhsl::new(OklabHue::from_radians(h), s, l).to_color()
        })
        .collect()
}

pub fn expand_palette(palette: Vec<Color>) -> Vec<Color> {
    let mut result = palette.clone();
    let n = palette.len();
    for i in 0..n {
        for j in i..n {
            let c = result[i].lerp(&result[j], 0.5);
            result.push(c);
        }
    }
    result
}

fn hex_to_color(hex: Vec<u32>) -> Vec<Color> {
    hex.iter()
        .map(|h| {
            let (r, g, b) = Srgb::from(*h).into_components();
            Color::from_rgba8(r, g, b, 255)
        })
        .collect::<Vec<Color>>()
}

fn make_palette(hex: Vec<u32>) -> Palette {
    let raw_palette = hex_to_color(hex);
    Palette::new(expand_palette(raw_palette))
}

const GRAYS: [u8; 8] = [202, 168, 135, 109, 95, 74, 61, 28];

fn gray_values() -> Vec<Color> {
    GRAYS.iter().map(|g| grays(*g)).collect()
}

impl Palettes {
    /// The base hex colors each named palette is built from (empty for the
    /// grayscale and mono palettes, which are derived from the gray ramp).
    pub fn base_hex(self) -> &'static [u32] {
        use Palettes::*;
        match self {
            Royalty => &[0x1C4572, 0x84561B, 0x6D3E32, 0x0A0E20],
            DeltaBlues => &[0x003566, 0x000000, 0x008080],
            PinotNoir => &[0x701C1C, 0x1A1717, 0x77806E],
            Algae => &[0xA3B18A, 0x588157, 0x3A5A40, 0x344E41],
            Scepter => &[0xB7A635, 0x4E1406],
            Fire => &[0x621708, 0x941B0C, 0xBC3908, 0xF6AA1C],
            Perfume => &[0xD9798B, 0x8C4962, 0x59364A, 0x594832],
            Rose => &[0xBF2642, 0x731F2E, 0x400C16],
            PorcoRosso => &[0x002B75, 0x862A23, 0xBD8878],
            SpiritedAway => &[0xD9A404, 0xF2B988, 0xBF3030, 0x0D0D0D],
            Totoro => &[0x6A7AB2, 0xF27E9D, 0x454259, 0x9B8660],
            GrayScale | MonoRed | MonoGreen | MonoBlue => &[],
        }
    }
}

/// The swatch colors offered by the color picker popup: a row of neutrals
/// plus the two leading base colors of each named palette, kept small enough
/// that 8-wide swatch rows stay within the picker's width.
pub fn swatches() -> Vec<egui::Color32> {
    use Palettes::*;
    let mut out = vec![egui::Color32::BLACK, egui::Color32::WHITE];
    out.extend([239u8, 202, 135, 95, 61, 28].map(egui::Color32::from_gray));
    for pal in [
        Royalty,
        DeltaBlues,
        PinotNoir,
        Algae,
        Scepter,
        Fire,
        Perfume,
        Rose,
        PorcoRosso,
        SpiritedAway,
        Totoro,
    ] {
        for h in pal.base_hex().iter().take(2) {
            let c = egui::Color32::from_rgb((h >> 16) as u8, (h >> 8) as u8, *h as u8);
            if !out.contains(&c) {
                out.push(c);
            }
        }
    }
    out
}

/// The full (expanded) color list a palette choice provides.
pub fn palette_colors(pal: Palettes) -> Vec<Color> {
    use Palettes::*;
    match pal {
        GrayScale => gray_values(),
        Royalty | DeltaBlues | PinotNoir | Algae | Scepter | Fire | Perfume | Rose | PorcoRosso
        | SpiritedAway | Totoro => expand_palette(hex_to_color(pal.base_hex().to_vec())),
        MonoBlue => {
            let mut cs = gray_values();
            cs.push(*ROYALBLUE);
            cs
        }
        MonoRed => {
            let mut cs = gray_values();
            cs.push(*BROWN);
            cs
        }
        MonoGreen => {
            let mut cs = gray_values();
            cs.push(*MEDIUMSEAGREEN);
            cs
        }
    }
}

pub fn color_palette(pal: Palettes) -> Palette {
    Palette::new(palette_colors(pal))
}

/// Interpolate a color list at t in [0, 1].
pub fn sample_colors(colors: &[Color], t: f32) -> Color {
    match colors.len() {
        0 => *WHITE,
        n => {
            let x = t.clamp(0.0, 1.0) * (n - 1) as f32;
            let i = x.floor() as usize;
            let j = (i + 1).min(n - 1);
            colors[i].lerp(&colors[j], x - i as f32)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Palettes {
    Royalty,
    DeltaBlues,
    PinotNoir,
    Algae,
    Scepter,
    Fire,
    Perfume,
    Rose,
    GrayScale,
    PorcoRosso,
    SpiritedAway,
    Totoro,
    MonoRed,
    MonoGreen,
    MonoBlue,
}

impl std::fmt::Display for Palettes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Palettes::Royalty => write!(f, "Royalty"),
            Palettes::DeltaBlues => write!(f, "Delta Blues"),
            Palettes::PinotNoir => write!(f, "Pinot Noir"),
            Palettes::Algae => write!(f, "Algae"),
            Palettes::Scepter => write!(f, "Scepter"),
            Palettes::Fire => write!(f, "Fire"),
            Palettes::Perfume => write!(f, "Perfume"),
            Palettes::Rose => write!(f, "Rose"),
            Palettes::GrayScale => write!(f, "Gray Scale"),
            Palettes::PorcoRosso => write!(f, "Porco Rosso"),
            Palettes::SpiritedAway => write!(f, "Spirited Away"),
            Palettes::Totoro => write!(f, "Totoro"),
            Palettes::MonoBlue => write!(f, "Mono Blue"),
            Palettes::MonoRed => write!(f, "Mono Red"),
            Palettes::MonoGreen => write!(f, "Mono Green"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorMode {
    Palette,
    Scale,
}

/// How each curve's color is chosen from the palette or scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorBy {
    Random,
    Cycle,
    Order,
    PositionX,
    PositionY,
    Radial,
    FlowAngle,
    NoiseValue,
    Region,
    AlongCurve,
}

impl std::fmt::Display for ColorBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ColorBy::Random => "Random",
                ColorBy::Cycle => "Cycle",
                ColorBy::Order => "Draw Order",
                ColorBy::PositionX => "Position X",
                ColorBy::PositionY => "Position Y",
                ColorBy::Radial => "Radial",
                ColorBy::FlowAngle => "Flow Angle",
                ColorBy::NoiseValue => "Noise Value",
                ColorBy::Region => "Region",
                ColorBy::AlongCurve => "Along Curve",
            }
        )
    }
}

fn default_color_by() -> Option<ColorBy> {
    Some(ColorBy::Random)
}

fn default_along_cycles() -> f32 {
    1.0
}

fn default_region_scale() -> f32 {
    1.5
}

fn default_region_colors() -> u32 {
    6
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ColorControls {
    pub mode: Option<ColorMode>,
    pub anchor1: egui::Color32,
    pub anchor2: egui::Color32,
    pub palette_choice: Option<Palettes>,
    #[serde(default = "default_color_by")]
    pub color_by: Option<ColorBy>,
    /// Along Curve: how many palette sweeps fit in one curve.
    #[serde(default = "default_along_cycles")]
    pub along_cycles: f32,
    /// Along Curve: sweep back and forth instead of jumping back to the start.
    #[serde(default)]
    pub along_mirror: bool,
    /// Along Curve: start each curve at a random point in the cycle.
    #[serde(default)]
    pub along_phase: bool,
    /// Region: the noise scale of the color patches; low is large patches.
    #[serde(default = "default_region_scale")]
    pub region_scale: f32,
    /// Region: how many distinct colors the patches quantize to.
    #[serde(default = "default_region_colors")]
    pub region_colors: u32,
    /// Position and Radial: run the palette in the opposite direction.
    #[serde(default)]
    pub reverse: bool,
}

impl Default for ColorControls {
    fn default() -> Self {
        Self {
            mode: Some(ColorMode::Scale),
            anchor1: egui::Color32::from_rgb(20, 134, 187),
            anchor2: egui::Color32::from_rgb(0, 0, 0),
            palette_choice: Some(Palettes::Royalty),
            color_by: Some(ColorBy::Random),
            along_cycles: 1.0,
            along_mirror: false,
            along_phase: false,
            region_scale: 1.5,
            region_colors: 6,
            reverse: false,
        }
    }
}

impl ColorControls {
    pub fn set_anchor1(mut self, color: egui::Color32) -> Self {
        self.anchor1 = color;
        self
    }

    pub fn set_anchor2(mut self, color: egui::Color32) -> Self {
        self.anchor2 = color;
        self
    }

    pub fn set_mode(mut self, mode: ColorMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn set_palette_choice(mut self, pal: Palettes) -> Self {
        self.palette_choice = Some(pal);
        self
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        use Palettes::*;
        section(ui, "Color");
        egui::Grid::new("color_mode")
            .spacing((15.0, 10.0))
            .min_col_width(90.0)
            .show(ui, |ui| {
                ui.label("Color Mode");
                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.mode, Some(ColorMode::Palette), "Palette");
                    ui.radio_value(&mut self.mode, Some(ColorMode::Scale), "Scale");
                });
                ui.end_row();
                pick_list(
                    ui,
                    "Color By",
                    &[
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
                    ],
                    &mut self.color_by,
                );
                if self.color_by == Some(ColorBy::AlongCurve) {
                    SliderRow::new("Cycles", &mut self.along_cycles, 1.0, 0.5..=10.0)
                        .hover(&["Palette sweeps along each curve."])
                        .steps(0.5, 1.0)
                        .decimals(1)
                        .show(ui);
                    ui.label("Mirror").on_hover_ui(|ui| {
                        ui.colored_label(
                            egui::Color32::ORANGE,
                            "Sweep back and forth instead of",
                        );
                        ui.colored_label(
                            egui::Color32::ORANGE,
                            "jumping back to the start color.",
                        );
                    });
                    ui.checkbox(&mut self.along_mirror, "");
                    ui.end_row();
                    ui.label("Random Phase").on_hover_ui(|ui| {
                        ui.colored_label(
                            egui::Color32::ORANGE,
                            "Start each curve at a random",
                        );
                        ui.colored_label(egui::Color32::ORANGE, "point in the cycle.");
                    });
                    ui.checkbox(&mut self.along_phase, "");
                    ui.end_row();
                }
                if matches!(
                    self.color_by,
                    Some(ColorBy::PositionX) | Some(ColorBy::PositionY) | Some(ColorBy::Radial)
                ) {
                    ui.label("Reverse").on_hover_ui(|ui| {
                        ui.colored_label(
                            egui::Color32::ORANGE,
                            "Run the palette in the",
                        );
                        ui.colored_label(egui::Color32::ORANGE, "opposite direction.");
                    });
                    ui.checkbox(&mut self.reverse, "");
                    ui.end_row();
                }
                if self.color_by == Some(ColorBy::Region) {
                    SliderRow::new("Region Scale", &mut self.region_scale, 1.5, 0.5..=10.0)
                        .hover(&["Patch size; low values give", "a few large patches."])
                        .steps(0.5, 1.0)
                        .decimals(1)
                        .show(ui);
                    SliderRow::new("Region Colors", &mut self.region_colors, 6, 2..=12)
                        .hover(&["Distinct colors the patches", "quantize to."])
                        .steps(1.0, 2.0)
                        .show(ui);
                }
                if self.mode == Some(ColorMode::Scale) {
                    color_picker(ui, "Anchor 1", &mut self.anchor1);
                    color_picker(ui, "Anchor 2", &mut self.anchor2);
                } else {
                    pick_list(
                        ui,
                        "Palette",
                        &[
                            Royalty,
                            DeltaBlues,
                            PinotNoir,
                            Algae,
                            Scepter,
                            Fire,
                            Perfume,
                            Rose,
                            GrayScale,
                            PorcoRosso,
                            SpiritedAway,
                            Totoro,
                            MonoRed,
                            MonoGreen,
                            MonoBlue,
                        ],
                        &mut self.palette_choice,
                    );
                }
            });
    }
}
