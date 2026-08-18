use crate::common::SEED;
use rand::RngCore;
use wassily::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Location {
    Grid,
    Rand,
    Halton,
    Poisson,
    Circle,
    Lissajous,
    Box,
    Column,
    Even,
}

impl Location {
    pub fn starts<R: RngCore>(
        &self,
        w: f32,
        h: f32,
        sep: f32,
        angle: f32,
        rng: &mut R,
    ) -> Vec<Point> {
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
                    pts.push(pt(rng.random_range(0.0..w), rng.random_range(0.0..h)));
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
                let divs = vec![1.0 / 6.0, 1.0 / 3.5, 1.0 / 2.5];
                for d in divs {
                    let delta = 0.5 * sep / w.max(h);
                    let mut theta = 0.0;
                    while theta <= TAU {
                        pts.push(pt(cx + d * w * theta.cos(), cy + d * h * theta.sin()));
                        theta += delta;
                    }
                }
            }
            Location::Box => {
                // Every point starts outside the piece, spaced `sep` apart
                // along the perimeter of a rectangle slightly larger than the
                // canvas; the curves have to flow into the frame.
                let margin = 0.05 * w.max(h);
                let (x0, x1) = (-margin, w + margin);
                let (y0, y1) = (-margin, h + margin);
                let mut t = 0.0;
                while x0 + t <= x1 {
                    pts.push(pt(x0 + t, y0));
                    pts.push(pt(x0 + t, y1));
                    t += sep;
                }
                let mut t = sep;
                while y0 + t < y1 {
                    pts.push(pt(x0, y0 + t));
                    pts.push(pt(x1, y0 + t));
                    t += sep;
                }
            }
            // A single centered column of seeds, spaced `sep` apart and
            // extending past the canvas; made for the Strips style, whose
            // two-sided curves sweep out full-width bands.
            Location::Column => {
                // A line of seeds through the center, rotated by `angle`:
                // 0 degrees is a vertical column, 90 a horizontal row, and
                // anything between a diagonal. Long enough to cover the
                // canvas at any rotation.
                let (cx, cy) = (w / 2.0, h / 2.0);
                let a = angle.to_radians();
                let (dx, dy) = (a.sin(), a.cos());
                let half = 0.5 * (w * w + h * h).sqrt() + 0.05 * w.max(h);
                let mut t = -half;
                while t <= half {
                    pts.push(pt(cx + t * dx, cy + t * dy));
                    t += sep;
                }
            }
            // Evenly spaced curves couple seeding with curve growth and
            // are generated in draw() instead.
            Location::Even => {}
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
                Location::Box => "Box",
                Location::Column => "Column",
                Location::Even => "Even",
            }
        )
    }
}
