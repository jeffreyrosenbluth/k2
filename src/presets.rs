use crate::background::Background;
use crate::color::{ColorControls, ColorMode, Palettes};
use crate::common::*;
use crate::dot::{DotControls, DotStyle};
use crate::extrude::{ExtrudeControls, ExtrudeDirection};
use crate::fractal::FractalControls;
use crate::gradient::GradStyle;
use crate::location::Location;
use crate::noise::{NoiseControls, NoiseFunction};
use crate::sine::SineControls;
use crate::size::{Dir, SizeControls, SizeFn};
use eframe::egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Preset {
    Ribbons,
    Worms,
    Solar,
    Vortex,
    Canyon,
    Splat,
    Tubes,
    Ducts,
    RedDwarf,
}

impl std::fmt::Display for Preset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Preset::Ribbons => "Ribbons",
                Preset::Worms => "Worms",
                Preset::Solar => "Solar",
                Preset::Vortex => "Vortex",
                Preset::Canyon => "Canyon",
                Preset::Splat => "Splat",
                Preset::Tubes => "Tubes",
                Preset::Ducts => "Ducts",
                Preset::RedDwarf => "Red Dwarf",
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
        hide_ends: true,
        extrude_controls: ExtrudeControls::new(
            SizeControls::default().set_size(150.0),
            Some(GradStyle::Fiber),
            // false,
        ),
        fractal_controls: FractalControls::default().set_octaves(1),
        density: 50.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::Royalty),
        background: Some(Background::LightGrain),
        width: 1080,
        height: 1080,
        ..Default::default()
    }
}

pub fn worms() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Extrusion),
        curve_direction: Some(CurveDirection::TwoSided),
        noise_controls: NoiseControls::new(NoiseFunction::Fbm, 3.5, 4.0),
        location: Some(Location::Halton),
        spacing: 2.0,
        stroke_width: 10.0,
        curve_length: 150,
        hide_ends: true,
        extrude_controls: ExtrudeControls::new(
            SizeControls::default()
                .set_size_fn(Some(SizeFn::Constant))
                .set_size(70.0),
            Some(GradStyle::Fiber),
        )
        .set_direction(ExtrudeDirection::Normal),
        fractal_controls: FractalControls::default()
            .set_octaves(2)
            .set_lacunarity(2.1),
        density: 60.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Scale)
            .set_anchor1(Color32::from_rgb(139, 152, 51))
            .set_palette_choice(Palettes::MonoRed),
        background: Some(Background::LightGrain),
        width: 1080,
        height: 1080,
        ..Default::default()
    }
}

pub fn solar() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Line),
        noise_controls: NoiseControls::new(NoiseFunction::Curl, 4.0, 1.2),
        location: Some(Location::Circle),
        spacing: 5.0,
        stroke_width: 2.0,
        curve_length: 100,
        density: 85.0,
        speed: 0.1,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::PinotNoir),
        background: Some(Background::LightFiber),
        width: 1080,
        height: 1080,
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
        background: Some(Background::LightFiber),
        width: 1080,
        height: 1080,
        ..Default::default()
    }
}

pub fn canyon() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Line),
        noise_controls: NoiseControls::new(NoiseFunction::Fbm, 3.0, 2.0),
        location: Some(Location::Poisson),
        hide_ends: true,
        fractal_controls: FractalControls::default().set_octaves(6),
        spacing: 5.0,
        stroke_width: 2.5,
        curve_length: 75,
        density: 100.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::Rose),
        background: Some(Background::DarkGrain),
        width: 1080,
        height: 1080,
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
        width: 1080,
        height: 1080,
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
            dot_stroke_color: Color32::from_rgb(0, 0, 0),
            ..Default::default()
        },
        density: 85.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::SpiritedAway),
        background: Some(Background::DarkFiber),
        width: 1080,
        height: 1080,
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
            dot_stroke_color: Color32::from_rgb(0, 0, 0),
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
        grain_color: Color32::from_rgb(195, 130, 65),
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
        hide_ends: true,
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
        background: Some(Background::DarkFiber),
        width: 1080,
        height: 1080,
        ..Default::default()
    }
}
