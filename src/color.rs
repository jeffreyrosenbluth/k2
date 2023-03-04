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

//   function saturate(c, s)
pub fn hex_to_color(hex: Vec<u32>) -> Vec<Color> {
    hex.iter()
        .map(|h| {
            let (r, g, b) = Srgb::from(*h).into_components();
            Color::from_rgba8(r, g, b, 255)
        })
        .collect::<Vec<Color>>()
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

pub fn color_line_black(color: Color) -> Vec<Color> {
    color_line(color, *BLACK, 6)
}

pub fn color_palette(index: u8) -> Vec<Color> {
    match index {
        0 => color_line_black(*SLATEGRAY),
        1 => color_line_black(*ROSYBROWN),
        2 => color_line_black(*CORNFLOWERBLUE),
        3 => color_line_black(*TOMATO),
        4 => color_line_black(*LIGHTCORAL),
        5 => color_line_black(*GOLD),
        6 => color_line_black(*PLUM),
        7 => color_line_black(*PALEVIOLETRED),
        8 => color_line_black(*MEDIUMSEAGREEN),
        9 => color_line_black(*YELLOWGREEN),
        10 => color_line_black(*MISTYROSE),
        11 => color_line_black(*CADETBLUE),
        _ => color_line_black(*SILVER),
    }
}

// pub fn color_palette(index: u8) -> Vec<Color> {
//     match index {
//         0 => hex_to_color(vec![0x1C4572, 0x84561B, 0x6D3E32, 0x0A0E20]),
//         1 => hex_to_color(vec![0x000000, 0x4682B4]),
//         2 => hex_to_color(vec![0x701C1C, 0x1A1717, 0x77806E]),
//         3 => hex_to_color(vec![0xA3B18A, 0x588157, 0x3A5A40, 0x344E41]),
//         4 => hex_to_color(vec![0xB7A635, 0x4E1406]),
//         5 => hex_to_color(vec![0x621708, 0x941B0C, 0xBC3908, 0xF6AA1C]),
//         6 => hex_to_color(vec![0xD9798B, 0x8C4962, 0x59364A, 0x594832]),
//         7 => hex_to_color(vec![0xBF2642, 0x731F2E, 0x400C16]),
//         8 => hex_to_color(vec![0x002B75, 0x862A23, 0xBD8878]),
//         9 => hex_to_color(vec![0xD9A404, 0xF2B988, 0xBF0303, 0x0D0D0D]),
//         10 => hex_to_color(vec![0x6A7AB2, 0xF27E9D, 0x454259, 0x9B8660]),
//         _ => hex_to_color(vec![0x000000, 0xE6E6E6, 0xA0A0A0]),
//     }
// }
