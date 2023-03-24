use wassily::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    Both,
    Horizontal,
    Vertical,
}

impl Distribution<Dir> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dir {
        let index: u8 = rng.gen_range(0..3);
        match index {
            0 => Dir::Both,
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
                Dir::Both => "Both",
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
        Dir::Both => (cx * cx / (w * w) + cy * cy / (h * h)).sqrt(),
        Dir::Horizontal => cx / w,
        Dir::Vertical => cy / h,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExtrusionStyle {
    Expanding,
    Contracting,
    Constant,
    Varying,
    Noisy,
}

impl std::fmt::Display for ExtrusionStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExtrusionStyle::Constant => "Constant",
                ExtrusionStyle::Expanding => "Expanding",
                ExtrusionStyle::Contracting => "Contracting",
                ExtrusionStyle::Varying => "Varying",
                ExtrusionStyle::Noisy => "Noisy",
            }
        )
    }
}

impl Distribution<ExtrusionStyle> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ExtrusionStyle {
        let index: u8 = rng.gen_range(0..5);
        match index {
            0 => ExtrusionStyle::Constant,
            1 => ExtrusionStyle::Expanding,
            2 => ExtrusionStyle::Contracting,
            3 => ExtrusionStyle::Varying,
            4 => ExtrusionStyle::Noisy,
            _ => unreachable!(),
        }
    }
}

impl ExtrusionStyle {
    pub fn calc(self, w: f32, h: f32, r: f32, dir: Dir) -> Box<dyn Fn(Point) -> f32> {
        match self {
            ExtrusionStyle::Expanding => Box::new(expanding(w, h, r, dir)),
            ExtrusionStyle::Contracting => Box::new(contracting(w, h, r, dir)),
            ExtrusionStyle::Varying => Box::new(varying(w, h, r)),
            ExtrusionStyle::Constant => Box::new(constant(r)),
            ExtrusionStyle::Noisy => Box::new(noisy(w, h, r)),
        }
    }
}

fn expanding(w: f32, h: f32, r: f32, dir: Dir) -> impl Fn(Point) -> f32 {
    move |p| f32::max(20.0, indep(p, w, h, dir) * r)
}

fn contracting(w: f32, h: f32, r: f32, dir: Dir) -> impl Fn(Point) -> f32 {
    move |p| f32::max(15.0, (0.5 - indep(p, w, h, dir)) * r)
}

fn varying(w: f32, h: f32, r: f32) -> impl Fn(Point) -> f32 {
    move |p| {
        let opts = NoiseOpts::with_wh(w, h).scales(10.0);
        let nf = Perlin::default().set_seed(2);
        f32::max(25.0, (noise2d_01(nf, &opts, p.x, p.y)) * r / 2.0)
    }
}

fn noisy(w: f32, h: f32, r: f32) -> impl Fn(Point) -> f32 {
    move |p| {
        let opts = NoiseOpts::with_wh(w, h).scales(16.0);
        let nf = Perlin::default().set_seed(1);
        let k = noise2d_01(nf, &opts, p.x, p.y);
        k * r
    }
}

fn constant(r: f32) -> impl Fn(Point) -> f32 {
    move |_| r * 0.5
}

fn periodic(w: f32, h: f32, r: f32, scale: f32, min_size: f32) -> impl Fn(Point) -> f32 {
    move |p| {
        let opts = NoiseOpts::with_wh(w, h).scales(scale);
        let nf = Perlin::default().set_seed(98713);
        f32::max(min_size, (noise2d_01(nf, &opts, p.x, p.y)) * r)
    }
}
