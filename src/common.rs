#![allow(dead_code)]

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::art::draw;
use crate::background::Background;
use crate::color::ColorControls;
use crate::dot::DotControls;
use crate::extrude::ExtrudeControls;
use crate::fractal::FractalControls;
use crate::noise::NoiseControls;
use crate::presets::Preset;
use crate::sine::SineControls;

use crate::{location::Location, presets::ribbons};
use eframe::egui;
use serde::{Deserialize, Serialize};

pub const WIDTH: u32 = 1000;
pub const HEIGHT: u32 = 1000;
pub const SEED: u64 = 98713;

pub struct K2 {
    pub controls: Controls,
    /// The controls as of the last texture regeneration; used to detect edits.
    pub last_drawn: Controls,
    pub texture: Option<egui::TextureHandle>,
    pub exporting: Arc<AtomicBool>,
}

impl K2 {
    pub fn new() -> Self {
        let controls = ribbons();
        Self {
            last_drawn: controls.clone(),
            controls,
            texture: None,
            exporting: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Regenerate the artwork and upload it as an egui texture.
    pub fn regenerate(&mut self, ctx: &egui::Context) {
        let canvas = draw(&self.controls, false);
        let image = egui::ColorImage::from_rgba_premultiplied(
            [
                canvas.pixmap.width() as usize,
                canvas.pixmap.height() as usize,
            ],
            canvas.pixmap.data(),
        );
        match &mut self.texture {
            Some(texture) => texture.set(image, egui::TextureOptions::LINEAR),
            None => {
                self.texture = Some(ctx.load_texture("art", image, egui::TextureOptions::LINEAR))
            }
        }
        self.last_drawn = self.controls.clone();
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Controls {
    pub preset: Option<Preset>,
    pub curve_style: Option<CurveStyle>,
    pub curve_direction: Option<CurveDirection>,
    pub spacing: f32,
    pub curve_length: u32,
    pub hide_ends: bool,
    pub grain_color: egui::Color32,
    pub solid_color: egui::Color32,
    pub location: Option<Location>,
    pub density: f32,
    pub noise_controls: NoiseControls,
    pub fractal_controls: FractalControls,
    pub speed: f32,
    pub stroke_width: f32,
    pub background: Option<Background>,
    pub width: u32,
    pub height: u32,
    pub sin_controls: SineControls,
    pub dot_controls: DotControls,
    pub extrude_controls: ExtrudeControls,
    pub color_mode_controls: ColorControls,
}

impl Controls {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            preset: Some(Preset::Ribbons),
            curve_style: Some(CurveStyle::Dots),
            spacing: 4.0,
            curve_length: 50,
            hide_ends: false,
            curve_direction: Some(CurveDirection::OneSided),
            grain_color: egui::Color32::from_rgb(128, 128, 128),
            solid_color: egui::Color32::from_rgb(245, 242, 235),
            location: Some(Location::Halton),
            noise_controls: NoiseControls::default(),
            density: 50.0,
            fractal_controls: FractalControls::default(),
            speed: 1.0,
            stroke_width: 1.0,
            background: Some(Background::LightFiber),
            width: 1080,
            height: 1080,
            sin_controls: SineControls::default(),
            dot_controls: DotControls::default(),
            extrude_controls: ExtrudeControls::default(),
            color_mode_controls: ColorControls::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurveStyle {
    Line,
    Dots,
    Extrusion,
}

impl std::fmt::Display for CurveStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CurveStyle::Line => "Line",
                CurveStyle::Dots => "Dots",
                CurveStyle::Extrusion => "Extrusion",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurveDirection {
    OneSided,
    TwoSided,
}

impl std::fmt::Display for CurveDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CurveDirection::OneSided => "One Sided",
                CurveDirection::TwoSided => "Two Sided",
            }
        )
    }
}
