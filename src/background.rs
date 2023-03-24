use wassily::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Background {
    Grain,
    Clouds,
}

impl Distribution<Background> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Background {
        let index: u8 = rng.gen_range(0..2);
        match index {
            0 => Background::Grain,
            1 => Background::Clouds,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Background {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Background::Grain => "Grain",
                Background::Clouds => "Clouds",
            }
        )
    }
}
pub struct BG(Canvas);

impl BG {
    pub fn grain(width: u32, height: u32) -> Self {
        let mut canvas = Canvas::new(width, height);
        canvas.fill(*WHITE);
        let mut rng = SmallRng::from_entropy();
        for i in 0..width {
            for j in 0..height {
                let brt = rng.gen_range(0..=255);
                let c = Color::from_rgba8(brt, brt, brt, 25);
                let mut paint = Paint::default();
                paint.set_color(c);
                paint.blend_mode = BlendMode::Multiply;
                ShapeBuilder::new()
                    .rect_xywh(pt(i, j), pt(1, 1))
                    .fill_paint(&paint)
                    .no_stroke()
                    .build()
                    .draw(&mut canvas);
            }
        }
        BG(canvas)
    }

    pub fn clouds(width: u32, height: u32) -> Self {
        let mut canvas = Canvas::new(width, height);
        let nf = Fbm::<Perlin>::default().set_octaves(4);
        let opts = NoiseOpts::default();
        for i in 0..width {
            for j in 0..height {
                let y =
                    225 + (30.0 * noise2d_01(&nf, &opts, i as f32 * 0.05, j as f32 * 0.10)) as u8;
                let c = Color::from_rgba8(y, y, y, 255);
                let mut paint = Paint::default();
                paint.set_color(c);
                paint.blend_mode = BlendMode::Multiply;
                ShapeBuilder::new()
                    .rect_xywh(pt(i, j), pt(1, 1))
                    .fill_paint(&paint)
                    .no_stroke()
                    .build()
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
        ShapeBuilder::new()
            .rect_xywh(pt(0, 0), pt(canvas.w_f32(), canvas.h_f32()))
            .fill_paint(&paint)
            .build()
            .draw(canvas);
    }
}