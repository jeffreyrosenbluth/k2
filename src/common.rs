#![allow(dead_code)]

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
use iced::widget::image;
use iced::Color;

pub const WIDTH: u32 = 1000;
pub const HEIGHT: u32 = 1000;
pub const SEED: u64 = 98713;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresetState {
    Set,
    NotSet,
}

#[derive(Clone)]
pub struct K2 {
    pub controls: Controls,
    pub image: image::Handle,
    pub width: u16,
    pub height: u16,
}

impl K2 {
    pub fn new() -> Self {
        let controls = ribbons();
        let canvas = draw(&controls, false);
        let w = canvas.width();
        let h = canvas.height();
        Self {
            controls,
            image: image::Handle::from_pixels(w, h, canvas.pixmap.take()),
            width: w as u16,
            height: h as u16,
        }
    }

    pub fn draw(&mut self, preset_state: PresetState) {
        let canvas = draw(&self.controls, false);
        self.width = canvas.width() as u16;
        self.height = canvas.height() as u16;
        self.image = image::Handle::from_pixels(
            canvas.pixmap.width(),
            canvas.pixmap.height(),
            canvas.pixmap.take(),
        );
        if preset_state == PresetState::NotSet {
            self.controls.preset = None;
        }
    }
}

#[derive(Clone)]
pub struct Controls {
    pub preset: Option<Preset>,
    pub curve_style: Option<CurveStyle>,
    pub curve_direction: Option<CurveDirection>,
    pub spacing: f32,
    pub curve_length: u32,
    pub grain_color: Color,
    pub show_grain_color_picker: bool,
    pub location: Option<Location>,
    pub density: f32,
    pub noise_controls: NoiseControls,
    pub fractal_controls: FractalControls,
    pub speed: f32,
    pub exporting: bool,
    pub stroke_width: f32,
    pub background: Option<Background>,
    pub width: String,
    pub height: String,
    pub border: bool,
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
            curve_direction: Some(CurveDirection::OneSided),
            grain_color: Color::from_rgb8(128, 128, 128),
            show_grain_color_picker: false,
            location: Some(Location::Halton),
            noise_controls: NoiseControls::default(),
            density: 50.0,
            fractal_controls: FractalControls::default(),
            speed: 1.0,
            exporting: false,
            stroke_width: 1.0,
            background: Some(Background::LightFiber),
            width: String::new(),
            height: String::new(),
            border: true,
            sin_controls: SineControls::default(),
            dot_controls: DotControls::default(),
            extrude_controls: ExtrudeControls::default(),
            color_mode_controls: ColorControls::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CurveDirection {
    OneSided,
    TwoSided,
}

impl From<CurveDirection> for String {
    fn from(direction: CurveDirection) -> Self {
        match direction {
            CurveDirection::OneSided => "One Sided".to_string(),
            CurveDirection::TwoSided => "Two Sided".to_string(),
        }
    }
}
