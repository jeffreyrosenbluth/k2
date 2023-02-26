use wassily::prelude::*;

use crate::color::*;
use crate::common::{Controls, HEIGHT, WIDTH};
use crate::field::*;
use crate::gradient::*;
use crate::noise::*;

pub fn draw(controls: &Controls, scale: f32) -> Canvas {
    let mut canvas = Canvas::with_scale(WIDTH, HEIGHT, scale);
    if let Ok(w) = controls.export_width.parse::<u32>() {
        if let Ok(h) = controls.export_height.parse::<u32>() {
            let aspect_ratio = h as f32 / w as f32;
            let h = aspect_ratio * WIDTH as f32;
            let s = w as f32 / WIDTH as f32;
            canvas = Canvas::with_scale(WIDTH, h as u32, s)
        }
    };
    // let step = if controls.spaced { 25.0 } else { 1.0 };
    let bg = BG::new(canvas.width, canvas.height);
    bg.canvas_bg(&mut canvas);
    let opts = NoiseOpts::with_wh(canvas.width(), canvas.height())
        .scales(controls.noise_scale)
        .factor(controls.noise_factor);
    let mut flow = Field {
        noise_function: match controls.noise_function.unwrap() {
            NoiseFunction::Fbm => Box::new(
                Fbm::<Perlin>::default()
                    .set_octaves(controls.octaves as usize)
                    .set_persistence(controls.persistence as f64),
            ),
            NoiseFunction::Billow => Box::new(
                Billow::<Perlin>::default()
                    .set_octaves(controls.octaves as usize)
                    .set_persistence(controls.persistence as f64),
            ),
            NoiseFunction::Ridged => Box::new(
                RidgedMulti::<Perlin>::default()
                    .set_octaves(controls.octaves as usize)
                    .set_persistence(controls.persistence as f64),
            ),
            NoiseFunction::Value => Box::<Value>::default(),
            NoiseFunction::Worley => {
                if controls.worley_dist {
                    Box::new(Worley::default().set_return_type(ReturnType::Distance))
                } else {
                    Box::<Worley>::default()
                }
            }
            NoiseFunction::Checkerboard => {
                Box::new(Checkerboard::default().set_size(controls.octaves as usize))
            }
            NoiseFunction::Cylinders => Box::new(
                TranslatePoint::new(Cylinders::default().set_frequency(controls.octaves as f64))
                    .set_x_translation(canvas.width as f64 / 2.0)
                    .set_y_translation(canvas.height as f64 / 2.0),
            ),
            NoiseFunction::Curl => Box::new(Curl::new(Perlin::default())),
        },
        noise_opts: opts,
        step_size: controls.spacing,
        width: canvas.width(),
        height: canvas.height(),
        max_length: controls.max_length,
        speed: controls.speed,
    };
    let starts =
        controls
            .location
            .unwrap()
            .starts(canvas.w_f32(), canvas.h_f32(), controls.grid_sep);
    let mut palette = Palette::new(expand_palette(color_palette(controls.palette_num)));
    palette.rotate_hue(controls.hue as f32);
    let len_fn = controls.len_type.unwrap().calc(
        canvas.w_f32(),
        canvas.h_f32(),
        controls.len_size,
        controls.len_dir.unwrap(),
    );
    let highlight = match controls.grad_style.unwrap() {
        GradStyle::Fiber => 2,
        GradStyle::Dark => 3,
        GradStyle::Light => 4,
        GradStyle::None => 5,
    };

    for p in starts {
        let pts = flow.curve(p.x, p.y);
        let c = palette.rand_color();

        for p in pts {
            let r = len_fn(p);
            let y0 = p.y - r;
            let y1 = p.y + r;
            let lg = paint_lg(p.x, y0, p.x, y1, c, highlight);
            ShapeBuilder::new()
                .line(pt(p.x, y0), pt(p.x, y1))
                .stroke_weight(controls.stroke_width)
                .stroke_paint(&lg)
                .build()
                .draw(&mut canvas);
        }
    }

    let border_color = palette.rand_color().darken_fixed(0.5);
    ShapeBuilder::new()
        .rect_xywh(pt(0, 0), pt(canvas.width, canvas.height))
        .no_fill()
        .stroke_color(border_color)
        .stroke_weight(20.0)
        .build()
        .draw(&mut canvas);
    canvas
}

pub async fn print(controls: Controls, scale: f32) {
    let canvas = draw(&controls, scale);
    let name = format!("./output/{}.png", "image");
    canvas.save_png(name);
}

pub struct BG(Canvas);

impl BG {
    pub fn new(width: u32, height: u32) -> Self {
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
