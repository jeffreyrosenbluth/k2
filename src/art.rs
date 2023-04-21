use wassily::prelude::*;

use crate::background::*;
use crate::color::{color_palette, color_scale, ColorMode};
use crate::common::{Controls, CurveDirection, CurveStyle, HEIGHT, SEED, WIDTH};
use crate::dot::DotStyle;
use crate::field::Field;
use crate::gradient::paint_lg;
use crate::noise::*;

fn choose_flow(controls: &Controls, w: u32, h: u32) -> Field {
    let mut opts = NoiseOpts::with_wh(w, h)
        .scales(controls.noise_controls.noise_scale)
        .factor(controls.noise_controls.noise_factor);
    Field {
        noise_function: match controls
            .noise_controls
            .noise_function
            .expect("controls.noise_function cannot be None")
        {
            NoiseFunction::Fbm => Box::new(
                Fbm::<Perlin>::default()
                    .set_octaves(controls.fractal_controls.octaves as usize)
                    .set_persistence(controls.fractal_controls.persistence as f64)
                    .set_lacunarity(controls.fractal_controls.lacunarity as f64)
                    .set_frequency(controls.fractal_controls.frequency as f64),
            ),
            NoiseFunction::Billow => Box::new(
                Billow::<Perlin>::default()
                    .set_octaves(controls.fractal_controls.octaves as usize)
                    .set_lacunarity(controls.fractal_controls.lacunarity as f64)
                    .set_frequency(controls.fractal_controls.frequency as f64)
                    .set_persistence(controls.fractal_controls.persistence as f64),
            ),
            NoiseFunction::Ridged => Box::new(
                RidgedMulti::<Perlin>::default()
                    .set_octaves(controls.fractal_controls.octaves as usize)
                    .set_lacunarity(controls.fractal_controls.lacunarity as f64)
                    .set_frequency(controls.fractal_controls.frequency as f64)
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
            NoiseFunction::Curl => {
                let nf = Fbm::<Perlin>::default()
                    .set_octaves(controls.fractal_controls.octaves as usize)
                    .set_lacunarity(controls.fractal_controls.lacunarity as f64)
                    .set_frequency(controls.fractal_controls.frequency as f64)
                    .set_persistence(controls.fractal_controls.persistence as f64);
                Box::new(Curl::new(nf))
            }
            NoiseFunction::Magnet => {
                opts = NoiseOpts::default();
                let w = w as f32;
                let h = h as f32;
                Box::new(Magnet::new(vec![
                    pt(0.25 * w, 0.25 * h),
                    pt(0.25 * w, 0.75 * h),
                    pt(0.75 * w, 0.25 * h),
                    pt(0.75 * w, 0.75 * h),
                ]))
            }
            NoiseFunction::Gravity => {
                opts = NoiseOpts::default();
                let w = w as f32;
                let h = h as f32;
                Box::new(Curl::new(Magnet::new(vec![
                    pt(0.25 * w, 0.25 * h),
                    pt(0.25 * w, 0.75 * h),
                    pt(0.75 * w, 0.25 * h),
                    pt(0.75 * w, 0.75 * h),
                ])))
            }
            NoiseFunction::Sinusoidal => Box::new(Sinusoidal::new(
                controls.sin_controls.xfreq as f64,
                controls.sin_controls.yfreq as f64,
                controls.sin_controls.xexp as f64,
                controls.sin_controls.yexp as f64,
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
        Background::ColorGrain => {
            BG::color_grain(canvas.width, canvas.height, &mut rng, controls.grain_color)
        }
    };
    bg.canvas_bg(&mut canvas);

    let mut flow = choose_flow(controls, canvas.width, canvas.height);

    let starts = controls
        .location
        .expect("controls.location cannot be None")
        .starts(
            canvas.w_f32(),
            canvas.h_f32(),
            105.0 - controls.density,
            &mut rng,
        );

    let mut palette = match controls
        .color_mode_controls
        .mode
        .expect("controls.mode cannot be None")
    {
        ColorMode::Scale => Palette::new(color_scale(
            Color::from_rgba(
                controls.color_mode_controls.anchor1.r,
                controls.color_mode_controls.anchor1.g,
                controls.color_mode_controls.anchor1.b,
                1.0,
            )
            .unwrap(),
            Color::from_rgba(
                controls.color_mode_controls.anchor2.r,
                controls.color_mode_controls.anchor2.g,
                controls.color_mode_controls.anchor2.b,
                1.0,
            )
            .unwrap(),
            8,
        )),
        ColorMode::Palette => color_palette(controls.color_mode_controls.palette_choice.unwrap()),
    };

    let len_fn = if controls.curve_style == Some(CurveStyle::Dots) {
        controls.dot_controls.size_controls.size_fn.unwrap().calc(
            canvas.w_f32(),
            canvas.h_f32(),
            controls.dot_controls.size_controls.size,
            controls.dot_controls.size_controls.direction.unwrap(),
            controls.dot_controls.size_controls.size_scale,
            controls.dot_controls.size_controls.min_size,
        )
    } else {
        controls
            .extrude_controls
            .size_controls
            .size_fn
            .expect("controls.size_fn cannot be None")
            .calc(
                canvas.w_f32(),
                canvas.h_f32(),
                controls.extrude_controls.size_controls.size,
                controls
                    .extrude_controls
                    .size_controls
                    .direction
                    .expect("controls.direction cannot be None"),
                controls.extrude_controls.size_controls.size_scale,
                controls.extrude_controls.size_controls.min_size,
            )
    };

    for p in starts {
        let pts = match controls
            .curve_direction
            .expect("controls.curve_direction cannot be None")
        {
            CurveDirection::OneSided => flow.curve1(p.x, p.y),
            CurveDirection::TwoSided => flow.curve2(p.x, p.y),
        };
        let c = palette.rand_color();

        match controls
            .curve_style
            .expect("controls.curve_style cannot be None")
        {
            CurveStyle::Dots => {
                let sc = Color::from_rgba(
                    controls.dot_controls.dot_stroke_color.r,
                    controls.dot_controls.dot_stroke_color.g,
                    controls.dot_controls.dot_stroke_color.b,
                    1.0,
                )
                .unwrap();
                for p in pts {
                    let r = len_fn(p);
                    let mut sb = match controls
                        .dot_controls
                        .dot_style
                        .expect("controls.dot_style cannot be None")
                    {
                        DotStyle::Circle => ShapeBuilder::new().circle(p, r),
                        DotStyle::Square => ShapeBuilder::new().rect_cwh(p, pt(2.0 * r, 2.0 * r)),
                        DotStyle::Pearl => ShapeBuilder::new().pearl(
                            p,
                            r,
                            r,
                            controls.dot_controls.pearl_sides,
                            controls.dot_controls.pearl_smoothness,
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
                    let lg = paint_lg(
                        p.x,
                        y0,
                        p.x,
                        y1,
                        c,
                        controls
                            .extrude_controls
                            .grad_style
                            .expect("controls.extrude_controls.grad_style cannot be None"),
                        &mut rng,
                    );
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
