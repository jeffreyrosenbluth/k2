#![allow(dead_code)]

use crate::art::draw;
use crate::background::Background;
use crate::gradient::GradStyle;
use crate::location::Location;
use crate::noise::NoiseFunction;
use crate::size::{Dir, SizeFn};
use iced::widget::image;
use iced::Color;

pub const WIDTH: u32 = 1000;
pub const HEIGHT: u32 = 1000;
pub const SEED: u64 = 98713;

#[derive(Clone)]
pub struct Xtrusion {
    pub controls: Controls,
    pub image: image::Handle,
    pub width: u16,
    pub height: u16,
}

impl Xtrusion {
    pub fn new() -> Self {
        let controls = Controls::new();
        let canvas = draw(&controls, false);
        Self {
            controls,
            image: image::Handle::from_pixels(canvas.width, canvas.height, canvas.pixmap.take()),
            width: canvas.width as u16,
            height: canvas.height as u16,
        }
    }

    pub fn draw(&mut self) {
        let canvas = draw(&self.controls, false);
        self.width = canvas.width() as u16;
        self.height = canvas.height() as u16;
        self.image = image::Handle::from_pixels(
            canvas.pixmap.width(),
            canvas.pixmap.height(),
            canvas.pixmap.take(),
        );
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Preset {
    Slinky,
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
                Preset::Slinky => "Slinky",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DotStyle {
    Circle,
    Square,
    Pearl,
}

impl std::fmt::Display for DotStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DotStyle::Circle => "Circle",
                DotStyle::Square => "Square",
                DotStyle::Pearl => "Pearl",
            }
        )
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
    pub show_color_picker3: bool,
    pub location: Option<Location>,
    pub density: f32,
    pub noise_controls: NoiseControls,
    pub fractal_controls: FractalControls,
    pub speed: f32,
    pub size_controls: SizeControls,
    pub grad_style: Option<GradStyle>,
    pub dot_style: Option<DotStyle>,
    pub pearl_sides: u32,
    pub pearl_smoothness: u32,
    pub exporting: bool,
    pub stroke_width: f32,
    pub dot_stroke_color: Color,
    pub background: Option<Background>,
    pub width: String,
    pub height: String,
    pub border: bool,
    pub sin_xfreq: f32,
    pub sin_yfreq: f32,
    pub sin_xexp: f32,
    pub sin_yexp: f32,
}

impl Controls {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            preset: Some(Preset::Slinky),
            curve_style: Some(CurveStyle::Dots),
            spacing: 4.0,
            curve_length: 50,
            color1: Color::from_rgb8(20, 134, 187),
            color2: Color::from_rgb8(0, 0, 0),
            show_color_picker1: false,
            show_color_picker2: false,
            show_color_picker3: false,
            location: Some(Location::Halton),
            noise_controls: NoiseControls::default(),
            density: 50.0,
            fractal_controls: FractalControls::default(),
            speed: 1.0,
            size_controls: SizeControls::default(),
            grad_style: Some(GradStyle::None),
            dot_style: Some(DotStyle::Circle),
            pearl_sides: 4,
            pearl_smoothness: 3,
            exporting: false,
            stroke_width: 1.0,
            dot_stroke_color: Color::from_rgb8(255, 255, 255),
            background: Some(Background::Clouds),
            width: String::new(),
            height: String::new(),
            border: true,
            sin_xfreq: 1.0,
            sin_yfreq: 1.0,
            sin_xexp: 2.0,
            sin_yexp: 2.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FractalControls {
    pub octaves: u8,
    pub persistence: f32,
    pub lacunarity: f32,
    pub frequency: f32,
}

impl Default for FractalControls {
    fn default() -> Self {
        Self {
            octaves: 4,
            persistence: 0.5,
            lacunarity: 2.094395,
            frequency: 1.0,
        }
    }
}

impl FractalControls {
    pub fn new(octaves: u8, persistence: f32, lacunarity: f32, frequency: f32) -> Self {
        Self {
            octaves,
            persistence,
            lacunarity,
            frequency,
        }
    }

    pub fn set_octaves(mut self, octaves: u8) -> Self {
        self.octaves = octaves;
        self
    }

    pub fn set_persistence(mut self, persistence: f32) -> Self {
        self.persistence = persistence;
        self
    }

    pub fn set_lacunarity(mut self, lacunarity: f32) -> Self {
        self.lacunarity = lacunarity;
        self
    }

    pub fn set_frequency(mut self, frequency: f32) -> Self {
        self.frequency = frequency;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SizeControls {
    pub size_fn: Option<SizeFn>,
    pub size: f32,
    pub direction: Option<Dir>,
    pub size_scale: f32,
    pub min_size: f32,
}

impl Default for SizeControls {
    fn default() -> Self {
        Self {
            size_fn: Some(SizeFn::Contracting),
            size: 100.0,
            direction: Some(Dir::Both),
            size_scale: 10.0,
            min_size: 25.0,
        }
    }
}

impl SizeControls {
    pub fn new(size_fn: SizeFn, size: f32, direction: Dir, size_scale: f32, min_size: f32) -> Self {
        Self {
            size_fn: Some(size_fn),
            size,
            direction: Some(direction),
            size_scale,
            min_size,
        }
    }
    pub fn set_size_fn(mut self, size_fn: Option<SizeFn>) -> Self {
        self.size_fn = size_fn;
        self
    }

    pub fn set_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn set_direction(mut self, direction: Option<Dir>) -> Self {
        self.direction = direction;
        self
    }

    pub fn set_size_scale(mut self, size_scale: f32) -> Self {
        self.size_scale = size_scale;
        self
    }

    pub fn set_min_size(mut self, min_size: f32) -> Self {
        self.min_size = min_size;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoiseControls {
    pub noise_function: Option<NoiseFunction>,
    pub noise_factor: f32,
    pub noise_scale: f32,
}

impl NoiseControls {
    pub fn new(noise_function: NoiseFunction, noise_scale: f32, noise_factor: f32) -> Self {
        Self {
            noise_function: Some(noise_function),
            noise_factor,
            noise_scale,
        }
    }

    pub fn set_noise_function(mut self, noise_function: NoiseFunction) -> Self {
        self.noise_function = Some(noise_function);
        self
    }

    pub fn set_noise_factor(mut self, noise_factor: f32) -> Self {
        self.noise_factor = noise_factor;
        self
    }

    pub fn set_noise_scale(mut self, noise_scale: f32) -> Self {
        self.noise_scale = noise_scale;
        self
    }
}

impl Default for NoiseControls {
    fn default() -> Self {
        Self {
            noise_function: Some(NoiseFunction::Fbm),
            noise_factor: 1.0,
            noise_scale: 4.0,
        }
    }
}
