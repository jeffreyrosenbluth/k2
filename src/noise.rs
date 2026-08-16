#![allow(dead_code)]

use wassily::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NoiseControls {
    pub noise_function: Option<NoiseFunction>,
    pub noise_factor: f32,
    pub noise_scale: f32,
}

impl NoiseControls {
    pub fn new(noise_function: NoiseFunction, noise_scale: f32, noise_factor: f32) -> Self {
        Self {
            noise_function: Some(noise_function),
            noise_factor,
            noise_scale,
        }
    }

    pub fn set_noise_function(mut self, noise_function: NoiseFunction) -> Self {
        self.noise_function = Some(noise_function);
        self
    }

    pub fn set_noise_factor(mut self, noise_factor: f32) -> Self {
        self.noise_factor = noise_factor;
        self
    }

    pub fn set_noise_scale(mut self, noise_scale: f32) -> Self {
        self.noise_scale = noise_scale;
        self
    }
}

impl Default for NoiseControls {
    fn default() -> Self {
        Self {
            noise_function: Some(NoiseFunction::Fbm),
            noise_factor: 1.0,
            noise_scale: 4.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NoiseFunction {
    Fbm,
    BasicMulti,
    HybridMulti,
    Billow,
    Ridged,
    Value,
    Cylinders,
    Worley,
    Curl,
    Sinusoidal,
}

impl std::fmt::Display for NoiseFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NoiseFunction::Fbm => "Fbm",
                NoiseFunction::BasicMulti => "Basic Multi",
                NoiseFunction::HybridMulti => "Hybrid Multi",
                NoiseFunction::Billow => "Billow",
                NoiseFunction::Ridged => "Ridged",
                NoiseFunction::Cylinders => "Cylinders",
                NoiseFunction::Value => "Value",
                NoiseFunction::Worley => "Worley",
                NoiseFunction::Curl => "Curl",
                NoiseFunction::Sinusoidal => "Sinusoidal",
            }
        )
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
