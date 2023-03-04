use wassily::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GradStyle {
    None,
    Light,
    Dark,
    Fiber,
}

impl Distribution<GradStyle> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GradStyle {
        let index: u8 = rng.gen_range(0..4);
        match index {
            0 => GradStyle::None,
            1 => GradStyle::Light,
            2 => GradStyle::Dark,
            3 => GradStyle::Fiber,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for GradStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GradStyle::None => "None",
                GradStyle::Light => "Light",
                GradStyle::Dark => "Dark",
                GradStyle::Fiber => "Fiber",
            }
        )
    }
}

pub fn paint_lg<'a>(x0: f32, y0: f32, x1: f32, y1: f32, color1: Color, caps: u8) -> Paint<'a> {
    let mut rng = SmallRng::from_entropy();
    let color0 = Color::from_rgba8(230, 230, 230, 255);
    let stops = match caps {
        2 => vec![
            GradientStop::new(0.0, *WHITE),
            GradientStop::new(rng.gen_range(0.7..1.0), color1),
        ],
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
            GradientStop::new(0.8, color1),
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
