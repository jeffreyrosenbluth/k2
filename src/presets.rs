use crate::background::Background;
use crate::color::{ColorBy, ColorControls, ColorMode, Palettes};
use crate::common::*;
use crate::dot::{DotControls, DotStyle};
use crate::extrude::{ExtrudeControls, ExtrudeDirection};
use crate::fractal::FractalControls;
use crate::gradient::GradStyle;
use crate::location::Location;
use crate::noise::{NoiseControls, NoiseFunction, WorleyControls, WorleyReturn};
use crate::sine::SineControls;
use crate::size::{Dir, SizeControls, SizeFn};
use eframe::egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Preset {
    Ribbons,
    Worms,
    Solar,
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
        location: Some(Location::Halton),
        spacing: 2.0,
        stroke_width: 4.0,
        curve_length: 150,
        hide_ends: true,
        extrude_controls: ExtrudeControls::new(
            SizeControls::default().set_size(150.0),
            Some(GradStyle::Fiber),
        ),
        fractal_controls: FractalControls::default().set_octaves(1),
        density: 60.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::Royalty)
            .set_color_by(ColorBy::Region)
            .set_region(1.5, 9),
        background: Some(Background::LightGrain),
        grain_amount: 0.2,
        width: 1080,
        height: 1080,
        ..Default::default()
    }
}

pub fn worms() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Extrusion),
        curve_direction: Some(CurveDirection::TwoSided),
        noise_controls: NoiseControls::new(NoiseFunction::Fbm, 3.0, 2.7),
        location: Some(Location::Halton),
        spacing: 2.0,
        stroke_width: 16.0,
        opacity: 0.34,
        curve_length: 150,
        hide_ends: true,
        speed: 0.12,
        extrude_controls: ExtrudeControls::new(
            SizeControls::default()
                .set_size_fn(Some(SizeFn::Constant))
                .set_size(75.0),
            Some(GradStyle::Double),
        )
        .set_direction(ExtrudeDirection::Normal),
        fractal_controls: FractalControls::default()
            .set_octaves(2)
            .set_lacunarity(2.1),
        density: 55.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::PorcoRosso)
            .set_color_by(ColorBy::Cycle),
        background: Some(Background::White),
        grain_amount: 0.2,
        width: 1080,
        height: 1080,
        ..Default::default()
    }
}

pub fn solar() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Line),
        noise_controls: NoiseControls::new(NoiseFunction::Curl, 2.0, 1.5),
        location: Some(Location::Circle),
        spacing: 5.0,
        stroke_width: 3.5,
        opacity: 0.25,
        curve_length: 200,
        density: 100.0,
        speed: 0.1,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::Scepter)
            .set_color_by(ColorBy::FlowAngle),
        background: Some(Background::Black),
        grain_amount: 0.2,
        width: 1080,
        height: 1080,
        ..Default::default()
    }
}

pub fn canyon() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Line),
        noise_controls: NoiseControls::new(NoiseFunction::Fbm, 3.0, 2.0),
        location: Some(Location::Phyllotaxis),
        hide_ends: true,
        fractal_controls: FractalControls::default().set_octaves(2),
        spacing: 5.0,
        stroke_width: 2.5,
        curve_length: 75,
        density: 90.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::Rose)
            .set_color_by(ColorBy::Order),
        background: Some(Background::Black),
        grain_amount: 0.2,
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
        noise_controls: NoiseControls::new(NoiseFunction::Worley, 4.0, 2.0),
        location: Some(Location::Rings),
        worley: WorleyControls {
            return_type: Some(WorleyReturn::Value),
            ..Default::default()
        },
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
        density: 90.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Scale)
            .set_anchor1(Color32::from_rgb(196, 154, 60))
            .set_anchor2(Color32::from_rgb(78, 67, 67))
            .set_palette_choice(Palettes::SpiritedAway)
            .set_color_by(ColorBy::Order)
            .set_along(4.0, true, false),
        background: Some(Background::Black),
        grain_amount: 0.2,
        width: 1080,
        height: 1080,
        ..Default::default()
    }
}

pub fn ducts() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Dots),
        noise_controls: NoiseControls::new(NoiseFunction::Sinusoidal, 4.0, 10.0),
        location: Some(Location::Grid),
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
        stroke_width: 1.0,
        curve_length: 150,
        density: 50.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::DeltaBlues)
            .set_color_by(ColorBy::PositionY),
        sin_controls: SineControls::new(2.0, 2.0, 1.0, 3.0),
        background: Some(Background::White),
        width: 1080,
        height: 1080,
        ..Default::default()
    }
}

pub fn red_dwarf() -> Controls {
    Controls {
        curve_style: Some(CurveStyle::Extrusion),
        curve_direction: Some(CurveDirection::TwoSided),
        noise_controls: NoiseControls::new(NoiseFunction::Billow, 4.0, 4.0),
        location: Some(Location::Circle),
        spacing: 1.0,
        stroke_width: 0.5,
        curve_length: 180,
        speed: 0.02,
        hide_ends: true,
        extrude_controls: ExtrudeControls::new(
            SizeControls::default()
                .set_direction(Some(Dir::Both))
                .set_size(150.0)
                .set_min_size(1.0)
                .set_size_fn(Some(SizeFn::Contracting)),
            Some(GradStyle::Plain),
        ),
        fractal_controls: FractalControls::default().set_octaves(1),
        density: 45.0,
        color_mode_controls: ColorControls::default()
            .set_mode(ColorMode::Palette)
            .set_palette_choice(Palettes::Fire)
            .set_color_by(ColorBy::Radial),
        background: Some(Background::Black),
        grain_amount: 0.2,
        width: 1080,
        height: 1080,
        ..Default::default()
    }
}
