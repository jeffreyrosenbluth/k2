use rand::RngCore;
use rayon::prelude::*;
use wassily::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Background {
    LightGrain,
    LightFiber,
    DarkGrain,
    DarkFiber,
    ColorGrain,
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
            }
        )
    }
}
pub struct BG(Canvas);

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

    pub fn color_grain<R: RngCore>(
        width: u32,
        height: u32,
        rng: &mut R,
        color: iced::Color,
    ) -> Self {
        // Color with alpha in [0.8, 0.95] composited over an opaque black base.
        let seed = rng.next_u64();
        Self::from_pixels(width, height, seed, |_, _, rng| {
            let alpha: f32 = rng.random_range(0.8..=0.95);
            Color::from_rgba(color.r * alpha, color.g * alpha, color.b * alpha, 1.0).unwrap()
        })
    }

    pub fn dark_grain<R: RngCore>(width: u32, height: u32, rng: &mut R) -> Self {
        // Black with alpha in [200, 240] composited over an opaque white base.
        let seed = rng.next_u64();
        Self::from_pixels(width, height, seed, |_, _, rng| {
            let alpha = rng.random_range(200..=240u16) as f32 / 255.0;
            let v = 1.0 - alpha;
            Color::from_rgba(v, v, v, 1.0).unwrap()
        })
    }

    pub fn light_grain<R: RngCore>(width: u32, height: u32, rng: &mut R) -> Self {
        // Gray at alpha 25/255, multiply-blended over an opaque white base.
        let seed = rng.next_u64();
        Self::from_pixels(width, height, seed, |_, _, rng| {
            let brt = rng.random_range(0..=255u16) as f32 / 255.0;
            let sa = 25.0 / 255.0;
            let v = 1.0 - sa * (1.0 - brt);
            Color::from_rgba(v, v, v, 1.0).unwrap()
        })
    }

    pub fn light_fiber(width: u32, height: u32) -> Self {
        let nf1 = Fbm::<Perlin>::default().set_octaves(4);
        let nf2: Turbulence<Fbm<Perlin>, Perlin> =
            Turbulence::new(nf1).set_power(2.0).set_roughness(6);
        let opts = NoiseOpts::default();
        Self::from_pixels(width, height, 0, |i, j, _| {
            let y = 255 - (40.0 * noise2d_01(&nf2, &opts, i as f32 * 0.005, j as f32 * 0.30)) as u8;
            Color::from_rgba8(y, y, y, 255)
        })
    }

    pub fn dark_fiber(width: u32, height: u32) -> Self {
        let nf1 = Fbm::<Perlin>::default().set_octaves(4);
        let nf2: Turbulence<Fbm<Perlin>, Perlin> =
            Turbulence::new(nf1).set_power(2.0).set_roughness(6);
        let opts = NoiseOpts::default();
        Self::from_pixels(width, height, 0, |i, j, _| {
            let y = 25 + (30.0 * noise2d_01(&nf2, &opts, i as f32 * 0.005, j as f32 * 0.30)) as u8;
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
