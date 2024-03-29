use rand::RngCore;
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
    pub fn color_grain<R: RngCore>(
        width: u32,
        height: u32,
        rng: &mut R,
        color: iced::Color,
    ) -> Self {
        let mut canvas = Canvas::new(width, height);
        canvas.fill(*BLACK);
        for i in 0..width {
            for j in 0..height {
                let alpha = rng.gen_range(0.8..=0.95);
                let c = Color::from_rgba(color.r, color.g, color.b, alpha).unwrap();
                let mut paint = Paint::default();
                paint.set_color(c);
                Shape::new()
                    .rect_xywh(pt(i, j), pt(1, 1))
                    .fill_paint(&paint)
                    .no_stroke()
                    .draw(&mut canvas);
            }
        }
        BG(canvas)
    }
    pub fn dark_grain<R: RngCore>(width: u32, height: u32, rng: &mut R) -> Self {
        let mut canvas = Canvas::new(width, height);
        canvas.fill(*WHITE);
        for i in 0..width {
            for j in 0..height {
                let alpha = rng.gen_range(200..=240);
                let c = Color::from_rgba8(0, 0, 0, alpha);
                let mut paint = Paint::default();
                paint.set_color(c);
                Shape::new()
                    .rect_xywh(pt(i, j), pt(1, 1))
                    .fill_paint(&paint)
                    .no_stroke()
                    .draw(&mut canvas);
            }
        }
        BG(canvas)
    }

    pub fn light_grain<R: RngCore>(width: u32, height: u32, rng: &mut R) -> Self {
        let mut canvas = Canvas::new(width, height);
        canvas.fill(*WHITE);
        for i in 0..width {
            for j in 0..height {
                let brt = rng.gen_range(0..=255);
                let c = Color::from_rgba8(brt, brt, brt, 25);
                let mut paint = Paint::default();
                paint.set_color(c);
                paint.blend_mode = BlendMode::Multiply;
                Shape::new()
                    .rect_xywh(pt(i, j), pt(1, 1))
                    .fill_paint(&paint)
                    .no_stroke()
                    .draw(&mut canvas);
            }
        }
        BG(canvas)
    }

    pub fn light_fiber(width: u32, height: u32) -> Self {
        let mut canvas = Canvas::new(width, height);
        let nf1 = Fbm::<Perlin>::default().set_octaves(4);
        let nf2: Turbulence<Fbm<Perlin>, Perlin> =
            Turbulence::new(nf1).set_power(2.0).set_roughness(6);
        let opts = NoiseOpts::default();
        for i in 0..width {
            for j in 0..height {
                let y =
                    255 - (40.0 * noise2d_01(&nf2, &opts, i as f32 * 0.005, j as f32 * 0.30)) as u8;
                let c = Color::from_rgba8(y, y, y, 255);
                let mut paint = Paint::default();
                paint.set_color(c);
                paint.blend_mode = BlendMode::Multiply;
                Shape::new()
                    .rect_xywh(pt(i, j), pt(1, 1))
                    .fill_paint(&paint)
                    .no_stroke()
                    .draw(&mut canvas);
            }
        }
        BG(canvas)
    }

    pub fn dark_fiber(width: u32, height: u32) -> Self {
        let mut canvas = Canvas::new(width, height);
        let nf1 = Fbm::<Perlin>::default().set_octaves(4);
        let nf2: Turbulence<Fbm<Perlin>, Perlin> =
            Turbulence::new(nf1).set_power(2.0).set_roughness(6);
        let opts = NoiseOpts::default();
        for i in 0..width {
            for j in 0..height {
                let y =
                    25 + (30.0 * noise2d_01(&nf2, &opts, i as f32 * 0.005, j as f32 * 0.30)) as u8;
                let c = Color::from_rgba8(y, y, y, 255);
                let mut paint = Paint::default();
                paint.set_color(c);
                paint.blend_mode = BlendMode::Multiply;
                Shape::new()
                    .rect_xywh(pt(i, j), pt(1, 1))
                    .fill_paint(&paint)
                    .no_stroke()
                    .draw(&mut canvas);
            }
        }
        BG(canvas)
    }

    pub fn bg(&self) -> Paint {
        let pattern = Pattern::new(
            (self.0).pixmap.as_ref(),
            SpreadMode::Repeat,
            FilterQuality::Nearest,
            1.0,
            Transform::identity(),
        );
        let p = paint_shader(pattern);
        p
    }

    pub fn canvas_bg(&self, canvas: &mut Canvas) {
        let paint = self.bg();
        Shape::new()
            .rect_xywh(pt(0, 0), pt(canvas.w_f32(), canvas.h_f32()))
            .fill_paint(&paint)
            .draw(canvas);
    }
}
