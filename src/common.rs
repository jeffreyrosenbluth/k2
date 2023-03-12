use crate::art::draw;
use crate::gradient::GradStyle;
use crate::length::{Dir, Len};
use crate::location::Location;
use crate::noise::NoiseFunction;
use iced::widget::image;
use rand::distributions::Standard;
use rand::prelude::*;

pub const WIDTH: u32 = 1000;
pub const HEIGHT: u32 = 1000;

// pub fn uniform_sum<R: Rng + ?Sized>(rng: &mut R, n: u32) -> f32 {
//     let mut sum = 0.0;
//     for _ in 0..n {
//         sum += rng.gen::<f32>();
//     }
//     sum / n as f32
// }

#[derive(Clone)]
pub struct Controls {
    pub hi_res: bool,
    pub xtrude: bool,
    pub spacing: f32,
    pub curve_length: u32,
    pub hue: u16,
    pub palette_num: u8,
    pub location: Option<Location>,
    pub grid_sep: f32,
    pub noise_factor: f32,
    pub noise_scale: f32,
    pub octaves: u8,
    pub persistence: f32,
    pub noise_function: Option<NoiseFunction>,
    pub speed: f32,
    pub len_type: Option<Len>,
    pub len_size: f32,
    pub len_dir: Option<Dir>,
    pub grad_style: Option<GradStyle>,
    pub exporting: bool,
    pub worley_dist: bool,
    pub stroke_width: f32,
    pub export_width: String,
    pub export_height: String,
}

impl Controls {
    pub fn new() -> Self {
        Self {
            hi_res: false,
            xtrude: true,
            spacing: 4.0,
            curve_length: 50,
            palette_num: 9,
            hue: 0,
            location: Some(Location::Halton),
            grid_sep: 50.0,
            noise_factor: 1.0,
            noise_scale: 4.0,
            octaves: 4,
            persistence: 0.5,
            noise_function: Some(NoiseFunction::Fbm),
            speed: 1.0,
            len_type: Some(Len::Contracting),
            len_size: 200.0,
            len_dir: Some(Dir::Both),
            grad_style: Some(GradStyle::None),
            exporting: false,
            worley_dist: false,
            stroke_width: 8.0,
            export_width: String::new(),
            export_height: String::new(),
        }
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self::new()
    }
}

impl Distribution<Controls> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Controls {
        let palette_num = rng.gen_range(0..11);
        let location: Option<Location> = Some(rng.gen());
        let grid_sep = rng.gen_range(25.0..75.0);
        let noise_function: Option<NoiseFunction> = Some(rng.gen());
        let max_factor = match noise_function.unwrap() {
            NoiseFunction::Fbm => 7.0,
            NoiseFunction::Billow => 7.0,
            NoiseFunction::Ridged => 7.0,
            NoiseFunction::Value => 7.0,
            NoiseFunction::Checkerboard => 2.0,
            NoiseFunction::Cylinders => 2.0,
            NoiseFunction::Worley => 7.0,
            NoiseFunction::Curl => 7.0,
        };
        let noise_factor = rng.gen_range(1.0..max_factor);
        let noise_scale = rng.gen_range(1.0..7.0);
        let octaves = rng.gen_range(1..9);
        let len_type: Option<Len> = Some(rng.gen());
        let len_size = rng.gen_range(50.0..300.0);
        let len_dir: Option<Dir> = Some(rng.gen());
        let cap: Option<GradStyle> = Some(rng.gen());
        Controls {
            palette_num,
            location,
            grid_sep,
            noise_factor,
            noise_scale,
            octaves,
            noise_function,
            len_type,
            len_size,
            len_dir,
            grad_style: cap,
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
        let mut controls = Controls::new();
        let canvas = draw(&mut controls, 1.0);
        Self {
            controls: Controls::new(),
            image: image::Handle::from_pixels(canvas.width, canvas.height, canvas.pixmap.take()),
        }
    }

    pub fn draw(&mut self) {
        let mut controls = Controls {
            export_width: String::new(),
            export_height: String::new(),
            ..self.controls
        };
        let canvas = draw(&mut controls, 1.0);
        self.image = image::Handle::from_pixels(canvas.width, canvas.height, canvas.pixmap.take());
    }
}
