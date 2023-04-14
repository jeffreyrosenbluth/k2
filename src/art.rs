use directories::UserDirs;
use std::path::PathBuf;
use wassily::prelude::*;

use crate::background::*;
use crate::color::*;
use crate::common::SEED;
use crate::common::{Controls, CurveStyle, DotStyle, HEIGHT, WIDTH};
use crate::field::*;
use crate::gradient::*;
use crate::noise::*;

fn choose_flow(controls: &Controls, w: u32, h: u32) -> Field {
    let mut opts = NoiseOpts::with_wh(w, h)
        .scales(controls.noise_controls.noise_scale)
        .factor(controls.noise_controls.noise_factor);
    Field {
        noise_function: match controls.noise_controls.noise_function.unwrap() {
            NoiseFunction::Fbm => Box::new(
                Fbm::<Perlin>::default()
                    .set_octaves(controls.fractal_controls.octaves as usize)
                    .set_persistence(controls.fractal_controls.persistence as f64),
            ),
            NoiseFunction::Billow => Box::new(
                Billow::<Perlin>::default()
                    .set_octaves(controls.fractal_controls.octaves as usize)
                    .set_persistence(controls.fractal_controls.persistence as f64),
            ),
            NoiseFunction::Ridged => Box::new(
                RidgedMulti::<Perlin>::default()
                    .set_octaves(controls.fractal_controls.octaves as usize)
                    .set_persistence(controls.fractal_controls.persistence as f64),
            ),
            NoiseFunction::Value => Box::<Value>::default(),
            NoiseFunction::Worley => {
                Box::new(Worley::default().set_return_type(ReturnType::Distance))
            }
            NoiseFunction::Cylinders => Box::new(
                TranslatePoint::new(
                    Cylinders::default()
                        .set_frequency(controls.fractal_controls.octaves as f64 / 2.0),
                )
                .set_x_translation(w as f64 / 2.0)
                .set_y_translation(h as f64 / 2.0),
            ),
            NoiseFunction::Curl => Box::new(Curl::new(Perlin::default())),
            NoiseFunction::Magnet => {
                opts = NoiseOpts::default();
                let mut rng = SmallRng::seed_from_u64(SEED);
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
                let mut rng = SmallRng::seed_from_u64(SEED);
                let (r1, r2, r3, r4, r5, r6): (f32, f32, f32, f32, f32, f32) = rng.gen();
                let w = w as f32;
                let h = h as f32;
                Box::new(Curl::new(Magnet::new(vec![
                    pt(r1 * w, r2 * h),
                    pt(r3 * w, r4 * h),
                    pt(r5 * w, r6 * h),
                ])))
            }
            NoiseFunction::Sinusoidal => Box::new(Sinusoidal::new(
                controls.sin_controls.sin_xfreq as f64,
                controls.sin_controls.sin_yfreq as f64,
                controls.sin_controls.sin_xexp as f64,
                controls.sin_controls.sin_yexp as f64,
            )),
        },
        noise_opts: opts,
        step_size: controls.spacing,
        width: w,
        height: h,
        curve_length: controls.curve_length,
        speed: controls.speed,
    }
}

pub fn draw(controls: &Controls, print: bool) -> Canvas {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    if let Ok(w) = controls.width.parse::<u32>() {
        if let Ok(h) = controls.height.parse::<u32>() {
            let aspect_ratio = w as f32 / h as f32;
            let mut ch = HEIGHT;
            let mut cw = WIDTH;
            if w >= h {
                ch = (WIDTH as f32 / aspect_ratio) as u32;
            } else {
                cw = (HEIGHT as f32 * aspect_ratio) as u32;
            }
            if print {
                canvas = Canvas::with_scale(cw, ch, std::cmp::max(w, h) as f32 / 1000.0)
            } else {
                canvas = Canvas::new(cw, ch)
            }
        }
    };

    let mut rng = SmallRng::seed_from_u64(SEED);

    let bg = match controls.background.unwrap() {
        Background::LightClouds => BG::light_clouds(canvas.width, canvas.height),
        Background::LightGrain => BG::light_grain(canvas.width, canvas.height, &mut rng),
        Background::DarkGrain => BG::dark_grain(canvas.width, canvas.height, &mut rng),
        Background::DarkClouds => BG::dark_clouds(canvas.width, canvas.height),
    };
    bg.canvas_bg(&mut canvas);

    let mut flow = choose_flow(controls, canvas.width, canvas.height);

    let starts = controls.location.unwrap().starts(
        canvas.w_f32(),
        canvas.h_f32(),
        105.0 - controls.density,
        &mut rng,
    );

    let mut palette = Palette::new(color_scale(
        Color::from_rgba(controls.color1.r, controls.color1.g, controls.color1.b, 1.0).unwrap(),
        Color::from_rgba(controls.color2.r, controls.color2.g, controls.color2.b, 1.0).unwrap(),
        8,
    ));

    let len_fn = controls.size_controls.size_fn.unwrap().calc(
        canvas.w_f32(),
        canvas.h_f32(),
        controls.size_controls.size,
        controls.size_controls.direction.unwrap(),
        controls.size_controls.size_scale,
        controls.size_controls.min_size,
    );

    for p in starts {
        let pts = flow.curve(p.x, p.y);
        let c = palette.rand_color();

        match controls.curve_style.unwrap() {
            CurveStyle::Dots => {
                let sc = Color::from_rgba(
                    controls.dot_stroke_color.r,
                    controls.dot_stroke_color.g,
                    controls.dot_stroke_color.b,
                    1.0,
                )
                .unwrap();
                for p in pts {
                    let r = len_fn(p);
                    let mut sb = match controls.dot_style.unwrap() {
                        DotStyle::Circle => ShapeBuilder::new().circle(p, r),
                        DotStyle::Square => ShapeBuilder::new().rect_cwh(p, pt(2.0 * r, 2.0 * r)),
                        DotStyle::Pearl => ShapeBuilder::new().pearl(
                            p,
                            r,
                            r,
                            controls.pearl_sides,
                            controls.pearl_smoothness,
                            &mut rng,
                        ),
                    };
                    if controls.stroke_width < 0.5 {
                        sb = sb.no_stroke();
                    } else {
                        sb = sb.stroke_weight(controls.stroke_width).stroke_color(sc)
                    }
                    sb.fill_color(c).build().draw(&mut canvas);
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
                    let lg = paint_lg(p.x, y0, p.x, y1, c, controls.grad_style.unwrap(), &mut rng);
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
    if controls.border {
        let border_color = palette[0].darken_fixed(0.35);
        ShapeBuilder::new()
            .rect_xywh(pt(0, 0), pt(canvas.width, canvas.height))
            .no_fill()
            .stroke_color(border_color)
            .stroke_weight(20.0)
            .build()
            .draw(&mut canvas);
    }
    canvas
}

pub async fn print(controls: Controls) {
    let canvas = draw(&controls, true);
    let dirs = UserDirs::new().unwrap();
    let dir = dirs.download_dir().unwrap();
    let path = format!(r"{}/{}", dir.to_string_lossy(), "k2");
    let mut num = 0;
    let mut sketch = PathBuf::from(format!(r"{path}_{num}"));
    sketch.set_extension("png");
    while sketch.exists() {
        num += 1;
        sketch = PathBuf::from(format!(r"{path}_{num}"));
        sketch.set_extension("png");
    }
    canvas.save_png(&sketch);
}
