use crate::background::Background;
use crate::common::*;
use crate::gradient::GradStyle;
use iced::Color;

pub fn xtrusion_fbm() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Extrusion),
        spacing: 2.0,
        stroke_width: 4.0,
        curve_length: 175,
        size: 200.0,
        octaves: 1,
        density: 65.0,
        color1: Color::from_rgb8(185, 95, 25),
        grad_style: Some(GradStyle::Fiber),
        background: Some(Background::Grain),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}
