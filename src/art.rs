use rand::RngCore;
use rayon::prelude::*;
use wassily::prelude::*;

use crate::background::*;
use crate::color::{color_scale, palette_colors, sample_colors, ColorBy, ColorMode};
use crate::common::{Controls, CurveDirection, CurveStyle, HEIGHT, SEED, WIDTH};
use crate::dot::DotStyle;
use crate::extrude::ExtrudeDirection;
use crate::field::Field;
use crate::gradient::paint_lg;
use crate::noise::*;

fn choose_flow(controls: &Controls, w: u32, h: u32) -> Field {
    let opts = NoiseOpts::with_wh(w, h)
        .scales(controls.noise_controls.noise_scale)
        .factor(controls.noise_controls.noise_factor);
    let fc = &controls.fractal_controls;
    let source = fc.source.unwrap_or(crate::noise::NoiseSource::Perlin);
    if source == crate::noise::NoiseSource::Worley {
        crate::noise::set_source_worley_config(controls.worley);
    }
    // A fractal generator over the chosen base noise, boxed.
    macro_rules! fractal {
        ($gen:ident) => {
            match source {
                crate::noise::NoiseSource::Perlin => Box::new(
                    $gen::<Perlin>::default()
                        .set_octaves(fc.octaves as usize)
                        .set_persistence(fc.persistence as f64)
                        .set_lacunarity(fc.lacunarity as f64)
                        .set_frequency(fc.frequency as f64),
                ) as Box<dyn NoiseFn<f64, 2>>,
                crate::noise::NoiseSource::Worley => Box::new(
                    $gen::<crate::noise::SourceWorley>::default()
                        .set_octaves(fc.octaves as usize)
                        .set_persistence(fc.persistence as f64)
                        .set_lacunarity(fc.lacunarity as f64)
                        .set_frequency(fc.frequency as f64),
                ) as Box<dyn NoiseFn<f64, 2>>,
            }
        };
    }
    let noise_function: Box<dyn NoiseFn<f64, 2>> = match controls
        .noise_controls
        .noise_function
        .expect("controls.noise_function cannot be None")
    {
            NoiseFunction::Fbm => fractal!(Fbm),
            NoiseFunction::BasicMulti => fractal!(BasicMulti),
            NoiseFunction::HybridMulti => fractal!(HybridMulti),
            NoiseFunction::Billow => fractal!(Billow),
            NoiseFunction::Ridged => fractal!(RidgedMulti),
            NoiseFunction::Value => Box::<Value>::default(),
            NoiseFunction::Worley => Box::new(crate::noise::build_worley(&controls.worley)),
            NoiseFunction::Cylinders => Box::new(
                TranslatePoint::new(
                    Cylinders::default()
                        .set_frequency(controls.fractal_controls.octaves as f64 / 2.0),
                )
                .set_x_translation(w as f64 / 2.0)
                .set_y_translation(h as f64 / 2.0),
            ),
            NoiseFunction::Curl => Box::new(Curl::new(fractal!(Fbm))),
            NoiseFunction::Image => {
                let noise = controls.image_noise.path.as_deref().and_then(|p| {
                    crate::imgnoise::cached_noise(
                        p,
                        controls
                            .image_noise
                            .color_map
                            .unwrap_or(crate::imgnoise::ColorMap::Lightness),
                        controls.image_noise.blur.max(0.0),
                        controls
                            .image_noise
                            .rotation
                            .unwrap_or(crate::imgnoise::Rotation::Deg0),
                    )
                });
                match noise {
                    Some(n) => Box::new(crate::imgnoise::SharedImgNoise(n)),
                    // No image chosen (or unreadable): a flat field.
                    None => Box::new(Constant::new(0.0)),
                }
            }
            NoiseFunction::Sinusoidal => Box::new(Sinusoidal::new(
                controls.sin_controls.xfreq as f64,
                controls.sin_controls.yfreq as f64,
                controls.sin_controls.xexp as f64,
                controls.sin_controls.yexp as f64,
            )),
    };
    // Optionally distort the field's input coordinates with Perlin turbulence.
    let noise_function: Box<dyn NoiseFn<f64, 2>> = if controls.turbulence.enabled {
        Box::new(
            Turbulence::<_, Perlin>::new(noise_function)
                .set_frequency(controls.turbulence.frequency as f64)
                .set_power(controls.turbulence.power as f64)
                .set_roughness(controls.turbulence.roughness as usize),
        )
    } else {
        noise_function
    };
    Field {
        noise_function,
        noise_opts: opts,
        step_size: controls.spacing,
        width: w,
        height: h,
        curve_length: controls.curve_length,
        speed: controls.speed,
        hide_ends: controls.hide_ends,
        // Rotate the whole flow with a rotated seed column, so the curves
        // stay perpendicular to the seed line.
        angle_offset: if controls.location == Some(crate::location::Location::Line) {
            -controls.column_angle.to_radians()
        } else {
            0.0
        },
    }
}

/// A color with its alpha scaled by the curve opacity.
fn fade(c: Color, alpha: f32) -> Color {
    Color::from_rgba(c.red(), c.green(), c.blue(), c.alpha() * alpha).unwrap()
}

fn gen_curve(flow: &Field, controls: &Controls, start: Point) -> Vec<Point> {
    match controls
        .curve_direction
        .expect("controls.curve_direction cannot be None")
    {
        CurveDirection::OneSided => flow.curve1(start.x, start.y),
        CurveDirection::TwoSided => flow.curve2(start.x, start.y),
    }
}

/// Fill the channel between two neighboring curves, leaving `strip_gap` of
/// it open; with AlongCurve coloring the fill glides along the strip.
fn paint_strip(
    controls: &Controls,
    a: &[Point],
    b: &[Point],
    c: Color,
    color_by: ColorBy,
    colors: &[Color],
    canvas: &mut Canvas,
) {
    let n = a.len().min(b.len());
    if n < 2 {
        return;
    }
    let gap = controls.strip_gap.clamp(0.0, 0.9);
    let (lo, hi) = (gap / 2.0, 1.0 - gap / 2.0);
    let lp = |i: usize, t: f32| {
        pt(
            a[i].x + t * (b[i].x - a[i].x),
            a[i].y + t * (b[i].y - a[i].y),
        )
    };
    // The strip is painted as a run of slightly overlapping quads rather
    // than one big polygon: diverging or crossing curve pairs would make a
    // single polygon self-intersect and cancel its own fill.
    for i in 0..n - 1 {
        let j = (i + 2).min(n - 1);
        let color = if color_by == ColorBy::AlongCurve {
            let t = i as f32 / (n - 2).max(1) as f32;
            sample_colors(colors, t)
        } else {
            c
        };
        let quad = [lp(i, lo), lp(j, lo), lp(j, hi), lp(i, hi)];
        Shape::new()
            .points(&quad)
            .fill_color(fade(color, controls.opacity))
            .no_stroke()
            .draw(canvas);
    }
}

#[allow(clippy::too_many_arguments)]
fn paint_curve(
    controls: &Controls,
    len_fn: &(dyn Fn(Point) -> f32 + Send + Sync),
    pts: &[Point],
    c: Color,
    color_by: ColorBy,
    colors: &[Color],
    rng: &mut SmallRng,
    canvas: &mut Canvas,
) {
    // For AlongCurve, the color glides through the palette along the curve:
    // `cycles` sweeps per curve, optionally mirrored at the turnaround, with
    // an optional per-curve random phase; otherwise every point uses the
    // curve color.
    let cycles = controls.color_mode_controls.along_cycles.max(0.01);
    let mirror = controls.color_mode_controls.along_mirror;
    let phase = if color_by == ColorBy::AlongCurve && controls.color_mode_controls.along_phase {
        rng.random_range(0.0..2.0f32)
    } else {
        0.0
    };
    let denom = (pts.len() - 1).max(1) as f32;
    let point_color = |i: usize| -> Color {
        if color_by == ColorBy::AlongCurve {
            let x = (i as f32 / denom) * cycles + phase;
            let t = if mirror {
                let y = x.rem_euclid(2.0);
                if y <= 1.0 {
                    y
                } else {
                    2.0 - y
                }
            } else if x > 0.0 && x.rem_euclid(1.0) == 0.0 {
                // Land exact cycle ends on the last color, so a single sweep
                // finishes the palette instead of wrapping to the first color.
                1.0
            } else {
                x.rem_euclid(1.0)
            };
            sample_colors(colors, t)
        } else {
            c
        }
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
            for (i, p) in pts.iter().enumerate() {
                let r = len_fn(*p);
                let mut sb = match controls
                    .dot_controls
                    .dot_style
                    .expect("controls.dot_style cannot be None")
                {
                    DotStyle::Circle => Shape::new().circle(*p, r),
                    DotStyle::Square => Shape::new().rect_cwh(*p, pt(2.0 * r, 2.0 * r)),
                    DotStyle::Pearl => Shape::new().pearl(
                        *p,
                        r,
                        r,
                        controls.dot_controls.pearl_sides,
                        controls.dot_controls.pearl_smoothness,
                        rng,
                    ),
                };
                if !controls.dot_controls.stroke || controls.stroke_width < 0.5 {
                    sb = sb.no_stroke();
                } else {
                    sb = sb
                        .stroke_weight(controls.stroke_width)
                        .stroke_color(fade(sc, controls.opacity))
                }
                sb.fill_color(fade(point_color(i), controls.opacity)).draw(canvas);
            }
        }
        CurveStyle::Line => {
            let max_jump = (4.0 * controls.spacing).max(20.0);
            if color_by == ColorBy::AlongCurve {
                // Per-segment strokes so the color can glide along the line.
                for i in 0..pts.len().saturating_sub(1) {
                    if pts[i].dist2(pts[i + 1]) > max_jump * max_jump {
                        continue;
                    }
                    Shape::new()
                        .line(pts[i], pts[i + 1])
                        .stroke_color(fade(point_color(i), controls.opacity))
                        .stroke_weight(controls.stroke_width)
                        .draw(canvas);
                }
            } else {
                // Split the polyline at any jump much larger than a step, so
                // a discontinuity in the point list never draws a stray chord.
                let mut seg_start = 0;
                for i in 0..pts.len() {
                    let broken = i + 1 == pts.len()
                        || pts[i].dist2(pts[i + 1]) > max_jump * max_jump;
                    if broken {
                        if i > seg_start {
                            Shape::new()
                                .points(&pts[seg_start..=i])
                                .no_fill()
                                .stroke_color(fade(c, controls.opacity))
                                .stroke_weight(controls.stroke_width)
                                .draw(canvas);
                        }
                        seg_start = i + 1;
                    }
                }
            }
        }
        // Strips are painted per neighbor pair in paint_strip.
        CurveStyle::Strips => {}
        CurveStyle::Extrusion => {
            let extrude_dir = controls
                .extrude_controls
                .direction
                .unwrap_or(ExtrudeDirection::Vertical);
            let grad_style = controls
                .extrude_controls
                .grad_style
                .expect("controls.extrude_controls.grad_style cannot be None");
            // Double blends toward a second palette color, drawn once per
            // curve so the ribbon shades consistently along its length.
            let color2 = if grad_style == crate::gradient::GradStyle::Double {
                // Scan forward from a random start for a color different from
                // the curve color, so the gradient never degenerates to a
                // single color. One rng draw either way, so pieces whose
                // draw already differed render exactly as before.
                let start = rng.random_range(0..colors.len());
                (0..colors.len())
                    .map(|k| colors[(start + k) % colors.len()])
                    .find(|&cand| cand != c)
                    .unwrap_or(c)
            } else {
                c
            };
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
                    point_color(i),
                    color2,
                    grad_style,
                    controls.opacity,
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

/// Render the artwork. `scale` multiplies the logical canvas size: 1.0 for
/// the display image, below 1.0 for fast previews, above 1.0 for print.
pub fn draw(controls: &Controls, scale: f32) -> Canvas {
    let w = controls.width.max(1);
    let h = controls.height.max(1);
    let aspect_ratio = w as f32 / h as f32;
    let mut ch = HEIGHT;
    let mut cw = WIDTH;
    if w >= h {
        ch = (WIDTH as f32 / aspect_ratio) as u32;
    } else {
        cw = (HEIGHT as f32 * aspect_ratio) as u32;
    }
    let mut canvas = Canvas::with_scale(cw, ch, scale);

    let mut rng = SmallRng::seed_from_u64(SEED);

    // Backgrounds render at physical resolution with scale-aware texture,
    // so grain and fiber keep the same relative size at any output size.
    let (pw, ph, ps) = (canvas.pixmap.width(), canvas.pixmap.height(), canvas.scale);
    let bg = match controls.background.unwrap() {
        Background::LightFiber => BG::light_fiber(pw, ph, ps),
        Background::LightGrain => {
            BG::light_grain(pw, ph, ps, controls.grain_amount, controls.grain_size, &mut rng)
        }
        Background::DarkGrain => {
            BG::dark_grain(pw, ph, ps, controls.grain_amount, controls.grain_size, &mut rng)
        }
        Background::DarkFiber => BG::dark_fiber(pw, ph, ps),
        Background::ColorGrain => BG::color_grain(
            pw,
            ph,
            ps,
            controls.grain_amount,
            controls.grain_size,
            &mut rng,
            controls.grain_color,
        ),
        Background::White => BG::solid(canvas.width(), canvas.height(), *WHITE),
        Background::Black => BG::solid(canvas.width(), canvas.height(), *BLACK),
        Background::Solid => BG::solid(
            canvas.width(),
            canvas.height(),
            Color::from_rgba8(
                controls.solid_color.r(),
                controls.solid_color.g(),
                controls.solid_color.b(),
                255,
            ),
        ),
    };
    bg.canvas_bg(&mut canvas);

    // The Field is rebuilt per render chunk below: noise 0.9's Worley holds an
    // Rc internally, so a single Field cannot be shared across threads.

    let sep = 105.0 - controls.density;
    let even = controls.location == Some(crate::location::Location::Even);
    let strips = controls.curve_style == Some(CurveStyle::Strips);
    let two_sided = controls.curve_direction == Some(CurveDirection::TwoSided);
    let starts = controls
        .location
        .expect("controls.location cannot be None")
        .starts(
            canvas.w_f32(),
            canvas.h_f32(),
            sep,
            controls.column_angle,
            controls.line_shift,
            &mut rng,
        );

    // Curves are pre-generated when seeding and growth are coupled (evenly
    // spaced streamlines) or when strips need the whole ordered set to pair
    // neighbors. Otherwise they are generated inside the render chunks.
    let pregen: Option<Vec<(Point, Vec<Point>)>> = if even {
        let flow = choose_flow(controls, canvas.width(), canvas.height());
        Some(flow.evenly_spaced(sep, two_sided))
    } else if strips {
        // End extension would misalign the point indices of neighboring
        // curves, so strips always use plain curves.
        let mut flow = choose_flow(controls, canvas.width(), canvas.height());
        flow.hide_ends = false;
        Some(
            starts
                .iter()
                .map(|p| (*p, gen_curve(&flow, controls, *p)))
                .collect(),
        )
    } else {
        None
    };
    let seeds: Vec<Point> = match &pregen {
        Some(v) => v.iter().map(|x| x.0).collect(),
        None => starts,
    };

    let colors: Vec<Color> = match controls
        .color_mode_controls
        .mode
        .expect("controls.mode cannot be None")
    {
        ColorMode::Scale => color_scale(
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
        ),
        ColorMode::Palette => palette_colors(controls.color_mode_controls.palette_choice.unwrap()),
    };
    let mut palette = Palette::new(colors.clone());
    let color_by = controls
        .color_mode_controls
        .color_by
        .unwrap_or(ColorBy::Random);
    // A field used only to sample flow angles or values for color assignment.
    let color_field = matches!(color_by, ColorBy::FlowAngle | ColorBy::NoiseValue)
        .then(|| choose_flow(controls, canvas.width(), canvas.height()));
    // A coarse, independent noise that carves the canvas into color patches.
    let region_noise = Fbm::<Perlin>::default().set_octaves(2);
    let region_opts = NoiseOpts::with_wh(canvas.width(), canvas.height())
        .scales(controls.color_mode_controls.region_scale);
    let region_colors = controls.color_mode_controls.region_colors.clamp(2, 12);
    let (cx, cy) = (canvas.w_f32() / 2.0, canvas.h_f32() / 2.0);

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
    let n_curves = seeds.len();
    let direction = |t: f32| {
        if controls.color_mode_controls.reverse {
            1.0 - t
        } else {
            t
        }
    };
    // Radial color maps each curve's distance through the distribution of
    // all start distances (its rank), so every part of the palette receives
    // an equal share of curves; a plain distance ratio would crowd nearly
    // everything into the outer colors, since area grows with radius.
    let radial_sorted: Vec<f32> = if color_by == ColorBy::Radial {
        let mut ds: Vec<f32> = seeds
            .iter()
            .map(|p| ((p.x - cx).powi(2) + (p.y - cy).powi(2)).sqrt())
            .collect();
        ds.sort_by(f32::total_cmp);
        ds
    } else {
        Vec::new()
    };
    let jobs: Vec<(usize, Point, Color, u64)> = seeds
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| {
            let c = match color_by {
                ColorBy::Random => palette.rand_color(),
                ColorBy::Cycle => colors[i % colors.len()],
                ColorBy::Order => {
                    sample_colors(&colors, i as f32 / (n_curves - 1).max(1) as f32)
                }
                ColorBy::PositionX => sample_colors(&colors, direction(p.x / canvas.w_f32())),
                ColorBy::PositionY => sample_colors(&colors, direction(p.y / canvas.h_f32())),
                ColorBy::Radial => {
                    let d = ((p.x - cx).powi(2) + (p.y - cy).powi(2)).sqrt();
                    // Points at the same distance share a rank, so rings of
                    // starts stay a single color.
                    let rank = radial_sorted.partition_point(|x| *x < d);
                    let t = rank as f32 / (radial_sorted.len() - 1).max(1) as f32;
                    sample_colors(&colors, direction(t))
                }
                ColorBy::FlowAngle => {
                    let f = color_field.as_ref().unwrap();
                    let a = noise2d(&f.noise_function, &f.noise_opts, p.x, p.y) * PI;
                    sample_colors(&colors, a.rem_euclid(TAU) / TAU)
                }
                ColorBy::NoiseValue => {
                    let f = color_field.as_ref().unwrap();
                    sample_colors(
                        &colors,
                        noise2d_01(&f.noise_function, &f.noise_opts, p.x, p.y),
                    )
                }
                ColorBy::Region => {
                    // Quantized rather than interpolated, so patches read as
                    // flat regions of distinct color, spread evenly through
                    // the palette.
                    let t = noise2d_01(&region_noise, &region_opts, p.x, p.y);
                    let bin = ((t * region_colors as f32) as u32).min(region_colors - 1);
                    sample_colors(&colors, bin as f32 / (region_colors - 1) as f32)
                }
                // Per-point colors are sampled in render_curve.
                ColorBy::AlongCurve => colors[0],
            };
            (i, p, c, rng.next_u64())
        })
        .collect();
    // Strips pair each curve with the next one from the ordered start list;
    // pairs whose seeds are far apart (grid column wraps, scattered
    // generators) are skipped.
    let pregen_curves: Option<Vec<Vec<Point>>> =
        pregen.map(|v| v.into_iter().map(|x| x.1).collect());
    let jobs: Vec<(usize, Point, Color, u64)> = if strips {
        let max_pair = (3.0 * sep.max(5.0)).powi(2);
        jobs.into_iter()
            .filter(|(i, p, _, _)| {
                *i + 1 < seeds.len() && p.dist2(seeds[*i + 1]) < max_pair
            })
            .collect()
    } else {
        jobs
    };
    let chunk_size = jobs
        .len()
        .div_ceil(rayon::current_num_threads())
        .max(1);
    let layers: Vec<Pixmap> = jobs
        .par_chunks(chunk_size)
        .map(|chunk| {
            let mut layer = Canvas::with_scale(canvas.width(), canvas.height(), canvas.scale);
            let flow = pregen_curves
                .is_none()
                .then(|| choose_flow(controls, canvas.width(), canvas.height()));
            for (i, p, c, seed) in chunk {
                let mut rng = SmallRng::seed_from_u64(*seed);
                if strips {
                    let curves = pregen_curves.as_ref().unwrap();
                    paint_strip(
                        controls,
                        &curves[*i],
                        &curves[*i + 1],
                        *c,
                        color_by,
                        &colors,
                        &mut layer,
                    );
                } else {
                    let generated;
                    let pts: &[Point] = match &pregen_curves {
                        Some(v) => &v[*i],
                        None => {
                            generated = gen_curve(flow.as_ref().unwrap(), controls, *p);
                            &generated
                        }
                    };
                    paint_curve(
                        controls,
                        len_fn.as_ref(),
                        pts,
                        *c,
                        color_by,
                        &colors,
                        &mut rng,
                        &mut layer,
                    );
                }
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
    canvas
}
