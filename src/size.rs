#![allow(dead_code)]

use crate::gui::{numeric, pick_list};
use eframe::egui;
use wassily::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    Both,
    Horizontal,
    Vertical,
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::Both => "Both",
                Dir::Horizontal => "Horizontal",
                Dir::Vertical => "Vertical",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SizeFn {
    Expanding,
    Contracting,
    Constant,
    Periodic,
}

impl std::fmt::Display for SizeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SizeFn::Constant => "Constant",
                SizeFn::Expanding => "Expanding",
                SizeFn::Contracting => "Contracting",
                SizeFn::Periodic => "Periodic",
            }
        )
    }
}

impl SizeFn {
    pub fn calc(
        self,
        w: f32,
        h: f32,
        r: f32,
        dir: Dir,
        scale: f32,
        min_size: f32,
    ) -> Box<dyn Fn(Point) -> f32 + Send + Sync> {
        match self {
            SizeFn::Expanding => Box::new(expanding(w, h, r, dir, min_size)),
            SizeFn::Contracting => Box::new(contracting(w, h, r, dir, min_size)),
            SizeFn::Constant => Box::new(constant(r)),
            SizeFn::Periodic => Box::new(periodic(w, h, r, scale, min_size)),
        }
    }
}

fn distance(p: Point, w: f32, h: f32, dir: Dir) -> f32 {
    let cx = (p.x - w / 2.0).abs();
    let cy = (p.y - h / 2.0).abs();
    match dir {
        Dir::Both => (cx * cx / (w * w) + cy * cy / (h * h)).sqrt(),
        Dir::Horizontal => cx / w,
        Dir::Vertical => cy / h,
    }
}

fn expanding(
    w: f32,
    h: f32,
    r: f32,
    dir: Dir,
    min_size: f32,
) -> impl Fn(Point) -> f32 + Send + Sync {
    move |p| f32::max(min_size, distance(p, w, h, dir) * r)
}

fn contracting(
    w: f32,
    h: f32,
    r: f32,
    dir: Dir,
    min_size: f32,
) -> impl Fn(Point) -> f32 + Send + Sync {
    move |p| f32::max(min_size, (0.5 - distance(p, w, h, dir)) * r)
}

fn constant(r: f32) -> impl Fn(Point) -> f32 + Send + Sync {
    move |_| r * 0.5
}

fn periodic(w: f32, h: f32, r: f32, scale: f32, min_size: f32) -> impl Fn(Point) -> f32 + Send + Sync {
    move |p| {
        let opts = NoiseOpts::with_wh(w, h).scales(scale);
        let nf = Perlin::default().set_seed(98713);
        f32::max(min_size, (noise2d_01(nf, &opts, p.x, p.y)) * r / 2.0)
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
    pub fn new(
        size_fn: Option<SizeFn>,
        size: f32,
        direction: Option<Dir>,
        size_scale: f32,
        min_size: f32,
    ) -> Self {
        Self {
            size_fn,
            size,
            direction,
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

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        use SizeFn::*;
        ui.separator();
        ui.label("Size");
        pick_list(
            ui,
            "Size Function",
            &[Constant, Expanding, Contracting, Periodic],
            &mut self.size_fn,
        );
        numeric(ui, "Size", &mut self.size, 5.0..=500.0, 5.0, 0);
        if self.size_fn == Some(Expanding) || self.size_fn == Some(Contracting) {
            pick_list(
                ui,
                "Direction",
                &[Dir::Both, Dir::Horizontal, Dir::Vertical],
                &mut self.direction,
            );
            numeric(ui, "Min Size", &mut self.min_size, 1.0..=50.0, 1.0, 1);
        } else if self.size_fn == Some(Periodic) {
            numeric(ui, "Size Scale", &mut self.size_scale, 1.0..=30.0, 1.0, 1);
            numeric(ui, "Min Size", &mut self.min_size, 1.0..=50.0, 1.0, 1);
        }
    }
}
