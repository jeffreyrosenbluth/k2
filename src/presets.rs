use crate::background::Background;
use crate::common::*;
use crate::gradient::GradStyle;
use crate::location::Location;
use crate::noise::NoiseFunction;
use crate::size::SizeFn;
use iced::Color;

pub fn rusty_ribbons() -> Controls {
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

pub fn ridges() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Dots),
        noise_function: Some(NoiseFunction::Ridged),
        spacing: 10.0,
        stroke_width: 0.5,
        curve_length: 55,
        size: 5.0,
        size_fn: Some(SizeFn::Periodic),
        size_scale: 3.0,
        min_size: 1.0,
        octaves: 2,
        density: 100.0,
        color1: Color::from_rgb8(111, 171, 181),
        background: Some(Background::DarkGrain),
        noise_scale: 3.0,
        noise_factor: 1.3,
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}

pub fn solar() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Line),
        noise_function: Some(NoiseFunction::Worley),
        location: Some(Location::Circle),
        border: false,
        noise_scale: 4.0,
        noise_factor: 1.0,
        spacing: 5.0,
        stroke_width: 1.0,
        curve_length: 165,
        density: 100.0,
        color1: Color::from_rgb8(0, 0, 0),
        color2: Color::from_rgb8(255, 255, 0),
        background: Some(Background::DarkGrain),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}

pub fn river_stones() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Dots),
        noise_function: Some(NoiseFunction::Magnet),
        location: Some(Location::Poisson),
        dot_style: Some(DotStyle::Pearl),
        size_fn: Some(SizeFn::Constant),
        size: 100.0,
        pearl_sides: 3,
        pearl_smoothness: 3,
        noise_scale: 3.0,
        noise_factor: 3.4,
        spacing: 100.0,
        stroke_width: 0.0,
        curve_length: 1,
        density: 35.0,
        color1: Color::from_rgb8(45, 10, 65),
        background: Some(Background::DarkClouds),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}

pub fn purple() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Extrusion),
        noise_function: Some(NoiseFunction::Curl),
        location: Some(Location::Halton),
        spacing: 1.0,
        stroke_width: 2.0,
        curve_length: 200,
        size: 80.0,
        size_fn: Some(SizeFn::Constant),
        noise_scale: 3.0,
        noise_factor: 1.0,
        octaves: 1,
        density: 72.0,
        color1: Color::from_rgb8(121, 72, 141),
        color2: Color::from_rgb8(71, 76, 141),
        grad_style: Some(GradStyle::None),
        background: Some(Background::Clouds),
        width: "1000".to_string(),
        height: "1200".to_string(),
        ..Default::default()
    }
}

pub fn canyon() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Line),
        noise_function: Some(NoiseFunction::Fbm),
        location: Some(Location::Poisson),
        noise_scale: 3.0,
        noise_factor: 2.0,
        octaves: 6,
        spacing: 5.0,
        stroke_width: 2.5,
        curve_length: 75,
        density: 100.0,
        color1: Color::from_rgb8(108, 82, 42),
        color2: Color::from_rgb8(203, 137, 137),
        background: Some(Background::DarkGrain),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}

pub fn stripes() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Extrusion),
        noise_function: Some(NoiseFunction::Fbm),
        noise_scale: 4.0,
        noise_factor: 1.0,
        location: Some(Location::Rand),
        spacing: 20.0,
        stroke_width: 12.5,
        curve_length: 150,
        size: 200.0,
        size_fn: Some(SizeFn::Periodic),
        min_size: 25.0,
        size_scale: 5.0,
        octaves: 6,
        persistence: 0.3,
        density: 65.0,
        color1: Color::from_rgb8(70, 185, 25),
        color2: Color::from_rgb8(50, 50, 50),
        grad_style: Some(GradStyle::None),
        background: Some(Background::DarkClouds),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}

pub fn splat() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Dots),
        noise_function: Some(NoiseFunction::Fbm),
        noise_scale: 2.0,
        noise_factor: 1.0,
        location: Some(Location::Halton),
        spacing: 7.0,
        stroke_width: 0.0,
        curve_length: 50,
        size: 40.0,
        size_fn: Some(SizeFn::Periodic),
        min_size: 6.0,
        size_scale: 10.0,
        dot_style: Some(DotStyle::Pearl),
        pearl_sides: 5,
        pearl_smoothness: 3,
        octaves: 1,
        density: 60.0,
        color1: Color::from_rgb8(30, 25, 180),
        color2: Color::from_rgb8(90, 175, 185),
        grad_style: Some(GradStyle::None),
        background: Some(Background::Grain),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}

pub fn tubes() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Dots),
        noise_function: Some(NoiseFunction::Value),
        location: Some(Location::Lissajous),
        spacing: 1.0,
        stroke_width: 1.0,
        curve_length: 0,
        size: 235.0,
        size_fn: Some(SizeFn::Periodic),
        size_scale: 3.0,
        min_size: 10.0,
        density: 65.0,
        color1: Color::from_rgb8(187, 42, 20),
        color2: Color::from_rgb8(155, 21, 48),
        background: Some(Background::DarkClouds),
        width: "1000".to_string(),
        height: "1200".to_string(),
        ..Default::default()
    }
}
