//! An image as a flow field, ported from mixel's warp algorithm: each pixel
//! maps through a color map to a value in [-1, 1], and the image is sampled
//! as a NoiseFn with mirror-reflect tiling.

use crate::gui::{pick_list, section, SliderRow};
use eframe::egui;
use image::RgbaImage;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use wassily::prelude::palette::{GetHue, IntoColor, Okhsl, Okhsv, Srgb, Xyz};
use wassily::prelude::NoiseFn;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl std::fmt::Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rotation::Deg0 => "0°",
                Rotation::Deg90 => "90°",
                Rotation::Deg180 => "180°",
                Rotation::Deg270 => "270°",
            }
        )
    }
}

fn apply_rotation(img: &RgbaImage, rotation: Rotation) -> RgbaImage {
    match rotation {
        Rotation::Deg0 => img.clone(),
        Rotation::Deg90 => image::imageops::rotate90(img),
        Rotation::Deg180 => image::imageops::rotate180(img),
        Rotation::Deg270 => image::imageops::rotate270(img),
    }
}

fn default_rotation() -> Option<Rotation> {
    Some(Rotation::Deg0)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorMap {
    Lightness,
    Hue,
    Saturation,
    WrappedHue,
    HueSat,
    LumaSat,
    Chroma,
    Value,
    XyzX,
    XyzZ,
}

impl std::fmt::Display for ColorMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ColorMap::Lightness => "Lightness",
                ColorMap::Hue => "Hue",
                ColorMap::Saturation => "Saturation",
                ColorMap::WrappedHue => "Wrapped Hue",
                ColorMap::HueSat => "Hue Sat",
                ColorMap::LumaSat => "Luma Sat",
                ColorMap::Chroma => "Chroma",
                ColorMap::Value => "Value",
                ColorMap::XyzX => "Xyz X",
                ColorMap::XyzZ => "Xyz Z",
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageNoiseControls {
    pub path: Option<String>,
    pub color_map: Option<ColorMap>,
    /// Gaussian blur sigma applied to the image before the color map.
    #[serde(default)]
    pub blur: f32,
    #[serde(default = "default_rotation")]
    pub rotation: Option<Rotation>,
}

impl Default for ImageNoiseControls {
    fn default() -> Self {
        Self {
            path: None,
            color_map: Some(ColorMap::Lightness),
            blur: 0.0,
            rotation: Some(Rotation::Deg0),
        }
    }
}

impl ImageNoiseControls {
    pub fn ui(&mut self, ui: &mut egui::Ui, thumb: &mut ThumbCache) {
        use ColorMap::*;
        section(ui, "Image Noise");
        egui::Grid::new("image_noise")
            .spacing((15.0, 10.0))
            .min_col_width(90.0)
            .show(ui, |ui| {
                ui.label("Image");
                let name = self
                    .path
                    .as_deref()
                    .and_then(|p| std::path::Path::new(p).file_name())
                    .map_or("Choose...".to_string(), |n| n.to_string_lossy().to_string());
                let button = ui.add(egui::Button::new(name).min_size(egui::vec2(150.0, 0.0)));
                let button = match self.path.as_deref() {
                    Some(p) => button.on_hover_text(p),
                    None => button.on_hover_ui(|ui| {
                        ui.colored_label(egui::Color32::ORANGE, "Click to select the image that");
                        ui.colored_label(egui::Color32::ORANGE, "drives the flow field.");
                    }),
                };
                if button.clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("image", &["png", "jpg", "jpeg"])
                        .pick_file()
                    {
                        self.path = Some(path.to_string_lossy().to_string());
                    }
                }
                ui.end_row();
                pick_list(
                    ui,
                    "Color Map",
                    &[
                        Lightness, Hue, Saturation, WrappedHue, HueSat, LumaSat, Chroma, Value,
                        XyzX, XyzZ,
                    ],
                    &mut self.color_map,
                );
                SliderRow::new("Blur", &mut self.blur, 0.0, 0.0..=500.0)
                    .hover(&["Gaussian blur of the image", "before the color map."])
                    .steps(1.0, 5.0)
                    .unclamped()
                    .show(ui);
                pick_list(
                    ui,
                    "Rotation",
                    &[
                        Rotation::Deg0,
                        Rotation::Deg90,
                        Rotation::Deg180,
                        Rotation::Deg270,
                    ],
                    &mut self.rotation,
                );
            });
        if let Some(texture) = thumb.texture(ui.ctx(), self) {
            ui.add_space(crate::gui::SPACE);
            ui.vertical_centered(|ui| {
                ui.add(egui::Image::new(&texture));
            });
        }
    }
}

/// A cached thumbnail of the processed (rotated, blurred) source image.
#[derive(Default)]
pub struct ThumbCache {
    key: Option<(String, u32, Rotation)>,
    texture: Option<egui::TextureHandle>,
}

const THUMB: f32 = 185.0;

impl ThumbCache {
    fn texture(
        &mut self,
        ctx: &egui::Context,
        controls: &ImageNoiseControls,
    ) -> Option<egui::TextureHandle> {
        let path = controls.path.clone()?;
        let rotation = controls.rotation.unwrap_or(Rotation::Deg0);
        let key = (path.clone(), controls.blur.to_bits(), rotation);
        if self.key.as_ref() != Some(&key) {
            let orig = original(&path)?;
            // Downscale first so blurring stays instant while sliding; the
            // blur sigma shrinks with the image to stay visually faithful.
            let (ow, oh) = (orig.width() as f32, orig.height() as f32);
            let s = (THUMB / ow).min(THUMB / oh).min(1.0);
            let (tw, th) = (
                (ow * s).round().max(1.0) as u32,
                (oh * s).round().max(1.0) as u32,
            );
            let mut small = image::imageops::resize(
                orig.as_ref(),
                tw,
                th,
                image::imageops::FilterType::Triangle,
            );
            small = apply_rotation(&small, rotation);
            if controls.blur > 0.0 {
                small = image::imageops::fast_blur(&small, controls.blur * s);
            }
            let ci = egui::ColorImage::from_rgba_unmultiplied(
                [small.width() as usize, small.height() as usize],
                small.as_raw(),
            );
            self.texture = Some(ctx.load_texture("imgnoise thumb", ci, Default::default()));
            self.key = Some(key);
        }
        self.texture.clone()
    }
}

/// Pre-computed color map values for fast lookups during curve generation.
pub struct ImgNoise {
    cache: Vec<f64>,
    width: usize,
    height: usize,
}

impl ImgNoise {
    fn new(img: &image::RgbaImage, colormap: ColorMap) -> Self {
        let width = img.width() as usize;
        let height = img.height() as usize;
        let cache = img
            .pixels()
            .map(|p| apply_colormap(p.0, colormap))
            .collect();
        Self {
            cache,
            width,
            height,
        }
    }

    #[inline]
    fn reflect(p: f64, period: f64) -> f64 {
        let mut p = p;
        while p < 0.0 {
            p += period * 2.0;
        }
        p %= 2.0 * period;
        let r = if p >= period { 2.0 * period - p } else { p };
        r.clamp(0.0, period - 1.0)
    }
}

impl NoiseFn<f64, 2> for ImgNoise {
    #[inline]
    fn get(&self, point: [f64; 2]) -> f64 {
        let x = point[0] * self.width as f64;
        let y = point[1] * self.height as f64;
        let px = (Self::reflect(x, self.width as f64) as usize).min(self.width - 1);
        let py = (Self::reflect(y, self.height as f64) as usize).min(self.height - 1);
        self.cache[py * self.width + px]
    }
}

/// A cheaply clonable handle so every render chunk can share one cache.
pub struct SharedImgNoise(pub Arc<ImgNoise>);

impl NoiseFn<f64, 2> for SharedImgNoise {
    #[inline]
    fn get(&self, point: [f64; 2]) -> f64 {
        self.0.get(point)
    }
}

static ORIGINAL: Mutex<Option<(String, Arc<RgbaImage>)>> = Mutex::new(None);

/// The decoded source image, cached until the path changes.
fn original(path: &str) -> Option<Arc<RgbaImage>> {
    let mut cache = ORIGINAL.lock().unwrap();
    if let Some((p, img)) = cache.as_ref() {
        if p == path {
            return Some(img.clone());
        }
    }
    let img = Arc::new(image::open(path).ok()?.to_rgba8());
    *cache = Some((path.to_string(), img.clone()));
    Some(img)
}

type NoiseKey = (String, ColorMap, u32, Rotation);
static CACHE: Mutex<Option<(NoiseKey, Arc<ImgNoise>)>> = Mutex::new(None);

/// The image noise for (path, colormap, blur, rotation), processed once and
/// cached until any of them change. None if the image cannot be read.
pub fn cached_noise(
    path: &str,
    colormap: ColorMap,
    blur: f32,
    rotation: Rotation,
) -> Option<Arc<ImgNoise>> {
    let key: NoiseKey = (path.to_string(), colormap, blur.to_bits(), rotation);
    let mut cache = CACHE.lock().unwrap();
    if let Some((k, n)) = cache.as_ref() {
        if *k == key {
            return Some(n.clone());
        }
    }
    let orig = original(path)?;
    let rotated = apply_rotation(&orig, rotation);
    let processed = if blur > 0.0 {
        image::imageops::fast_blur(&rotated, blur)
    } else {
        rotated
    };
    let noise = Arc::new(ImgNoise::new(&processed, colormap));
    *cache = Some((key, noise.clone()));
    Some(noise)
}

#[inline]
fn to_okhsl(c: [u8; 4]) -> Okhsl {
    let srgb = Srgb::new(
        c[0] as f32 / 255.0,
        c[1] as f32 / 255.0,
        c[2] as f32 / 255.0,
    );
    srgb.into_color()
}

#[inline]
fn to_okhsv(c: [u8; 4]) -> Okhsv {
    let srgb = Srgb::new(
        c[0] as f32 / 255.0,
        c[1] as f32 / 255.0,
        c[2] as f32 / 255.0,
    );
    srgb.into_color()
}

#[inline]
fn to_xyz(c: [u8; 4]) -> Xyz {
    let srgb = Srgb::new(
        c[0] as f32 / 255.0,
        c[1] as f32 / 255.0,
        c[2] as f32 / 255.0,
    );
    srgb.into_color()
}

#[inline]
fn apply_colormap(c: [u8; 4], colormap: ColorMap) -> f64 {
    match colormap {
        ColorMap::Lightness => 2.0 * (to_okhsl(c).lightness as f64 - 0.5),
        ColorMap::Hue => {
            let degrees = to_okhsl(c).get_hue().into_positive_degrees();
            2.0 * (degrees as f64 / 360.0 - 0.5)
        }
        ColorMap::Saturation => 2.0 * (to_okhsl(c).saturation as f64 - 0.5),
        ColorMap::Value => 2.0 * (to_okhsv(c).value as f64 - 0.5),
        ColorMap::WrappedHue => {
            let degrees = to_okhsl(c).get_hue().into_positive_degrees();
            let h = f64::min(degrees as f64, 360.0 - degrees as f64);
            2.0 * (h / 360.0 - 0.5)
        }
        ColorMap::HueSat => {
            let okhsl = to_okhsl(c);
            let hue = okhsl.get_hue().into_positive_degrees() as f64 / 360.0;
            2.0 * (hue * okhsl.saturation as f64 - 0.5)
        }
        ColorMap::LumaSat => {
            let okhsl = to_okhsl(c);
            2.0 * (okhsl.lightness as f64 * okhsl.saturation as f64 - 0.5)
        }
        ColorMap::Chroma => {
            let mx = c[0].max(c[1]).max(c[2]);
            let mn = c[0].min(c[1]).min(c[2]);
            2.0 * ((mx - mn) as f64 / 255.0 - 0.5)
        }
        ColorMap::XyzX => 2.0 * (to_xyz(c).x as f64 / 0.95 - 0.5),
        ColorMap::XyzZ => 2.0 * (to_xyz(c).z as f64 / 1.09 - 0.5),
    }
}
