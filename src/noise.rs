use wassily::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NoiseFunction {
    Fbm,
    Billow,
    Ridged,
    Value,
    Checkerboard,
    Cylinders,
    Worley,
    Curl,
}

impl Distribution<NoiseFunction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> NoiseFunction {
        let index: u8 = rng.gen_range(0..8);
        match index {
            0 => NoiseFunction::Fbm,
            1 => NoiseFunction::Checkerboard,
            2 => NoiseFunction::Cylinders,
            3 => NoiseFunction::Billow,
            4 => NoiseFunction::Value,
            5 => NoiseFunction::Ridged,
            6 => NoiseFunction::Worley,
            7 => NoiseFunction::Curl,
            _ => unreachable!(),
        }
    }
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
                NoiseFunction::Checkerboard => "Checkerboard",
                NoiseFunction::Cylinders => "Cylinders",
                NoiseFunction::Value => "Value",
                NoiseFunction::Worley => "Worley",
                NoiseFunction::Curl => "Curl",
            }
        )
    }
}
