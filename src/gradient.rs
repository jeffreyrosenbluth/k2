use rand::RngCore;
use wassily::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GradStyle {
    Plain,
    Double,
    Light,
    Dark,
    Fiber,
    LightFiber,
    DarkFiber,
}

impl std::fmt::Display for GradStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GradStyle::Plain => "Plain",
                GradStyle::Double => "Double",
                GradStyle::Light => "Light",
                GradStyle::Dark => "Dark",
                GradStyle::Fiber => "Fiber",
                GradStyle::LightFiber => "LightFiber",
                GradStyle::DarkFiber => "DarkFiber",
            }
        )
    }
}

pub fn paint_lg<'a, R: RngCore>(
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    color1: Color,
    color2: Color,
    grad_style: GradStyle,
    alpha: f32,
    rng: &mut R,
) -> Paint<'a> {
    use GradStyle::*;
    let stop = |p: f32, c: Color| {
        GradientStop::new(
            p,
            Color::from_rgba(c.red(), c.green(), c.blue(), c.alpha() * alpha).unwrap(),
        )
    };
    let color0 = Color::from_rgba8(230, 230, 230, 255);
    let stops = match grad_style {
        LightFiber => vec![
            stop(0.0, *WHITE),
            stop(rng.random_range(0.7..1.0), color1),
            stop(1.0, *WHITE),
        ],
        DarkFiber => vec![
            stop(0.0, Color::from_rgba8(30, 30, 30, 255)),
            stop(rng.random_range(0.05..0.25), *WHITE),
            stop(rng.random_range(0.7..1.0), color1),
            stop(1.0, Color::from_rgba8(30, 30, 30, 255)),
        ],
        Fiber => vec![
            stop(0.0, *WHITE),
            stop(rng.random_range(0.7..0.9), color1),
        ],
        Dark => vec![
            stop(0.0, Color::from_rgba8(30, 30, 30, 255)),
            stop(0.125, color0),
            stop(0.875, color1),
            stop(1.0, Color::from_rgba8(30, 30, 30, 255)),
        ],
        Light => vec![
            stop(0.0, *WHITE),
            stop(0.125, color0),
            stop(0.875, color1),
            stop(1.0, *WHITE),
        ],
        Plain => vec![
            stop(0.0, color0),
            stop(0.8, color1),
        ],
        Double => vec![
            stop(0.0, color2),
            stop(0.85, color1),
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
