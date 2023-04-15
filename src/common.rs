#![allow(dead_code)]

use crate::art::draw;
use crate::background::Background;
use crate::dot::DotControls;
use crate::extrude::ExtrudeControls;
use crate::fractal::FractalControls;
use crate::noise::NoiseControls;
use crate::sine::SineControls;

use crate::{location::Location, presets::rusty_ribbons};
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
        let controls = rusty_ribbons();
        let canvas = draw(&controls, false);
        Self {
            controls,
            image: image::Handle::from_pixels(canvas.width, canvas.height, canvas.pixmap.take()),
            width: canvas.width as u16,
            height: canvas.height as u16,
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
    pub spacing: f32,
    pub curve_length: u32,
    pub color1: Color,
    pub color2: Color,
    pub show_color_picker1: bool,
    pub show_color_picker2: bool,
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
}

impl Controls {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            preset: Some(Preset::RustyRibbons),
            curve_style: Some(CurveStyle::Dots),
            spacing: 4.0,
            curve_length: 50,
            color1: Color::from_rgb8(20, 134, 187),
            color2: Color::from_rgb8(0, 0, 0),
            show_color_picker1: false,
            show_color_picker2: false,
            location: Some(Location::Halton),
            noise_controls: NoiseControls::default(),
            density: 50.0,
            fractal_controls: FractalControls::default(),
            speed: 1.0,
            exporting: false,
            stroke_width: 1.0,
            background: Some(Background::LightClouds),
            width: String::new(),
            height: String::new(),
            border: true,
            sin_controls: SineControls::default(),
            dot_controls: DotControls::default(),
            extrude_controls: ExtrudeControls::default(),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Preset {
    RustyRibbons,
    Solar,
    RiverStones,
    Purple,
    Canyon,
    Stripes,
    Splat,
    Tubes,
    Ducts,
    Ridges,
}

impl std::fmt::Display for Preset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Preset::RustyRibbons => "Rusty Ribbons",
                Preset::Solar => "Solar",
                Preset::RiverStones => "River Stones",
                Preset::Purple => "Purple",
                Preset::Canyon => "Canyon",
                Preset::Stripes => "Stripes",
                Preset::Splat => "Splat",
                Preset::Tubes => "Tubes",
                Preset::Ducts => "Ducts",
                Preset::Ridges => "Ridges",
            }
        )
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
