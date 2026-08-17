#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{mpsc, Arc};

use crate::art::draw;
use crate::background::Background;
use crate::color::ColorControls;
use crate::dot::DotControls;
use crate::extrude::ExtrudeControls;
use crate::fractal::FractalControls;
use crate::imgnoise::{ImageNoiseControls, ThumbCache};
use crate::noise::NoiseControls;
use crate::presets::Preset;
use crate::sine::SineControls;

use crate::{location::Location, presets::ribbons};
use eframe::egui;
use serde::{Deserialize, Serialize};

pub const WIDTH: u32 = 1000;
pub const HEIGHT: u32 = 1000;
pub const SEED: u64 = 98713;
/// Scale of the fast preview rendered while the full image is in flight.
pub const PREVIEW_SCALE: f32 = 0.5;

/// A finished render arriving from a worker thread.
pub struct RenderMsg {
    epoch: u64,
    image: egui::ColorImage,
    logical: egui::Vec2,
    full: bool,
}

pub struct K2 {
    pub controls: Controls,
    /// The controls as of the last render kick-off; used to detect edits.
    pub last_drawn: Controls,
    pub texture: Option<egui::TextureHandle>,
    /// Logical (unscaled) size of the displayed image, for stable layout
    /// while previews and full renders of different resolutions swap in.
    pub image_logical: egui::Vec2,
    pub exporting: Arc<AtomicBool>,
    /// True from render kick-off until its full resolution image lands.
    pub rendering: bool,
    /// Set by the Draw button (and preset loads); consumed by the frame loop.
    pub pending_draw: bool,
    /// Thumbnail of the image noise source shown in the right panel.
    pub image_thumb: ThumbCache,
    epoch: Arc<AtomicU64>,
    tx: mpsc::Sender<RenderMsg>,
    rx: mpsc::Receiver<RenderMsg>,
}

impl K2 {
    pub fn new() -> Self {
        let controls = ribbons();
        let (tx, rx) = mpsc::channel();
        Self {
            last_drawn: controls.clone(),
            controls,
            texture: None,
            image_logical: egui::vec2(WIDTH as f32, HEIGHT as f32),
            exporting: Arc::new(AtomicBool::new(false)),
            rendering: false,
            pending_draw: false,
            image_thumb: ThumbCache::default(),
            epoch: Arc::new(AtomicU64::new(0)),
            tx,
            rx,
        }
    }

    /// Kick off an asynchronous render of the current controls: a fast
    /// reduced-scale preview first, then the full resolution image, each
    /// swapped in as it arrives. A newer kick-off supersedes older renders.
    pub fn start_render(&mut self, ctx: &egui::Context) {
        let epoch = self.epoch.fetch_add(1, Ordering::Relaxed) + 1;
        self.rendering = true;
        self.last_drawn = self.controls.clone();
        let controls = self.controls.clone();
        let latest = self.epoch.clone();
        let tx = self.tx.clone();
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            for (scale, full) in [(PREVIEW_SCALE, false), (1.0, true)] {
                if latest.load(Ordering::Relaxed) != epoch {
                    return;
                }
                let canvas = draw(&controls, scale);
                let image = egui::ColorImage::from_rgba_premultiplied(
                    [
                        canvas.pixmap.width() as usize,
                        canvas.pixmap.height() as usize,
                    ],
                    canvas.pixmap.data(),
                );
                let logical = egui::vec2(canvas.w_f32(), canvas.h_f32());
                if latest.load(Ordering::Relaxed) != epoch
                    || tx
                        .send(RenderMsg {
                            epoch,
                            image,
                            logical,
                            full,
                        })
                        .is_err()
                {
                    return;
                }
                ctx.request_repaint();
            }
        });
    }

    /// Apply any renders that have arrived, discarding superseded ones.
    pub fn poll_renders(&mut self, ctx: &egui::Context) {
        while let Ok(msg) = self.rx.try_recv() {
            if msg.epoch != self.epoch.load(Ordering::Relaxed) {
                continue;
            }
            self.image_logical = msg.logical;
            match &mut self.texture {
                Some(texture) => texture.set(msg.image, egui::TextureOptions::LINEAR),
                None => {
                    self.texture =
                        Some(ctx.load_texture("art", msg.image, egui::TextureOptions::LINEAR))
                }
            }
            if msg.full {
                self.rendering = false;
            }
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Controls {
    pub preset: Option<Preset>,
    pub curve_style: Option<CurveStyle>,
    pub curve_direction: Option<CurveDirection>,
    pub spacing: f32,
    pub curve_length: u32,
    pub hide_ends: bool,
    pub grain_color: egui::Color32,
    pub solid_color: egui::Color32,
    pub location: Option<Location>,
    pub density: f32,
    pub noise_controls: NoiseControls,
    pub fractal_controls: FractalControls,
    pub speed: f32,
    pub stroke_width: f32,
    pub background: Option<Background>,
    pub width: u32,
    pub height: u32,
    pub sin_controls: SineControls,
    pub dot_controls: DotControls,
    pub extrude_controls: ExtrudeControls,
    pub color_mode_controls: ColorControls,
    #[serde(default)]
    pub image_noise: ImageNoiseControls,
}

impl Controls {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            preset: Some(Preset::Ribbons),
            curve_style: Some(CurveStyle::Dots),
            spacing: 4.0,
            curve_length: 50,
            hide_ends: false,
            curve_direction: Some(CurveDirection::OneSided),
            grain_color: egui::Color32::from_rgb(128, 128, 128),
            solid_color: egui::Color32::from_rgb(245, 242, 235),
            location: Some(Location::Halton),
            noise_controls: NoiseControls::default(),
            density: 50.0,
            fractal_controls: FractalControls::default(),
            speed: 1.0,
            stroke_width: 1.0,
            background: Some(Background::LightFiber),
            width: 1080,
            height: 1080,
            sin_controls: SineControls::default(),
            dot_controls: DotControls::default(),
            extrude_controls: ExtrudeControls::default(),
            color_mode_controls: ColorControls::default(),
            image_noise: ImageNoiseControls::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurveStyle {
    Line,
    Dots,
    Extrusion,
}

impl std::fmt::Display for CurveStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CurveStyle::Line => "Line",
                CurveStyle::Dots => "Dots",
                CurveStyle::Extrusion => "Extrusion",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurveDirection {
    OneSided,
    TwoSided,
}

impl std::fmt::Display for CurveDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CurveDirection::OneSided => "One Sided",
                CurveDirection::TwoSided => "Two Sided",
            }
        )
    }
}
