use crate::common::SEED;
use rand::RngCore;
use wassily::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Location {
    Grid,
    Rand,
    Halton,
    Poisson,
    Circle,
    Lissajous,
}

impl Location {
    pub fn starts<R: RngCore>(&self, w: f32, h: f32, sep: f32, rng: &mut R) -> Vec<Point> {
        let mut pts = Vec::new();
        match &self {
            Location::Grid => {
                let mut i = 0.0;
                let mut j;
                while i <= w {
                    j = 0.0;
                    while j <= h {
                        pts.push(pt(i, j));
                        j += sep;
                    }
                    i += sep;
                }
            }
            Location::Rand => {
                let n = (w * h) / (sep * sep);
                for _ in 0..n as u32 {
                    pts.push(pt(rng.gen_range(0.0..w), rng.gen_range(0.0..h)));
                }
            }
            Location::Halton => {
                let n = (w * h) / (sep * sep);
                pts = halton_23(w, h, n as u32, SEED)
            }
            Location::Poisson => pts = poisson_disk(w, h, sep / 1.2, 0),
            Location::Circle => {
                let cx = w / 2.0;
                let cy = h / 2.0;
                let r = w.max(h);
                let radii = vec![r / 6.0, r / 3.5, r / 2.5];
                for r in radii {
                    let delta = 0.5 * sep / r;
                    let mut theta = 0.0;
                    while theta <= TAU {
                        pts.push(pt(cx + r * theta.cos(), cy + r * theta.sin()));
                        theta += delta;
                    }
                }
            }
            Location::Lissajous => {
                let n = (w * h) / (sep * sep);
                let cx = w / 2.0;
                let cy = h / 2.0;
                for i in 0..n as u32 {
                    let t = i as f32 * 2.0 * PI / n;
                    let x = 0.8 * w * (3.0 * t + PI / 2.0).sin();
                    let y = 0.8 * h * (2.0 * t).sin();
                    pts.push(pt(x / 2.0 + cx, y / 2.0 + cy));
                }
            }
        }
        pts
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Location::Grid => "Grid",
                Location::Rand => "Rand",
                Location::Halton => "Halton",
                Location::Poisson => "Poisson",
                Location::Circle => "Circle",
                Location::Lissajous => "Lissajous",
            }
        )
    }
}
