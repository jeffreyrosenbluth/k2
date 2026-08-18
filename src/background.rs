use rand::RngCore;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use wassily::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Background {
    LightGrain,
    LightFiber,
    DarkGrain,
    DarkFiber,
    ColorGrain,
    White,
    Black,
    Solid,
}

impl std::fmt::Display for Background {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Background::LightGrain => "Light Grain",
                Background::LightFiber => "Light Fiber",
                Background::DarkGrain => "Dark Grain",
                Background::DarkFiber => "Dark Fiber ",
                Background::ColorGrain => "Color Grain",
                Background::White => "Solid White",
                Background::Black => "Solid Black",
                Background::Solid => "Solid Color",
            }
        )
    }
}
pub struct BG(Canvas);

/// A zero-mean, unit-variance film grain field: gaussian noise softly
/// blurred with a radius proportional to the render scale, so the grain
/// keeps the same size relative to the image at preview, display, and
/// print resolutions.
fn film_grain(width: u32, height: u32, scale: f32, size: f32, seed: u64) -> Vec<f32> {
    let w = width as usize;
    let h = height as usize;
    let mut noise = vec![0.0f32; w * h];
    noise
        .par_chunks_mut(w)
        .enumerate()
        .for_each(|(j, row)| {
            let mut rng =
                SmallRng::seed_from_u64(seed ^ (j as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
            for v in row.iter_mut() {
                // sum of three uniforms approximates a gaussian
                *v = rng.random_range(-0.5..0.5f32)
                    + rng.random_range(-0.5..0.5f32)
                    + rng.random_range(-0.5..0.5f32);
            }
        });
    // Two separable box-blur passes approximate a gaussian blur and give
    // the grain its soft, clustered film character.
    let radius = ((0.8 * scale * size).round() as usize).min(24);
    for _ in 0..2 {
        if radius == 0 {
            break;
        }
        let src = noise.clone();
        noise.par_chunks_mut(w).enumerate().for_each(|(j, row)| {
            for (i, v) in row.iter_mut().enumerate() {
                let lo = i.saturating_sub(radius);
                let hi = (i + radius).min(w - 1);
                let sum: f32 = src[j * w + lo..=j * w + hi].iter().sum();
                *v = sum / (hi - lo + 1) as f32;
            }
        });
        let src = noise.clone();
        noise.par_chunks_mut(w).enumerate().for_each(|(j, row)| {
            let lo = j.saturating_sub(radius);
            let hi = (j + radius).min(h - 1);
            for (i, v) in row.iter_mut().enumerate() {
                let mut sum = 0.0;
                for jj in lo..=hi {
                    sum += src[jj * w + i];
                }
                *v = sum / (hi - lo + 1) as f32;
            }
        });
    }
    // Blur shrinks the contrast; renormalize to unit variance.
    let var: f32 =
        noise.iter().step_by(97).map(|v| v * v).sum::<f32>() / (noise.len() / 97).max(1) as f32;
    let gain = 1.0 / var.sqrt().max(1e-6);
    noise.par_iter_mut().for_each(|v| *v *= gain);
    noise
}

impl BG {
    // Writes each pixel directly instead of rasterizing a 1x1 rect per pixel;
    // rows run in parallel with a deterministic per-row rng.
    fn from_pixels(
        width: u32,
        height: u32,
        seed: u64,
        f: impl Fn(u32, u32, &mut SmallRng) -> Color + Sync,
    ) -> Self {
        let mut canvas = Canvas::new(width, height);
        canvas
            .pixmap
            .pixels_mut()
            .par_chunks_mut(width as usize)
            .enumerate()
            .for_each(|(j, row)| {
                let mut rng =
                    SmallRng::seed_from_u64(seed ^ (j as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
                for (i, px) in row.iter_mut().enumerate() {
                    *px = f(i as u32, j as u32, &mut rng).premultiply().to_color_u8();
                }
            });
        BG(canvas)
    }

    pub fn solid(width: u32, height: u32, color: Color) -> Self {
        let mut canvas = Canvas::new(width, height);
        canvas.fill(color);
        BG(canvas)
    }

    /// A grain background: `base` luminance modulated by film grain of
    /// strength `amp`, optionally tinting a color.
    fn grained(
        width: u32,
        height: u32,
        scale: f32,
        size: f32,
        seed: u64,
        base: f32,
        amp: f32,
        tint: Option<(f32, f32, f32)>,
    ) -> Self {
        let grain = film_grain(width, height, scale, size, seed);
        let mut canvas = Canvas::new(width, height);
        canvas
            .pixmap
            .pixels_mut()
            .par_chunks_mut(width as usize)
            .enumerate()
            .for_each(|(j, row)| {
                for (i, px) in row.iter_mut().enumerate() {
                    let v = (base + amp * grain[j * width as usize + i]).clamp(0.0, 1.0);
                    let c = match tint {
                        Some((r, g, b)) => {
                            Color::from_rgba(r * v, g * v, b * v, 1.0).unwrap()
                        }
                        None => Color::from_rgba(v, v, v, 1.0).unwrap(),
                    };
                    *px = c.premultiply().to_color_u8();
                }
            });
        BG(canvas)
    }

    pub fn color_grain<R: RngCore>(
        width: u32,
        height: u32,
        scale: f32,
        amount: f32,
        size: f32,
        rng: &mut R,
        color: eframe::egui::Color32,
    ) -> Self {
        let seed = rng.next_u64();
        let tint = (
            color.r() as f32 / 255.0,
            color.g() as f32 / 255.0,
            color.b() as f32 / 255.0,
        );
        Self::grained(width, height, scale, size, seed, 0.875, 0.05 * amount, Some(tint))
    }

    pub fn dark_grain<R: RngCore>(
        width: u32,
        height: u32,
        scale: f32,
        amount: f32,
        size: f32,
        rng: &mut R,
    ) -> Self {
        let seed = rng.next_u64();
        Self::grained(width, height, scale, size, seed, 0.14, 0.05 * amount, None)
    }

    pub fn light_grain<R: RngCore>(
        width: u32,
        height: u32,
        scale: f32,
        amount: f32,
        size: f32,
        rng: &mut R,
    ) -> Self {
        let seed = rng.next_u64();
        Self::grained(width, height, scale, size, seed, 0.95, 0.035 * amount, None)
    }

    pub fn light_fiber(width: u32, height: u32, scale: f32) -> Self {
        let nf1 = Fbm::<Perlin>::default().set_octaves(4);
        let nf2: Turbulence<Fbm<Perlin>, Perlin> =
            Turbulence::new(nf1).set_power(2.0).set_roughness(6);
        let opts = NoiseOpts::default();
        Self::from_pixels(width, height, 0, |i, j, _| {
            let y = 255
                - (40.0 * noise2d_01(&nf2, &opts, i as f32 / scale * 0.005, j as f32 / scale * 0.30))
                    as u8;
            Color::from_rgba8(y, y, y, 255)
        })
    }

    pub fn dark_fiber(width: u32, height: u32, scale: f32) -> Self {
        let nf1 = Fbm::<Perlin>::default().set_octaves(4);
        let nf2: Turbulence<Fbm<Perlin>, Perlin> =
            Turbulence::new(nf1).set_power(2.0).set_roughness(6);
        let opts = NoiseOpts::default();
        Self::from_pixels(width, height, 0, |i, j, _| {
            let y = 25
                + (30.0 * noise2d_01(&nf2, &opts, i as f32 / scale * 0.005, j as f32 / scale * 0.30))
                    as u8;
            Color::from_rgba8(y, y, y, 255)
        })
    }

    pub fn bg(&self) -> Paint<'_> {
        let pattern = Pattern::new(
            (self.0).pixmap.as_ref(),
            SpreadMode::Repeat,
            FilterQuality::Nearest,
            1.0,
            Transform::identity(),
        );
        paint_shader(pattern)
    }

    pub fn canvas_bg(&self, canvas: &mut Canvas) {
        let paint = self.bg();
        Shape::new()
            .rect_xywh(pt(0, 0), pt(canvas.w_f32(), canvas.h_f32()))
            .fill_paint(&paint)
            .draw(canvas);
    }
}
