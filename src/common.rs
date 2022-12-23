use crate::art::{draw, Cap, Dir, Len, Location};
use iced::widget::image;
use rand::distributions::Standard;
use rand::prelude::*;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 450;

#[derive(Clone)]
pub struct Controls {
    pub spaced: bool,
    pub hue: f32,
    pub palette_num: u8,
    pub location: Option<Location>,
    pub grid_sep: f32,
    pub noise_factor: f32,
    pub noise_scale: f32,
    pub octaves: u8,
    pub curl: bool,
    pub len_type: Option<Len>,
    pub len_size: f32,
    pub len_dir: Option<Dir>,
    pub len_freq: f32,
    pub cap: Option<Cap>,
    pub exporting: bool,
}

impl Controls {
    pub fn new() -> Self {
        Self {
            spaced: false,
            palette_num: 0,
            hue: 0.0,
            location: Some(Location::Rand),
            grid_sep: 50.0,
            noise_factor: 4.0,
            noise_scale: 4.0,
            octaves: 1,
            curl: false,
            len_type: Some(Len::Contracting),
            len_size: 150.0,
            len_dir: Some(Dir::Circle),
            len_freq: 5.0,
            cap: Some(Cap::None),
            exporting: false,
        }
    }
}

impl Distribution<Controls> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Controls {
        let spaced = rng.gen_bool(0.25);
        let hue = rng.gen_range(0.0..360.0);
        let palette_num = rng.gen_range(0..10);
        let location: Option<Location> = Some(rng.gen());
        let grid_sep = rng.gen_range(25.0..100.0);
        let noise_factor = rng.gen_range(1.0..7.0);
        let noise_scale = rng.gen_range(1.0..7.0);
        let octaves = rng.gen_range(1..8);
        let curl = false;
        let len_type: Option<Len> = Some(rng.gen());
        let len_size = rng.gen_range(100.0..325.0);
        let len_dir: Option<Dir> = Some(rng.gen());
        let len_freq = rng.gen_range(1.0..10.0);
        let cap: Option<Cap> = Some(rng.gen());
        Controls {
            spaced,
            hue,
            palette_num,
            location,
            grid_sep,
            noise_factor,
            noise_scale,
            octaves,
            curl,
            len_type,
            len_size,
            len_dir,
            len_freq,
            cap,
            exporting: false,
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
}
