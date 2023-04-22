use crate::background::Background;
use crate::color::{ColorControls, ColorMode, Palettes};
use crate::common::*;
use crate::dot::{DotControls, DotStyle};
use crate::extrude::ExtrudeControls;
use crate::fractal::FractalControls;
use crate::gradient::GradStyle;
use crate::location::Location;
use crate::noise::{NoiseControls, NoiseFunction};
use crate::sine::SineControls;
use crate::size::{Dir, SizeControls, SizeFn};
use iced::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Preset {
    Ribbons,
    Solar,
    RiverStones,
    Vortex,
    Canyon,
    Fence,
    Splat,
    Tubes,
    Ducts,
    Symmetry,
    PomPom,
    RedDwarf,
    Ridges,
}

impl std::fmt::Display for Preset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Preset::Ribbons => "Ribbons",
                Preset::Solar => "Solar",
                Preset::RiverStones => "River Stones",
                Preset::Vortex => "Vortex",
                Preset::Canyon => "Canyon",
                Preset::Fence => "Fence",
                Preset::Splat => "Splat",
                Preset::Tubes => "Tubes",
                Preset::Ducts => "Ducts",
                Preset::Symmetry => "Symmetry",
                Preset::PomPom => "Pom Pom",
                Preset::RedDwarf => "Red Dwarf",
                Preset::Ridges => "Ridges (slow!)",
            }
        )
    }
}

pub fn ribbons() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Extrusion),
        curve_direction: Some(CurveDirection::TwoSided),
        noise_controls: NoiseControls::new(NoiseFunction::Fbm, 3.5, 4.0),
        spacing: 2.0,
        stroke_width: 4.0,
        curve_length: 150,
        extrude_controls: ExtrudeControls::new(
            SizeControls::default().set_size(200.0),
            Some(GradStyle::Fiber),
            // false,
        ),
        fractal_controls: FractalControls::default().set_octaves(1),
        density: 50.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::Royalty),
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
        dot_controls: DotControls {
            size_controls: SizeControls::default()
                .set_size(5.0)
                .set_size_fn(Some(SizeFn::Periodic))
                .set_size_scale(3.0)
                .set_min_size(1.0),
            ..Default::default()
        },
        fractal_controls: FractalControls::default().set_octaves(2),
        density: 100.0,
        color_mode_controls: ColorControls::default().set_anchor1(Color::from_rgb8(111, 171, 181)),
        background: Some(Background::DarkGrain),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}

pub fn solar() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Line),
        noise_controls: NoiseControls::new(NoiseFunction::Curl, 4.0, 1.2),
        location: Some(Location::Circle),
        border: true,
        spacing: 5.0,
        stroke_width: 2.0,
        curve_length: 100,
        density: 100.0,
        speed: 0.1,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::PinotNoir),
        background: Some(Background::LightClouds),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}

pub fn river_stones() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Dots),
        noise_controls: NoiseControls::new(NoiseFunction::Cylinders, 3.0, 3.4),
        location: Some(Location::Poisson),
        dot_controls: DotControls {
            dot_style: Some(DotStyle::Pearl),
            size_controls: SizeControls::default()
                .set_size(165.0)
                .set_size_fn(Some(SizeFn::Periodic))
                .set_size_scale(5.0)
                .set_min_size(25.0),
            pearl_sides: 5,
            pearl_smoothness: 3,
            ..Default::default()
        },
        spacing: 100.0,
        stroke_width: 0.0,
        curve_length: 1,
        density: 45.0,
        color_mode_controls: ColorControls::default().set_anchor1(Color::from_rgb8(45, 10, 65)),
        background: Some(Background::ColorGrain),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}

pub fn vortex() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Extrusion),
        curve_direction: Some(CurveDirection::TwoSided),
        noise_controls: NoiseControls::new(NoiseFunction::Curl, 3.0, 1.0),
        location: Some(Location::Halton),
        spacing: 1.0,
        stroke_width: 2.0,
        curve_length: 200,
        extrude_controls: ExtrudeControls::new(
            SizeControls::default()
                .set_size_fn(Some(SizeFn::Constant))
                .set_size(80.0),
            Some(GradStyle::Plain),
            // false,
        ),
        fractal_controls: FractalControls::default().set_octaves(1),
        density: 72.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::DeltaBlues),
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
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::Rose),
        background: Some(Background::DarkGrain),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}

pub fn fence() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Extrusion),
        curve_direction: Some(CurveDirection::TwoSided),
        noise_controls: NoiseControls::new(NoiseFunction::Fbm, 4.0, 1.0),
        location: Some(Location::Rand),
        spacing: 15.0,
        stroke_width: 12.5,
        curve_length: 150,
        extrude_controls: ExtrudeControls::new(
            SizeControls::new(
                Some(SizeFn::Periodic),
                200.0,
                Some(crate::size::Dir::Both),
                5.0,
                25.0,
            ),
            Some(GradStyle::Plain),
            // false,
        ),
        fractal_controls: FractalControls::default()
            .set_octaves(6)
            .set_persistence(0.3),
        density: 40.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::Algae),
        background: Some(Background::ColorGrain),
        grain_color: Color::from_rgb8(152, 194, 152),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}

pub fn splat() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Dots),
        curve_direction: Some(CurveDirection::TwoSided),
        noise_controls: NoiseControls::new(NoiseFunction::Fbm, 2.0, 1.0),
        location: Some(Location::Halton),
        spacing: 7.0,
        stroke_width: 0.0,
        curve_length: 50,
        dot_controls: DotControls {
            dot_style: Some(DotStyle::Pearl),
            size_controls: SizeControls::default()
                .set_size(40.0)
                .set_size_scale(10.0)
                .set_min_size(6.0)
                .set_size_fn(Some(SizeFn::Periodic))
                .set_direction(Some(crate::size::Dir::Both)),
            pearl_sides: 5,
            pearl_smoothness: 3,
            ..Default::default()
        },
        fractal_controls: FractalControls::default().set_octaves(1),
        density: 60.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::GrayScale),
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
        stroke_width: 0.5,
        curve_length: 15,
        dot_controls: DotControls {
            size_controls: SizeControls::default()
                .set_size(235.0)
                .set_size_scale(3.0)
                .set_min_size(10.0)
                .set_size_fn(Some(SizeFn::Periodic))
                .set_direction(Some(crate::size::Dir::Both)),
            dot_stroke_color: Color::from_rgb8(0, 0, 0),
            ..Default::default()
        },
        density: 85.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::SpiritedAway),
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
        dot_controls: DotControls {
            dot_style: Some(DotStyle::Square),
            dot_stroke_color: Color::from_rgb8(0, 0, 0),
            size_controls: SizeControls::default()
                .set_size(100.0)
                .set_size_scale(10.0)
                .set_min_size(10.0)
                .set_size_fn(Some(SizeFn::Periodic))
                .set_direction(Some(crate::size::Dir::Both)),
            ..Default::default()
        },
        spacing: 2.0,
        stroke_width: 0.5,
        curve_length: 150,
        density: 50.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::Fire),
        sin_controls: SineControls::new(2.0, 2.0, 1.0, 3.0),
        background: Some(Background::ColorGrain),
        grain_color: Color::from_rgb8(195, 130, 65),
        ..Default::default()
    }
}

pub fn symmetry() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Line),
        noise_controls: NoiseControls::new(NoiseFunction::Gravity, 1.0, 1.0),
        location: Some(Location::Rand),
        spacing: 1.0,
        stroke_width: 1.5,
        curve_length: 100,
        density: 100.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::Totoro),
        background: Some(Background::ColorGrain),
        grain_color: Color::from_rgb8(215, 155, 190),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}

pub fn pompom() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Line),
        noise_controls: NoiseControls::new(NoiseFunction::Magnet, 1.0, 1.0),
        location: Some(Location::Poisson),
        spacing: 80.0,
        stroke_width: 0.5,
        curve_length: 25,
        density: 100.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::SpiritedAway),
        background: Some(Background::DarkGrain),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}

pub fn red_dwarf() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Extrusion),
        curve_direction: Some(CurveDirection::TwoSided),
        noise_controls: NoiseControls::new(NoiseFunction::Billow, 3.5, 4.0),
        location: Some(Location::Circle),
        spacing: 1.0,
        stroke_width: 0.5,
        curve_length: 180,
        speed: 0.01,
        extrude_controls: ExtrudeControls::new(
            SizeControls::default()
                .set_direction(Some(Dir::Both))
                .set_size(150.0)
                .set_min_size(1.0)
                .set_size_fn(Some(SizeFn::Contracting)),
            Some(GradStyle::Plain),
            // false,
        ),
        fractal_controls: FractalControls::default().set_octaves(1),
        density: 65.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::PorcoRosso),
        background: Some(Background::DarkClouds),
        width: "1080".to_string(),
        height: "1080".to_string(),
        ..Default::default()
    }
}
