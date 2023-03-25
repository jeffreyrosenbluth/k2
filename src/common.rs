use crate::art::draw;
use crate::background::Background;
use crate::gradient::GradStyle;
use crate::length::{Dir, ExtrusionStyle};
use crate::location::Location;
use crate::noise::NoiseFunction;
use iced::widget::image;
use iced::Color;
use rand::distributions::Standard;
use rand::prelude::*;

pub const WIDTH: u32 = 1000;
pub const HEIGHT: u32 = 1000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CurveStyle {
    Line,
    Dots,
    Extrusion,
}

impl Distribution<CurveStyle> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CurveStyle {
        let index: u8 = rng.gen_range(0..3);
        match index {
            0 => CurveStyle::Line,
            1 => CurveStyle::Dots,
            2 => CurveStyle::Extrusion,
            _ => unreachable!(),
        }
    }
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

#[derive(Clone)]
pub struct Controls {
    pub hi_res: bool,
    pub curve_style: Option<CurveStyle>,
    pub spacing: f32,
    pub curve_length: u32,
    pub color1: Color,
    pub color2: Color,
    pub show_color_picker1: bool,
    pub show_color_picker2: bool,
    pub location: Option<Location>,
    pub grid_sep: f32,
    pub noise_factor: f32,
    pub noise_scale: f32,
    pub octaves: u8,
    pub persistence: f32,
    pub noise_function: Option<NoiseFunction>,
    pub speed: f32,
    pub len_type: Option<ExtrusionStyle>,
    pub len_size: f32,
    pub len_dir: Option<Dir>,
    pub grad_style: Option<GradStyle>,
    pub exporting: bool,
    pub worley_dist: bool,
    pub stroke_width: f32,
    pub background: Option<Background>,
    pub export_width: String,
    pub export_height: String,
}

impl Controls {
    pub fn new() -> Self {
        Self {
            hi_res: false,
            curve_style: Some(CurveStyle::Dots),
            spacing: 4.0,
            curve_length: 50,
            color1: Color::from_rgb8(20, 134, 187),
            color2: Color::from_rgb8(0, 0, 0),
            show_color_picker1: false,
            show_color_picker2: false,
            location: Some(Location::Halton),
            grid_sep: 50.0,
            noise_factor: 1.0,
            noise_scale: 4.0,
            octaves: 4,
            persistence: 0.5,
            noise_function: Some(NoiseFunction::Fbm),
            speed: 1.0,
            len_type: Some(ExtrusionStyle::Contracting),
            len_size: 200.0,
            len_dir: Some(Dir::Both),
            grad_style: Some(GradStyle::None),
            exporting: false,
            worley_dist: false,
            stroke_width: 8.0,
            background: Some(Background::Clouds),
            export_width: String::new(),
            export_height: String::new(),
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = SmallRng::from_entropy();
        let mut rand_controls: Controls = rng.gen();
        rand_controls.hi_res = self.hi_res;
        rand_controls.stroke_width = self.stroke_width;
        // rand_controls.spacing = self.spacing;
        rand_controls.curve_length = self.curve_length;
        rand_controls.grid_sep = self.grid_sep;
        *self = rand_controls;
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self::new()
    }
}

impl Distribution<Controls> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Controls {
        let location: Option<Location> = Some(rng.gen());
        let grid_sep = rng.gen_range(25.0..75.0);
        let noise_function: Option<NoiseFunction> = Some(rng.gen());
        let color1 = Color::from_rgb8(
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        );
        let color2 = Color::from_rgb8(
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        );
        let max_factor = match noise_function.unwrap() {
            NoiseFunction::Fbm => 7.0,
            NoiseFunction::Billow => 7.0,
            NoiseFunction::Ridged => 7.0,
            NoiseFunction::Value => 7.0,
            NoiseFunction::Cylinders => 2.0,
            NoiseFunction::Worley => 7.0,
            NoiseFunction::Curl => 7.0,
            NoiseFunction::Magnet => 7.0,
            NoiseFunction::Gravity => 7.0,
        };
        let noise_factor = rng.gen_range(1.0..max_factor);
        let noise_scale = rng.gen_range(1.0..7.0);
        let octaves = rng.gen_range(1..9);
        let len_type: Option<ExtrusionStyle> = Some(rng.gen());
        let len_size = rng.gen_range(50.0..300.0);
        let len_dir: Option<Dir> = Some(rng.gen());
        let grad_style: Option<GradStyle> = Some(rng.gen());
        let curve_style: Option<CurveStyle> = Some(rng.gen());
        let background: Option<Background> = Some(rng.gen());
        let spacing = rng.gen_range(1.0..50.0);
        Controls {
            location,
            grid_sep,
            noise_factor,
            noise_scale,
            octaves,
            noise_function,
            color1,
            color2,
            len_type,
            len_size,
            len_dir,
            grad_style,
            curve_style,
            background,
            spacing,
            ..Default::default()
        }
    }
}
pub struct Xtrusion {
    pub controls: Controls,
    pub image: image::Handle,
}

impl Xtrusion {
    pub fn new() -> Self {
        let controls = Controls::new();
        let canvas = draw(&controls, 1.0);
        Self {
            controls: Controls::new(),
            image: image::Handle::from_pixels(canvas.width, canvas.height, canvas.pixmap.take()),
        }
    }

    pub fn draw(&mut self) {
        let controls = Controls {
            export_width: String::new(),
            export_height: String::new(),
            ..self.controls
        };
        let canvas = draw(&controls, 1.0);
        self.image = image::Handle::from_pixels(canvas.width, canvas.height, canvas.pixmap.take());
    }
}
