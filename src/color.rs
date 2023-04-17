#![allow(dead_code)]

use iced::{
    widget::{button, radio, row, text, Column},
    Alignment::Center,
    Element,
};
use iced_aw::ColorPicker;
use wassily::prelude::{palette::LuvHue, palette::Saturate, palette::Shade, Color, *};

use crate::gui::lpicklist;

#[derive(Debug, Clone, PartialEq)]
pub enum ColorPickerMessage {
    Choose,
    Submit(iced::Color),
    Cancel,
}

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

pub fn expand_palette(palette: Vec<Color>) -> Vec<Color> {
    let mut result = palette.clone();
    let n = palette.len();
    for i in 0..n {
        for j in i..n {
            let c = result[i].lerp(&result[j], 0.5);
            result.push(c);
        }
    }
    return result;
}

fn hex_to_color(hex: Vec<u32>) -> Vec<Color> {
    hex.iter()
        .map(|h| {
            let (r, g, b) = Srgb::from(*h).into_components();
            Color::from_rgba8(r, g, b, 255)
        })
        .collect::<Vec<Color>>()
}

fn make_palette(hex: Vec<u32>) -> Palette {
    let raw_palette = hex_to_color(hex);
    Palette::new(expand_palette(raw_palette))
}

pub fn color_palette(pal: Palettes) -> Palette {
    use Palettes::*;
    match pal {
        Royalty => make_palette(vec![0x1C4572, 0x84561B, 0x6D3E32, 0x0A0E20]),
        DeltaBlues => make_palette(vec![0x003566, 0x000000, 0x008080]),
        PinotNoir => make_palette(vec![0x701C1C, 0x1A1717, 0x77806E]),
        Algae => make_palette(vec![0xA3B18A, 0x588157, 0x3A5A40, 0x344E41]),
        Scepter => make_palette(vec![0xB7A635, 0x4E1406]),
        Fire => make_palette(vec![0x621708, 0x941B0C, 0xBC3908, 0xF6AA1C]),
        Perfume => make_palette(vec![0xD9798B, 0x8C4962, 0x59364A, 0x594832]),
        Rose => make_palette(vec![0xBF2642, 0x731F2E, 0x400C16]),
        GrayScale => make_palette(vec![0x000000, 0xE6E6E6, 0xA0A0A0]),
        PorcoRosso => make_palette(vec![0x002B75, 0x862A23, 0xBD8878]),
        SpiritedAway => make_palette(vec![0xD9A404, 0xF2B988, 0xBF3030, 0x0D0D0D]),
        Totoro => make_palette(vec![0x6A7AB2, 0xF27E9D, 0x454259, 0x9B8660]),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Palettes {
    Royalty,
    DeltaBlues,
    PinotNoir,
    Algae,
    Scepter,
    Fire,
    Perfume,
    Rose,
    GrayScale,
    PorcoRosso,
    SpiritedAway,
    Totoro,
}

impl std::fmt::Display for Palettes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Palettes::Royalty => write!(f, "Royalty"),
            Palettes::DeltaBlues => write!(f, "Delta Blues"),
            Palettes::PinotNoir => write!(f, "Pinot Noir"),
            Palettes::Algae => write!(f, "Algae"),
            Palettes::Scepter => write!(f, "Scepter"),
            Palettes::Fire => write!(f, "Fire"),
            Palettes::Perfume => write!(f, "Perfume"),
            Palettes::Rose => write!(f, "Rose"),
            Palettes::GrayScale => write!(f, "Gray Scale"),
            Palettes::PorcoRosso => write!(f, "Porco Rosso"),
            Palettes::SpiritedAway => write!(f, "Spirited Away"),
            Palettes::Totoro => write!(f, "Totoro"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorMode {
    Palette,
    Scale,
}

impl From<ColorMode> for String {
    fn from(mode: ColorMode) -> Self {
        match mode {
            ColorMode::Palette => "Palette",
            ColorMode::Scale => "Color Scale",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColorMessage {
    Mode(ColorMode),
    Anchor1(ColorPickerMessage),
    Anchor2(ColorPickerMessage),
    PaletteChoice(Palettes),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorControls {
    pub mode: Option<ColorMode>,
    pub anchor1: iced::Color,
    pub anchor2: iced::Color,
    pub show_picker_1: bool,
    pub show_picker_2: bool,
    pub palette_choice: Option<Palettes>,
    pub dirty: bool,
}

impl Default for ColorControls {
    fn default() -> Self {
        Self {
            mode: Some(ColorMode::Scale),
            anchor1: iced::Color::from_rgb8(20, 134, 187),
            anchor2: iced::Color::from_rgb8(0, 0, 0),
            show_picker_1: false,
            show_picker_2: false,
            palette_choice: Some(Palettes::Royalty),
            dirty: false,
        }
    }
}

impl<'a> ColorControls {
    pub fn new(
        mode: Option<ColorMode>,
        anchor1: iced::Color,
        anchor2: iced::Color,
        show_picker_1: bool,
        show_picker_2: bool,
        palette_choice: Option<Palettes>,
        dirty: bool,
    ) -> Self {
        Self {
            mode,
            anchor1,
            anchor2,
            show_picker_1,
            show_picker_2,
            palette_choice,
            dirty,
        }
    }

    pub fn set_anchor1(mut self, color: iced::Color) -> Self {
        self.anchor1 = color;
        self
    }

    pub fn set_anchor2(mut self, color: iced::Color) -> Self {
        self.anchor2 = color;
        self
    }

    pub fn set_mode(mut self, mode: ColorMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn set_palette_choice(mut self, pal: Palettes) -> Self {
        self.palette_choice = Some(pal);
        self
    }

    pub fn update(&mut self, message: ColorMessage) {
        use ColorMessage::*;
        use ColorPickerMessage::*;
        match message {
            Mode(m) => {
                self.mode = Some(m);
                self.dirty = true;
            }
            Anchor1(message) => match message {
                Choose => {
                    self.show_picker_1 = true;
                    self.dirty = false;
                }
                Submit(color) => {
                    self.anchor1 = color;
                    self.show_picker_1 = false;
                    self.dirty = true;
                }
                Cancel => {
                    self.show_picker_1 = false;
                    self.dirty = false;
                }
            },
            Anchor2(message) => match message {
                Choose => {
                    self.show_picker_2 = true;
                    self.dirty = false;
                }
                Submit(color) => {
                    self.anchor2 = color;
                    self.show_picker_2 = false;
                    self.dirty = true;
                }
                Cancel => {
                    self.show_picker_2 = false;
                    self.dirty = false;
                }
            },
            PaletteChoice(c) => {
                self.palette_choice = Some(c);
                self.dirty = true;
            }
            Null => self.dirty = true,
        }
    }

    pub fn view(&mut self) -> Element<'a, ColorMessage> {
        use ColorMessage::*;
        use Palettes::*;
        let mut col = Column::new();
        let mode = row([ColorMode::Palette, ColorMode::Scale]
            .iter()
            .cloned()
            .map(|m| radio(m, m, self.mode, Mode).text_size(15).size(15))
            .map(Element::from)
            .collect())
        .spacing(15);
        col = col.push(mode);
        if self.mode == Some(ColorMode::Scale) {
            let color_button1 = button(text("Anchor 1 Color").size(15))
                .on_press(Anchor1(ColorPickerMessage::Choose));
            let color_button2 = button(text("Anchor 2 Color").size(15))
                .on_press(Anchor2(ColorPickerMessage::Choose));
            let color_picker1 = ColorPicker::new(
                self.show_picker_1,
                self.anchor1,
                color_button1,
                Anchor1(ColorPickerMessage::Cancel),
                |c| Anchor1(ColorPickerMessage::Submit(c)),
            );
            let color_picker2 = ColorPicker::new(
                self.show_picker_2,
                self.anchor2,
                color_button2,
                Anchor2(ColorPickerMessage::Cancel),
                |c| Anchor2(ColorPickerMessage::Submit(c)),
            );
            col = col
                .push(
                    row![
                        color_picker1,
                        text(format!(
                            "{:3} {:3} {:3}",
                            (self.anchor1.r * 255.0) as u8,
                            (self.anchor1.g * 255.0) as u8,
                            (self.anchor1.b * 255.0) as u8
                        ))
                        .size(15)
                    ]
                    .spacing(15)
                    .align_items(Center),
                )
                .push(
                    row![
                        color_picker2,
                        text(format!(
                            "{:3} {:3} {:3}",
                            (self.anchor2.r * 255.0) as u8,
                            (self.anchor2.g * 255.0) as u8,
                            (self.anchor2.b * 255.0) as u8
                        ))
                        .size(15)
                    ]
                    .spacing(15)
                    .align_items(Center),
                )
                .spacing(15);
        } else {
            let pc = lpicklist::LPickList::new(
                "Palette".to_string(),
                vec![
                    Royalty,
                    DeltaBlues,
                    PinotNoir,
                    Algae,
                    Scepter,
                    Fire,
                    Perfume,
                    Rose,
                    GrayScale,
                    PorcoRosso,
                    SpiritedAway,
                    Totoro,
                ],
                self.palette_choice,
                |x| x.map_or(Null, PaletteChoice),
            );
            col = col.push(pc);
        }
        col.spacing(15).into()
    }
}
