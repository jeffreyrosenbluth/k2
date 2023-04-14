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
        size_controls: SizeControls::default().set_size(200.0),
        fractal_controls: FractalControls::default().set_octaves(1),
        density: 65.0,
        color1: Color::from_rgb8(185, 95, 25),
        grad_style: Some(GradStyle::Fiber),
        background: Some(Background::LightGrain),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}

pub fn ridges() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Dots),
        noise_controls: NoiseControls::new(NoiseFunction::Ridged, 3.0, 1.3),
        spacing: 10.0,
        stroke_width: 0.5,
        curve_length: 55,
        size_controls: SizeControls::default()
            .set_size(5.0)
            .set_size_fn(Some(SizeFn::Periodic))
            .set_size_scale(3.0)
            .set_min_size(1.0),
        fractal_controls: FractalControls::default().set_octaves(2),
        density: 100.0,
        color1: Color::from_rgb8(111, 171, 181),
        background: Some(Background::DarkGrain),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}

pub fn solar() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Line),
        noise_controls: NoiseControls::new(NoiseFunction::Worley, 4.0, 1.3),
        location: Some(Location::Circle),
        border: false,
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
        noise_controls: NoiseControls::new(NoiseFunction::Magnet, 3.0, 3.4),
        location: Some(Location::Poisson),
        dot_style: Some(DotStyle::Pearl),
        size_controls: SizeControls::default()
            .set_size_fn(Some(SizeFn::Constant))
            .set_size(100.0),
        pearl_sides: 3,
        pearl_smoothness: 3,
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
        noise_controls: NoiseControls::new(NoiseFunction::Curl, 3.0, 1.0),
        location: Some(Location::Halton),
        spacing: 1.0,
        stroke_width: 2.0,
        curve_length: 200,
        size_controls: SizeControls::default()
            .set_size_fn(Some(SizeFn::Constant))
            .set_size(80.0),
        fractal_controls: FractalControls::default().set_octaves(1),
        density: 72.0,
        color1: Color::from_rgb8(121, 72, 141),
        color2: Color::from_rgb8(71, 76, 141),
        grad_style: Some(GradStyle::None),
        background: Some(Background::LightClouds),
        width: "1000".to_string(),
        height: "1200".to_string(),
        ..Default::default()
    }
}

pub fn canyon() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Line),
        noise_controls: NoiseControls::new(NoiseFunction::Fbm, 3.0, 2.0),
        location: Some(Location::Poisson),
        fractal_controls: FractalControls::default().set_octaves(6),
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
        noise_controls: NoiseControls::new(NoiseFunction::Fbm, 4.0, 1.0),
        location: Some(Location::Rand),
        spacing: 20.0,
        stroke_width: 12.5,
        curve_length: 150,
        size_controls: SizeControls::new(
            SizeFn::Periodic,
            200.0,
            crate::size::Dir::Both,
            5.0,
            25.0,
        ),
        fractal_controls: FractalControls::default()
            .set_octaves(6)
            .set_persistence(0.3),
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
        noise_controls: NoiseControls::new(NoiseFunction::Fbm, 2.0, 1.0),
        location: Some(Location::Halton),
        spacing: 7.0,
        stroke_width: 0.0,
        curve_length: 50,
        size_controls: SizeControls::new(SizeFn::Periodic, 40.0, crate::size::Dir::Both, 10.0, 6.0),
        dot_style: Some(DotStyle::Pearl),
        pearl_sides: 5,
        pearl_smoothness: 3,
        fractal_controls: FractalControls::default().set_octaves(1),
        density: 60.0,
        color1: Color::from_rgb8(30, 25, 180),
        color2: Color::from_rgb8(90, 175, 185),
        grad_style: Some(GradStyle::None),
        background: Some(Background::LightGrain),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}

pub fn tubes() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Dots),
        noise_controls: NoiseControls::default().set_noise_function(NoiseFunction::Value),
        location: Some(Location::Lissajous),
        spacing: 1.0,
        stroke_width: 1.0,
        curve_length: 0,
        size_controls: SizeControls::new(
            SizeFn::Periodic,
            235.0,
            crate::size::Dir::Both,
            3.0,
            10.0,
        ),
        density: 65.0,
        color1: Color::from_rgb8(187, 42, 20),
        color2: Color::from_rgb8(155, 21, 48),
        background: Some(Background::DarkClouds),
        width: "1000".to_string(),
        height: "1200".to_string(),
        ..Default::default()
    }
}

pub fn ducts() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Dots),
        noise_controls: NoiseControls::new(NoiseFunction::Sinusoidal, 4.0, 4.0),
        location: Some(Location::Halton),
        dot_style: Some(DotStyle::Square),
        dot_stroke_color: Color::from_rgb8(0, 0, 0),
        spacing: 2.0,
        stroke_width: 0.5,
        curve_length: 150,
        size_controls: SizeControls::new(
            SizeFn::Periodic,
            100.0,
            crate::size::Dir::Both,
            10.0,
            10.0,
        ),
        density: 50.0,
        color1: Color::from_rgb8(218, 187, 55),
        color2: Color::from_rgb8(229, 15, 15),
        sin_controls: SinControls::new(2.0, 2.0, 1.0, 3.0),
        background: Some(Background::LightGrain),
        ..Default::default()
    }
}
