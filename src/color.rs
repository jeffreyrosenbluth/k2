use wassily::prelude::{palette::LuvHue, *};

pub fn expand_palette(palette: Vec<Color>) -> Vec<Color> {
    let mut result = palette.clone();
    let n = palette.len();
    for i in 0..n {
        for j in i + 1..n {
            let c = result[i].lerp(&result[j], 0.25);
            result.push(c);
        }
    }
    result
}

// hex_to_color converts a vector of hex values to a vector of Color
pub fn hex_to_color(hex: Vec<u32>) -> Vec<Color> {
    hex.iter()
        .map(|h| {
            let (r, g, b) = Srgb::from(*h).into_components();
            Color::from_rgba8(r, g, b, 255)
        })
        .collect::<Vec<Color>>()
}

pub enum ScaleDirection {
    Light,
    Dark,
}

pub fn color_scale(color: Color, n: u8, sd: ScaleDirection) -> Vec<Color> {
    let c = Hsluv::from_color(&color);
    let (s, l) = match sd {
        ScaleDirection::Light => (0.0, 100.0),
        ScaleDirection::Dark => (100.0, 0.0),
    };
    (0..n)
        .map(|p| {
            let t = p as f32 * 1.0 / (n - 1) as f32;
            let s = (1.0 - t) * c.saturation + t * s;
            let l = (1.0 - t) * c.l + t * l;
            Hsluv::new(c.hue, s, l).to_color()
        })
        .collect()
}

pub fn color_bi(color: Color) -> Vec<Color> {
    color_scale(color, 8, ScaleDirection::Dark)
}

pub fn color_line(color1: Color, color2: Color, n: u8) -> Vec<Color> {
    let c1 = Hsluv::from_color(&color1);
    let c2 = Hsluv::from_color(&color2);
    (0..n)
        .map(|p| {
            let t = p as f32 * 1.0 / (n - 1) as f32;
            let h = (1.0 - t) * c1.hue.to_positive_radians() + t * c2.hue.to_positive_radians();
            let s = (1.0 - t) * c1.saturation + t * c2.saturation;
            let l = (1.0 - t) * c1.l + t * c2.l;
            Hsluv::new(LuvHue::from_radians(h), s, l).to_color()
        })
        .collect()
}

pub fn color_palette(index: u8) -> Vec<Color> {
    match index {
        0 => color_bi(*SLATEGRAY),
        1 => color_bi(*ROSYBROWN),
        2 => color_bi(*CORNFLOWERBLUE),
        3 => color_bi(*TOMATO),
        4 => color_bi(*GOLD),
        5 => color_bi(*PLUM),
        6 => color_bi(*PALEVIOLETRED),
        7 => color_bi(*MEDIUMSEAGREEN),
        8 => color_bi(*YELLOWGREEN),
        9 => color_bi(*CADETBLUE),
        _ => color_bi(*SILVER),
    }
}
