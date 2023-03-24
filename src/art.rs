// use wassily::prelude::distance_functions::*;
use wassily::prelude::*;

use crate::background::*;
use crate::color::*;
use crate::common::{Controls, CurveStyle, HEIGHT, WIDTH};
use crate::field::*;
use crate::gradient::*;
use crate::noise::*;

fn choose_flow(controls: &Controls, w: u32, h: u32) -> Field {
    let mut opts = NoiseOpts::with_wh(w, h)
        .scales(controls.noise_scale)
        .factor(controls.noise_factor);
    Field {
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
            NoiseFunction::Worley => Box::new(
                Worley::default().set_return_type(ReturnType::Distance), // .set_distance_function(chebyshev),
            ),
            NoiseFunction::Cylinders => Box::new(
                TranslatePoint::new(
                    Cylinders::default().set_frequency(controls.octaves as f64 / 2.0),
                )
                .set_x_translation(w as f64 / 2.0)
                .set_y_translation(h as f64 / 2.0),
            ),
            NoiseFunction::Curl => Box::new(Curl::new(Perlin::default())),
            NoiseFunction::Magnet => {
                opts = NoiseOpts::default();
                let mut rng = SmallRng::from_entropy();
                let (r1, r2, r3, r4, r5, r6): (f32, f32, f32, f32, f32, f32) = rng.gen();
                let w = w as f32;
                let h = h as f32;
                Box::new(Magnet::new(vec![
                    pt(r1 * w, r2 * h),
                    pt(r3 * w, r4 * h),
                    pt(r5 * w, r6 * h),
                ]))
            }
            NoiseFunction::Gravity => {
                opts = NoiseOpts::default();
                let mut rng = SmallRng::from_entropy();
                let (r1, r2, r3, r4, r5, r6): (f32, f32, f32, f32, f32, f32) = rng.gen();
                let w = w as f32;
                let h = h as f32;
                Box::new(Curl::new(Magnet::new(vec![
                    pt(r1 * w, r2 * h),
                    pt(r3 * w, r4 * h),
                    pt(r5 * w, r6 * h),
                ])))
            }
        },
        noise_opts: opts,
        step_size: controls.spacing,
        width: w,
        height: h,
        curve_length: controls.curve_length,
        speed: controls.speed,
    }
}

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

    let bg = match controls.background.unwrap() {
        Background::Clouds => BG::clouds(canvas.width, canvas.height),
        Background::Grain => BG::grain(canvas.width, canvas.height),
    };
    bg.canvas_bg(&mut canvas);

    let mut flow = choose_flow(controls, canvas.width(), canvas.height());

    let starts =
        controls
            .location
            .unwrap()
            .starts(canvas.w_f32(), canvas.h_f32(), controls.grid_sep);

    let mut palette = Palette::new(color_scale(
        Color::from_rgba(controls.color1.r, controls.color1.g, controls.color1.b, 1.0).unwrap(),
        Color::from_rgba(controls.color2.r, controls.color2.g, controls.color2.b, 1.0).unwrap(),
        8,
    ));

    let len_fn = controls.len_type.unwrap().calc(
        canvas.w_f32(),
        canvas.h_f32(),
        controls.len_size,
        controls.len_dir.unwrap(),
    );

    for p in starts {
        let pts = flow.curve(p.x, p.y);
        let c = palette.rand_color();

        match controls.curve_style.unwrap() {
            CurveStyle::Dots => {
                for p in pts {
                    let r = len_fn(p) / 15.0;
                    ShapeBuilder::new()
                        .circle(p, r)
                        .no_stroke()
                        .fill_color(c)
                        .build()
                        .draw(&mut canvas);
                }
            }
            CurveStyle::Line => ShapeBuilder::new()
                .points(&pts)
                .no_fill()
                .stroke_color(c)
                .stroke_weight(controls.stroke_width)
                .build()
                .draw(&mut canvas),
            CurveStyle::Extrusion => {
                for p in pts {
                    let r = len_fn(p);
                    let y0 = p.y - r;
                    let y1 = p.y + r;
                    let lg = paint_lg(p.x, y0, p.x, y1, c, controls.grad_style.unwrap());
                    ShapeBuilder::new()
                        .line(pt(p.x, y0), pt(p.x, y1))
                        .stroke_weight(controls.stroke_width)
                        .stroke_paint(&lg)
                        .build()
                        .draw(&mut canvas);
                }
            }
        }
    }

    let border_color = palette.rand_color().darken_fixed(0.25);
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
