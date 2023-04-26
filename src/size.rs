#![allow(dead_code)]

use crate::gui::{lpicklist::LPickList, numeric_input::NumericInput};
use iced::{
    widget::{Column, Rule},
    Element,
};
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
    ) -> Box<dyn Fn(Point) -> f32> {
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

fn expanding(w: f32, h: f32, r: f32, dir: Dir, min_size: f32) -> impl Fn(Point) -> f32 {
    move |p| f32::max(min_size, distance(p, w, h, dir) * r)
}

fn contracting(w: f32, h: f32, r: f32, dir: Dir, min_size: f32) -> impl Fn(Point) -> f32 {
    move |p| f32::max(min_size, (0.5 - distance(p, w, h, dir)) * r)
}

fn constant(r: f32) -> impl Fn(Point) -> f32 {
    move |_| r * 0.5
}

fn periodic(w: f32, h: f32, r: f32, scale: f32, min_size: f32) -> impl Fn(Point) -> f32 {
    move |p| {
        let opts = NoiseOpts::with_wh(w, h).scales(scale);
        let nf = Perlin::default().set_seed(98713);
        f32::max(min_size, (noise2d_01(nf, &opts, p.x, p.y)) * r / 2.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SizeMessage {
    SizeFn(SizeFn),
    Size(f32),
    Direction(Dir),
    SizeScale(f32),
    MinSize(f32),
    Null,
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

impl<'a> SizeControls {
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

    pub fn update(&mut self, message: SizeMessage) {
        match message {
            SizeMessage::SizeFn(size_fn) => {
                self.size_fn = Some(size_fn);
            }
            SizeMessage::Size(size) => {
                self.size = size;
            }
            SizeMessage::Direction(direction) => self.direction = Some(direction),
            SizeMessage::SizeScale(size_scale) => {
                self.size_scale = size_scale;
            }
            SizeMessage::MinSize(min_size) => {
                self.min_size = min_size;
            }
            SizeMessage::Null => (),
        }
    }

    pub fn view(&mut self) -> Element<'a, SizeMessage> {
        use self::SizeFn::*;
        use SizeMessage::*;
        let mut col = Column::new()
            .push(Rule::horizontal(10))
            .push("Size")
            .push(LPickList::new(
                "Size Function".to_string(),
                vec![Constant, Expanding, Contracting, Periodic],
                self.size_fn,
                |x| x.map_or(Null, SizeMessage::SizeFn),
            ))
            .push(
                NumericInput::new(
                    "Size".to_string(),
                    self.size,
                    5.0..=500.0,
                    5.0,
                    0,
                    SizeMessage::Size,
                )
                .decimals(0),
            );
        if self.size_fn == Some(Expanding) || self.size_fn == Some(Contracting) {
            col = col
                .push(LPickList::new(
                    "Direction".to_string(),
                    vec![Dir::Both, Dir::Horizontal, Dir::Vertical],
                    self.direction,
                    |x| x.map_or(Null, SizeMessage::Direction),
                ))
                .push(NumericInput::new(
                    "Min Size".to_string(),
                    self.min_size,
                    1.0..=50.0,
                    1.0,
                    1,
                    SizeMessage::MinSize,
                ))
        } else if self.size_fn == Some(Periodic) {
            col = col
                .push(NumericInput::new(
                    "Size Scale".to_string(),
                    self.size_scale,
                    1.0..=30.0,
                    1.0,
                    1,
                    SizeMessage::SizeScale,
                ))
                .push(NumericInput::new(
                    "Min Size".to_string(),
                    self.min_size,
                    1.0..=50.0,
                    1.0,
                    1,
                    SizeMessage::MinSize,
                ))
        }
        col.spacing(15).into()
    }
}
