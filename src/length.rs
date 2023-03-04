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
pub enum Len {
    Expanding,
    Contracting,
    Constant,
    Varying,
    Noisy,
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
                Len::Noisy => "Noisy",
            }
        )
    }
}

impl Distribution<Len> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Len {
        let index: u8 = rng.gen_range(0..5);
        match index {
            0 => Len::Constant,
            1 => Len::Expanding,
            2 => Len::Contracting,
            3 => Len::Varying,
            4 => Len::Noisy,
            _ => unreachable!(),
        }
    }
}

impl Len {
    pub fn calc(self, w: f32, h: f32, r: f32, dir: Dir) -> Box<dyn Fn(Point) -> f32> {
        match self {
            Len::Expanding => Box::new(expanding(w, h, r, dir)),
            Len::Contracting => Box::new(contracting(w, h, r, dir)),
            Len::Varying => Box::new(varying(w, h, r)),
            Len::Constant => Box::new(constant(r)),
            Len::Noisy => Box::new(noisy(w, h, r)),
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
