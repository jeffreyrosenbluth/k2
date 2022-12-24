use wassily::prelude::*;

use crate::common::{Controls, HEIGHT, WIDTH};
use crate::field::*;

pub fn draw(controls: &Controls, scale: f32) -> Canvas {
    let mut canvas = Canvas::with_scale(WIDTH, HEIGHT, scale);
    let step = if controls.spaced { 3.0 } else { 1.0 };
    canvas.fill(*WHITE);
    let bg = BG::new(canvas.width, canvas.height);
    bg.canvas_bg(&mut canvas);
    let max_length = 250;
    let fbm = Fbm::<Perlin>::default().set_octaves(controls.octaves as usize);
    let cyl = Cylinders::default();
    let opts = NoiseOpts::with_wh(canvas.width(), canvas.height())
        .scales(controls.noise_scale)
        .factor(controls.noise_factor);
    let mut flow = Field {
        noise_function: if controls.curl {
            Box::new(cyl)
        } else {
            Box::new(fbm)
        },
        noise_opts: opts,
        step_size: step,
        width: canvas.width(),
        height: canvas.height(),
        max_length,
    };
    let starts =
        controls
            .location
            .unwrap()
            .starts(canvas.w_f32(), canvas.h_f32(), controls.grid_sep);
    let mut palette = Palette::new(expand_palette(color_palette(controls.palette_num)));
    palette.rotate_hue(controls.hue);
    let len_fn = controls.len_type.unwrap().calc(
        canvas.w_f32(),
        canvas.h_f32(),
        controls.len_size,
        controls.len_freq,
        controls.len_dir.unwrap(),
    );
    for p in starts.iter() {
        let highlight = match controls.cap.unwrap() {
            Cap::None => 2,
            Cap::Light => 4,
            Cap::Dark => 3,
        };
        let pts = flow.curve(p.x, p.y);
        let c = palette.rand_color();
        for p in pts {
            let r = len_fn(p);
            let lg = paint_lg(p.x, p.y - r, p.x, p.y + r, c, highlight);
            ShapeBuilder::new()
                .line(pt(p.x, p.y - r), pt(p.x, p.y + r))
                .stroke_weight(1.0)
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Cap {
    None,
    Light,
    Dark,
}

impl Distribution<Cap> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cap {
        let index: u8 = rng.gen_range(0..3);
        match index {
            0 => Cap::None,
            1 => Cap::Light,
            2 => Cap::Dark,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Cap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cap::None => "None",
                Cap::Light => "Light",
                Cap::Dark => "Dark",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    Circle,
    Horizontal,
    Vertical,
}

impl Distribution<Dir> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dir {
        let index: u8 = rng.gen_range(0..3);
        match index {
            0 => Dir::Circle,
            1 => Dir::Horizontal,
            2 => Dir::Vertical,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::Circle => "Circle",
                Dir::Horizontal => "Horizontal",
                Dir::Vertical => "Vertical",
            }
        )
    }
}

fn indep(p: Point, w: f32, h: f32, dir: Dir) -> f32 {
    let cx = (p.x - w / 2.0).abs();
    let cy = (p.y - h / 2.0).abs();
    match dir {
        Dir::Circle => (cx * cx / (w * w) + cy * cy / (h * h)).sqrt(),
        Dir::Horizontal => cx / w,
        Dir::Vertical => cy / h,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Len {
    Expanding,
    Contracting,
    Constant,
    Varying,
}

impl std::fmt::Display for Len {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Len::Constant => "Constant",
                Len::Expanding => "Expanding",
                Len::Contracting => "Contracting",
                Len::Varying => "Varying",
            }
        )
    }
}

impl Distribution<Len> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Len {
        let index: u8 = rng.gen_range(0..4);
        match index {
            0 => Len::Constant,
            1 => Len::Expanding,
            2 => Len::Contracting,
            3 => Len::Varying,
            _ => unreachable!(),
        }
    }
}

impl Len {
    fn calc(self, w: f32, h: f32, r: f32, freq: f32, dir: Dir) -> Box<dyn Fn(Point) -> f32> {
        match self {
            Len::Expanding => Box::new(expanding(w, h, r, dir)),
            Len::Contracting => Box::new(contracting(w, h, r, dir)),
            Len::Varying => Box::new(varying(w, h, r, dir, freq)),
            Len::Constant => Box::new(constant(r)),
        }
    }
}

fn expanding(w: f32, h: f32, r: f32, dir: Dir) -> impl Fn(Point) -> f32 {
    move |p| indep(p, w, h, dir) * r
}

fn contracting(w: f32, h: f32, r: f32, dir: Dir) -> impl Fn(Point) -> f32 {
    move |p| (0.5 - indep(p, w, h, dir)) * r
}

fn varying(w: f32, h: f32, r: f32, dir: Dir, freq: f32) -> impl Fn(Point) -> f32 {
    move |p| ((freq * TAU * indep(p, w, h, dir)).sin() + 1.25) * 0.25 * r
}

fn constant(r: f32) -> impl Fn(Point) -> f32 {
    move |_| r / 2.0
}

pub async fn print(controls: Controls, scale: f32) {
    let canvas = draw(&controls, scale);
    let name = format!("./output/{}.png", "image");
    canvas.save_png(name);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Location {
    Grid,
    Rand,
    Halton,
    Poisson,
    Circle,
    Trig,
}

impl Location {
    fn starts(&self, w: f32, h: f32, sep: f32) -> Vec<Point> {
        let mut pts = Vec::new();
        let mut rng = SmallRng::from_entropy();
        match &self {
            Location::Grid => {
                let mut i = 0.0;
                let mut j;
                while i <= w {
                    j = 0.0;
                    while j <= h {
                        pts.push(pt(i, j));
                        j += sep;
                    }
                    i += sep;
                }
            }
            Location::Rand => {
                let n = (w * h) / (sep * sep);
                for _ in 0..n as u32 {
                    pts.push(pt(rng.gen_range(0.0..w), rng.gen_range(0.0..h)));
                }
            }
            Location::Halton => {
                let seed: u64 = rng.gen();
                let n = (w * h) / (sep * sep);
                pts = halton_23(w, h, n as u32, seed)
            }
            Location::Poisson => pts = poisson_disk(w, h, sep / 1.2, 0),
            Location::Circle => {
                let cx = w / 2.0;
                let cy = h / 2.0;
                let radii = vec![w / 6.0, w / 3.5, w / 2.5];
                for r in radii {
                    let delta = 0.5 * sep / r;
                    let mut theta = 0.0;
                    while theta <= TAU {
                        pts.push(pt(cx + r * theta.cos(), cy + r * theta.sin()));
                        theta += delta;
                    }
                }
            }
            Location::Trig => {
                let n = (w * h) / (sep * sep);
                let p1 = if rng.gen_bool(0.5) { 3 } else { 1 };
                let p2 = if rng.gen_bool(0.5) { 3 } else { 1 };
                let f1 = rng.gen_range(1.0..=13.0);
                let f2 = rng.gen_range(1.0..=13.0);
                for i in 0..n as u32 {
                    let x = (w / 3.0) * (f1 * TAU * i as f32 / n as f32).sin().powi(p1);
                    let y = (h / 3.0) * (f2 * TAU * i as f32 / n as f32).sin().powi(p2);
                    pts.push(pt(x + w / 2.0, y + h / 2.0));
                }
            }
        }
        pts
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Location::Grid => "Grid",
                Location::Rand => "Rand",
                Location::Halton => "Halton",
                Location::Poisson => "Poisson",
                Location::Circle => "Circle",
                Location::Trig => "Trig",
            }
        )
    }
}

impl Distribution<Location> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Location {
        let index: u8 = rng.gen_range(0..6);
        match index {
            0 => Location::Grid,
            1 => Location::Rand,
            2 => Location::Halton,
            3 => Location::Poisson,
            4 => Location::Circle,
            5 => Location::Trig,
            _ => unreachable!(),
        }
    }
}

pub struct BG(Canvas);

impl BG {
    pub fn new(width: u32, height: u32) -> Self {
        let mut canvas = Canvas::new(width, height);
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
            FilterQuality::Bicubic,
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

pub fn paint_lg<'a>(x0: f32, y0: f32, x1: f32, y1: f32, color1: Color, caps: u8) -> Paint<'a> {
    let color0 = Color::from_rgba8(230, 230, 230, 255);
    let stops = match caps {
        3 => vec![
            GradientStop::new(0.0, color0),
            GradientStop::new(0.875, color1),
            GradientStop::new(1.0, Color::from_rgba8(30, 30, 30, 255)),
        ],
        4 => vec![
            GradientStop::new(0.0, *WHITE),
            GradientStop::new(0.125, color0),
            GradientStop::new(0.875, color1),
            GradientStop::new(1.0, *WHITE),
        ],
        _ => vec![
            GradientStop::new(0.0, color0),
            GradientStop::new(1.0, color1),
        ],
    };
    let lg = LinearGradient::new(
        pt(x0, y0),
        pt(x1, y1),
        stops,
        SpreadMode::Pad,
        Transform::identity(),
    )
    .unwrap();
    paint_shader(lg)
}

fn expand_palette(palette: Vec<Color>) -> Vec<Color> {
    let mut result = palette.clone();
    let n = palette.len();
    for i in 0..n {
        for j in i..n {
            let c = result[i].lerp(&result[j], 0.5);
            result.push(c);
        }
    }
    return result;
}

//   function saturate(c, s)
fn hex_to_color(hex: Vec<u32>) -> Vec<Color> {
    hex.iter()
        .map(|h| {
            let (r, g, b) = Srgb::from(*h).into_components();
            Color::from_rgba8(r, g, b, 255)
        })
        .collect::<Vec<Color>>()
}

fn color_palette(index: u8) -> Vec<Color> {
    match index {
        0 => hex_to_color(vec![
            0x1C4572, 0x84561B, 0x010101, 0x607994, 0x6D3E32, 0x0A0E20,
        ]),
        1 => hex_to_color(vec![0x003566, 0x000000]),
        2 => hex_to_color(vec![0x703030, 0x010101, 0x7E827A]),
        3 => hex_to_color(vec![0x283618, 0xBC6C25]),
        4 => hex_to_color(vec![0xA3B18A, 0x588157, 0x3A5A40, 0x344E41]),
        5 => hex_to_color(vec![0xB7A635, 0x4E1406, 0x704514]),
        6 => hex_to_color(vec![0x621708, 0x941B0C, 0xBc3908, 0xF6AA1C]),
        7 => hex_to_color(vec![0xD9798B, 0x8C4962, 0x59364A, 0x594832]),
        8 => hex_to_color(vec![0xBF2642, 0x731F2E, 0x400C16]),
        _ => hex_to_color(vec![0x000000, 0xFFFFFF]),
    }
}
