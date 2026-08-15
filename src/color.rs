#![allow(dead_code)]

use crate::gui::{color_picker, pick_list};
use eframe::egui;
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

fn gray_values() -> Vec<Color> {
    let gs = vec![239, 223, 202, 168, 135, 109, 95, 74, 61, 28];
    gs.iter().map(|g| grays(*g)).collect()
}

pub fn color_palette(pal: Palettes) -> Palette {
    use Palettes::*;
    match pal {
        Royalty => make_palette(vec![0x1C4572, 0x84561B, 0x6D3E32, 0x0A0E20]),
        DeltaBlues => make_palette(vec![0x003566, 0x000000, 0x008080]),
        PinotNoir => make_palette(vec![0x701C1C, 0x1A1717, 0x77806E]),
        Algae => make_palette(vec![0xA3B18A, 0x588157, 0x3A5A40, 0x344E41]),
        Scepter => make_palette(vec![0xB7A635, 0x4E1406]),
        Fire => make_palette(vec![0x621708, 0x941B0C, 0xBC3908, 0xF6AA1C]),
        Perfume => make_palette(vec![0xD9798B, 0x8C4962, 0x59364A, 0x594832]),
        Rose => make_palette(vec![0xBF2642, 0x731F2E, 0x400C16]),
        GrayScale => Palette::new(gray_values()),
        PorcoRosso => make_palette(vec![0x002B75, 0x862A23, 0xBD8878]),
        SpiritedAway => make_palette(vec![0xD9A404, 0xF2B988, 0xBF3030, 0x0D0D0D]),
        Totoro => make_palette(vec![0x6A7AB2, 0xF27E9D, 0x454259, 0x9B8660]),
        MonoBlue => {
            let mut cs = gray_values();
            cs.push(*ROYALBLUE);
            Palette::new(cs)
        }
        MonoRed => {
            let mut cs = gray_values();
            cs.push(*BROWN);
            Palette::new(cs)
        }
        MonoGreen => {
            let mut cs = gray_values();
            cs.push(*MEDIUMSEAGREEN);
            Palette::new(cs)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorMode {
    Palette,
    Scale,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorControls {
    pub mode: Option<ColorMode>,
    pub anchor1: egui::Color32,
    pub anchor2: egui::Color32,
    pub palette_choice: Option<Palettes>,
}

impl Default for ColorControls {
    fn default() -> Self {
        Self {
            mode: Some(ColorMode::Scale),
            anchor1: egui::Color32::from_rgb(20, 134, 187),
            anchor2: egui::Color32::from_rgb(0, 0, 0),
            palette_choice: Some(Palettes::Royalty),
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
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.mode, Some(ColorMode::Palette), "Palette");
            ui.radio_value(&mut self.mode, Some(ColorMode::Scale), "Color Scale");
        });
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
    }
}
