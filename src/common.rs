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
    pub location: Option<Location>,
    pub density: f32,
    pub noise_factor: f32,
    pub noise_scale: f32,
    pub octaves: u8,
    pub persistence: f32,
    pub lacunarity: f32,
    pub frequency: f32,
    pub noise_function: Option<NoiseFunction>,
    pub speed: f32,
    pub size_fn: Option<SizeFn>,
    pub size: f32,
    pub direction: Option<Dir>,
    pub size_scale: f32,
    pub min_size: f32,
    pub grad_style: Option<GradStyle>,
    pub dot_style: Option<DotStyle>,
    pub pearl_sides: u32,
    pub pearl_smoothness: u32,
    pub exporting: bool,
    pub worley_dist: bool,
    pub stroke_width: f32,
    pub background: Option<Background>,
    pub width: String,
    pub height: String,
    pub border: bool,
}

impl Controls {
    pub fn new() -> Self {
        Self {
            preset: Some(Preset::Slinky),
            curve_style: Some(CurveStyle::Dots),
            spacing: 4.0,
            curve_length: 50,
            color1: Color::from_rgb8(20, 134, 187),
            color2: Color::from_rgb8(0, 0, 0),
            show_color_picker1: false,
            show_color_picker2: false,
            location: Some(Location::Halton),
            density: 50.0,
            noise_factor: 1.0,
            noise_scale: 4.0,
            octaves: 4,
            persistence: 0.5,
            lacunarity: 2.094395,
            frequency: 1.0,
            noise_function: Some(NoiseFunction::Fbm),
            speed: 1.0,
            size_fn: Some(SizeFn::Contracting),
            size: 100.0,
            direction: Some(Dir::Both),
            size_scale: 10.0,
            min_size: 25.0,
            grad_style: Some(GradStyle::None),
            dot_style: Some(DotStyle::Circle),
            pearl_sides: 4,
            pearl_smoothness: 3,
            exporting: false,
            worley_dist: false,
            stroke_width: 1.0,
            background: Some(Background::Clouds),
            width: String::new(),
            height: String::new(),
            border: true,
        }
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self::new()
    }
}
