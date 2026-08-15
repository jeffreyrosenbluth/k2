use rand::RngCore;
use rayon::prelude::*;
use wassily::prelude::*;

use crate::background::*;
use crate::color::{color_palette, color_scale, ColorMode};
use crate::common::{Controls, CurveDirection, CurveStyle, HEIGHT, SEED, WIDTH};
use crate::dot::DotStyle;
use crate::extrude::ExtrudeDirection;
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

fn render_curve(
    controls: &Controls,
    flow: &Field,
    len_fn: &(dyn Fn(Point) -> f32 + Send + Sync),
    start: Point,
    c: Color,
    rng: &mut SmallRng,
    canvas: &mut Canvas,
) {
    let pts = match controls
        .curve_direction
        .expect("controls.curve_direction cannot be None")
    {
        CurveDirection::OneSided => flow.curve1(start.x, start.y),
        CurveDirection::TwoSided => flow.curve2(start.x, start.y),
    };

    match controls
        .curve_style
        .expect("controls.curve_style cannot be None")
    {
        CurveStyle::Dots => {
            let sc = Color::from_rgba8(
                controls.dot_controls.dot_stroke_color.r(),
                controls.dot_controls.dot_stroke_color.g(),
                controls.dot_controls.dot_stroke_color.b(),
                255,
            );
            for p in pts {
                let r = len_fn(p);
                let mut sb = match controls
                    .dot_controls
                    .dot_style
                    .expect("controls.dot_style cannot be None")
                {
                    DotStyle::Circle => Shape::new().circle(p, r),
                    DotStyle::Square => Shape::new().rect_cwh(p, pt(2.0 * r, 2.0 * r)),
                    DotStyle::Pearl => Shape::new().pearl(
                        p,
                        r,
                        r,
                        controls.dot_controls.pearl_sides,
                        controls.dot_controls.pearl_smoothness,
                        rng,
                    ),
                };
                if controls.stroke_width < 0.5 {
                    sb = sb.no_stroke();
                } else {
                    sb = sb.stroke_weight(controls.stroke_width).stroke_color(sc)
                }
                sb.fill_color(c).draw(canvas);
            }
        }
        CurveStyle::Line => Shape::new()
            .points(&pts)
            .no_fill()
            .stroke_color(c)
            .stroke_weight(controls.stroke_width)
            .draw(canvas),
        CurveStyle::Extrusion => {
            let extrude_dir = controls
                .extrude_controls
                .direction
                .unwrap_or(ExtrudeDirection::Vertical);
            for (i, p) in pts.iter().enumerate() {
                let r = len_fn(*p);
                // Half-extent of the extruded line: along the y-axis, the
                // x-axis, or the normal to the curve at this point (estimated
                // from the neighboring points).
                let (dx, dy) = match extrude_dir {
                    ExtrudeDirection::Vertical => (0.0, r),
                    ExtrudeDirection::Horizontal => (r, 0.0),
                    ExtrudeDirection::Normal => {
                        let prev = pts[i.saturating_sub(1)];
                        let next = pts[(i + 1).min(pts.len() - 1)];
                        let tx = next.x - prev.x;
                        let ty = next.y - prev.y;
                        let len = (tx * tx + ty * ty).sqrt();
                        if len < f32::EPSILON {
                            (0.0, r)
                        } else {
                            (-ty / len * r, tx / len * r)
                        }
                    }
                };
                let (x0, y0) = (p.x - dx, p.y - dy);
                let (x1, y1) = (p.x + dx, p.y + dy);
                let lg = paint_lg(
                    x0,
                    y0,
                    x1,
                    y1,
                    c,
                    controls
                        .extrude_controls
                        .grad_style
                        .expect("controls.extrude_controls.grad_style cannot be None"),
                    rng,
                );
                Shape::new()
                    .line(pt(x0, y0), pt(x1, y1))
                    .stroke_weight(controls.stroke_width)
                    .stroke_paint(&lg)
                    .draw(canvas);
            }
        }
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
        Background::LightFiber => BG::light_fiber(canvas.width(), canvas.height()),
        Background::LightGrain => BG::light_grain(canvas.width(), canvas.height(), &mut rng),
        Background::DarkGrain => BG::dark_grain(canvas.width(), canvas.height(), &mut rng),
        Background::DarkFiber => BG::dark_fiber(canvas.width(), canvas.height()),
        Background::ColorGrain => BG::color_grain(
            canvas.width(),
            canvas.height(),
            &mut rng,
            controls.grain_color,
        ),
    };
    bg.canvas_bg(&mut canvas);

    // The Field is rebuilt per render chunk below: noise 0.9's Worley holds an
    // Rc internally, so a single Field cannot be shared across threads.

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
            Color::from_rgba8(
                controls.color_mode_controls.anchor1.r(),
                controls.color_mode_controls.anchor1.g(),
                controls.color_mode_controls.anchor1.b(),
                255,
            ),
            Color::from_rgba8(
                controls.color_mode_controls.anchor2.r(),
                controls.color_mode_controls.anchor2.g(),
                controls.color_mode_controls.anchor2.b(),
                255,
            ),
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

    // Assign per-curve colors and rng seeds sequentially so the result is
    // deterministic, then rasterize chunks of curves on separate threads into
    // transparent layers, composited in order to preserve overlap semantics.
    let jobs: Vec<(Point, Color, u64)> = starts
        .into_iter()
        .map(|p| (p, palette.rand_color(), rng.next_u64()))
        .collect();
    let chunk_size = jobs
        .len()
        .div_ceil(rayon::current_num_threads())
        .max(1);
    let layers: Vec<Pixmap> = jobs
        .par_chunks(chunk_size)
        .map(|chunk| {
            let mut layer = Canvas::with_scale(canvas.width(), canvas.height(), canvas.scale);
            let flow = choose_flow(controls, canvas.width(), canvas.height());
            for (p, c, seed) in chunk {
                let mut rng = SmallRng::seed_from_u64(*seed);
                render_curve(controls, &flow, len_fn.as_ref(), *p, *c, &mut rng, &mut layer);
            }
            layer.pixmap
        })
        .collect();
    for layer in layers {
        canvas.pixmap.draw_pixmap(
            0,
            0,
            layer.as_ref(),
            &PixmapPaint::default(),
            Transform::identity(),
            None,
        );
    }
    if controls.border {
        let border_color = palette[0].darken_fixed(0.35);
        Shape::new()
            .rect_xywh(pt(0, 0), pt(canvas.width(), canvas.height()))
            .no_fill()
            .stroke_color(border_color)
            .stroke_weight(20.0)
            .draw(&mut canvas);
    }
    canvas
}
