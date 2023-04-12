use wassily::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NoiseFunction {
    Fbm,
    Billow,
    Ridged,
    Value,
    Cylinders,
    Worley,
    Curl,
    Magnet,
    Gravity,
    Sinusoidal,
}

impl std::fmt::Display for NoiseFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NoiseFunction::Fbm => "Fbm",
                NoiseFunction::Billow => "Billow",
                NoiseFunction::Ridged => "Ridged",
                NoiseFunction::Cylinders => "Cylinders",
                NoiseFunction::Value => "Value",
                NoiseFunction::Worley => "Worley",
                NoiseFunction::Curl => "Curl",
                NoiseFunction::Magnet => "Magnet",
                NoiseFunction::Gravity => "Gravity",
                NoiseFunction::Sinusoidal => "Sinusoidal",
            }
        )
    }
}

pub struct Magnet {
    sinks: Vec<Point>,
}

impl Magnet {
    pub fn new(sinks: Vec<Point>) -> Self {
        Self { sinks }
    }
}

impl NoiseFn<f64, 2> for Magnet {
    fn get(&self, point: [f64; 2]) -> f64 {
        let mut p = Point::zero();
        let mut min_sink = f64::MAX;
        for s in &self.sinks {
            let d = pt(point[0], point[1]).dist2(pt(s.x, s.y)) as f64;
            if d < min_sink {
                min_sink = d;
                p = *s;
            }
        }
        if min_sink == f64::MAX {
            return 0.0;
        }
        (p.y as f64 - point[1]).atan2(p.x as f64 - point[0]) / std::f64::consts::PI
    }
}

pub struct Sinusoidal {
    x_freq: f64,
    y_freq: f64,
    x_exp: f64,
    y_exp: f64,
}

impl Sinusoidal {
    pub fn new(x_freq: f64, y_freq: f64, x_exp: f64, y_exp: f64) -> Self {
        Self {
            x_freq,
            y_freq,
            x_exp,
            y_exp,
        }
    }
}

impl NoiseFn<f64, 2> for Sinusoidal {
    fn get(&self, point: [f64; 2]) -> f64 {
        std::f64::consts::PI
            * (2.0
                + (self.x_freq * point[0]).sin().powf(self.x_exp)
                + (self.y_freq * point[1]).sin().powf(self.y_exp))
    }
}
