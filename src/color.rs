use wassily::prelude::{palette::LuvHue, palette::Saturate, palette::Shade, *};

pub fn color_scale(color1: Color, color2: Color, n: u8) -> Vec<Color> {
    let c1 = Hsluv::from_color(&color1);
    let c2 = Hsluv::from_color(&color2);
    let hsl1 = c1.desaturate(0.5).lighten(0.5);
    let hsl2 = c2.saturate(0.5).darken(0.5);
    (0..n)
        .map(|p| {
            let t = p as f32 * 1.0 / (n - 1) as f32;
            let h = (1.0 - t) * hsl1.hue.to_positive_radians() + t * hsl2.hue.to_positive_radians();
            let s = (1.0 - t) * hsl1.saturation + t * hsl2.saturation;
            let l = (1.0 - t) * hsl1.l + t * hsl2.l;
            Hsluv::new(LuvHue::from_radians(h), s, l).to_color()
        })
        .collect()
}
