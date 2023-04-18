use directories::UserDirs;
use iced::{
    widget::{
        button, image, radio, row, text, text_input, toggler, vertical_space, Container, Rule,
    },
    Alignment::{self, Center},
    Application, Command, Element, Settings, Theme,
};
use iced_aw::ColorPicker;
use std::path::PathBuf;

mod art;
mod background;
mod color;
mod common;
mod dot;
mod extrude;
mod field;
mod fractal;
mod gradient;
mod gui;
mod location;
mod noise;
mod presets;
mod sine;
mod size;

use crate::art::draw;
use crate::background::Background;
use crate::color::{ColorControls, ColorMessage, ColorPickerMessage};
use crate::common::{PresetState::NotSet, *};
use crate::dot::{DotControls, DotMessage};
use crate::extrude::{ExtrudeControls, ExtrudeMessage};
use crate::fractal::Fractal;
use crate::gui::{lpicklist, lslider::LSlider};
use crate::location::Location;
use crate::noise::NoiseFunction;
use crate::presets::*;
use crate::sine::{SineControls, SineMessage};

const TEXT_SIZE: u16 = 15;

pub fn main() -> iced::Result {
    env_logger::init();
    let mut settings = Settings::default();
    settings.window.size = (1480, 1100);
    K2::run(settings)
}

pub async fn print(controls: Controls) {
    let canvas = draw(&controls, true);
    let dirs = UserDirs::new().unwrap();
    let dir = dirs.download_dir().unwrap();
    let path = format!(r"{}/{}", dir.to_string_lossy(), "k2");
    let mut num = 0;
    let mut sketch = PathBuf::from(format!(r"{path}_{num}"));
    sketch.set_extension("png");
    while sketch.exists() {
        num += 1;
        sketch = PathBuf::from(format!(r"{path}_{num}"));
        sketch.set_extension("png");
    }
    canvas.save_png(&sketch);
}

#[derive(Debug, Clone)]
pub enum Message {
    Preset(Preset),
    CurveStyle(CurveStyle),
    CurveDirection(CurveDirection),
    Space(f32),
    CurveLength(u32),
    Export,
    Draw(PresetState),
    Loc(Location),
    Density(f32),
    Octaves(u8),
    Factor(f32),
    NoiseScale(f32),
    Persistence(f32),
    Lacunarity(f32),
    Frequency(f32),
    Noise(NoiseFunction),
    Speed(f32),
    ExportComplete(()),
    StrokeWidth(f32),
    WidthSet(String),
    Width,
    HeightSet(String),
    Height,
    GrainColor(ColorPickerMessage),
    ColorMode(ColorMessage),
    Background(Background),
    Border(bool),
    Sinusoid(SineMessage),
    Dot(DotMessage),
    Extrude(ExtrudeMessage),
    Null,
}

impl Application for K2 {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (K2, Command<Message>) {
        (Self::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("K2")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        use crate::presets::Preset::*;
        use Message::*;
        use PresetState::*;
        match message {
            Preset(p) => {
                self.controls = match p {
                    Ribbons => ribbons(),
                    Solar => solar(),
                    RiverStones => river_stones(),
                    Vortex => vortex(),
                    Canyon => canyon(),
                    Fence => fence(),
                    Splat => splat(),
                    Tubes => tubes(),
                    Ducts => ducts(),
                    Symmetry => symmetry(),
                    PomPom => pompom(),
                    RedDwarf => red_dwarf(),
                    Ridges => ridges(),
                };
                self.controls.preset = Some(p);
                self.draw(Set);
            }
            CurveStyle(cs) => {
                self.controls.curve_style = Some(cs);
                self.draw(NotSet);
            }
            CurveDirection(cd) => {
                self.controls.curve_direction = Some(cd);
                self.draw(NotSet);
            }
            Space(b) => self.controls.spacing = b,
            CurveLength(l) => self.controls.curve_length = l,
            Export => {
                self.controls.exporting = true;
                return Command::perform(print(self.controls.clone()), ExportComplete);
            }
            Loc(loc) => {
                self.controls.location = Some(loc);
                self.draw(NotSet);
            }
            Density(s) => self.controls.density = s,
            Draw(state) => {
                self.draw(state);
            }
            Octaves(o) => self.controls.fractal_controls.octaves = o,
            Persistence(p) => self.controls.fractal_controls.persistence = p,
            Lacunarity(l) => self.controls.fractal_controls.lacunarity = l,
            Frequency(f) => self.controls.fractal_controls.frequency = f,
            Factor(f) => self.controls.noise_controls.noise_factor = f,
            NoiseScale(s) => self.controls.noise_controls.noise_scale = s,
            Noise(n) => {
                self.controls.noise_controls.noise_function = Some(n);
                self.draw(NotSet);
            }
            Speed(s) => self.controls.speed = s,
            Dot(d) => {
                self.controls.dot_controls.update(d);
                if self.controls.dot_controls.dirty {
                    self.draw(NotSet)
                };
            }
            Extrude(e) => {
                self.controls.extrude_controls.update(e);
                if self.controls.extrude_controls.dirty {
                    self.draw(NotSet)
                };
            }
            ExportComplete(_) => self.controls.exporting = false,
            StrokeWidth(w) => self.controls.stroke_width = w,
            WidthSet(w) => {
                self.controls.width = w;
            }
            Width => self.draw(NotSet),
            HeightSet(h) => self.controls.height = h,
            Height => self.draw(NotSet),
            Message::GrainColor(c) => match c {
                ColorPickerMessage::Choose => self.controls.show_grain_color_picker = true,
                ColorPickerMessage::Submit(k) => {
                    self.controls.grain_color = k;
                    self.controls.show_grain_color_picker = false;
                    self.draw(NotSet)
                }
                ColorPickerMessage::Cancel => self.controls.show_grain_color_picker = false,
            },
            Message::Background(b) => {
                self.controls.background = Some(b);
                self.draw(NotSet);
            }
            Border(b) => {
                self.controls.border = b;
                self.draw(NotSet);
            }
            Null => {}
            Sinusoid(s) => match s {
                SineMessage::Draw => self.draw(NotSet),
                _ => self.controls.sin_controls.update(s),
            },
            ColorMode(c) => {
                self.controls.color_mode_controls.update(c);
                if self.controls.color_mode_controls.dirty {
                    self.draw(NotSet)
                };
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        use crate::Background::*;
        use crate::NoiseFunction::*;
        use crate::Preset::*;
        use Message::*;
        let img_view = image::viewer(self.image.clone()).min_scale(1.0);
        let mut left_panel = iced::widget::column![];
        let mut right_panel = iced::widget::column![];
        let grain_color_button = button(text("Grain Color").size(15))
            .on_press(Message::GrainColor(ColorPickerMessage::Choose));
        let grain_color_picker = ColorPicker::new(
            self.controls.show_grain_color_picker,
            self.controls.grain_color,
            grain_color_button,
            Message::GrainColor(ColorPickerMessage::Cancel),
            |c| Message::GrainColor(ColorPickerMessage::Submit(c)),
        );
        let color_mode = crate::ColorControls::new(
            self.controls.color_mode_controls.mode,
            self.controls.color_mode_controls.anchor1,
            self.controls.color_mode_controls.anchor2,
            self.controls.color_mode_controls.show_picker_1,
            self.controls.color_mode_controls.show_picker_2,
            self.controls.color_mode_controls.palette_choice,
            self.controls.color_mode_controls.dirty,
        )
        .view()
        .map(Message::ColorMode);

        right_panel = right_panel.push(vertical_space(5.0));
        left_panel = left_panel
            .push(vertical_space(5.0))
            .push(
                row!(
                    text("Width").size(15).width(90),
                    text("Height").size(15).width(90)
                )
                .spacing(15),
            )
            .push(
                row!(
                    text_input("1000", &self.controls.width)
                        .on_input(WidthSet)
                        .size(15)
                        .width(90)
                        .on_submit(Width),
                    text_input("1000", &self.controls.height)
                        .on_input(HeightSet)
                        .size(15)
                        .width(90)
                        .on_submit(Height),
                )
                .spacing(15),
            )
            .push(lpicklist::LPickList::new(
                "Preset".to_string(),
                vec![
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
                ],
                self.controls.preset,
                |x| x.map_or(Null, Preset),
            ))
            .push(lpicklist::LPickList::new(
                "Curve Style".to_string(),
                vec![
                    crate::CurveStyle::Line,
                    crate::CurveStyle::Dots,
                    crate::CurveStyle::Extrusion,
                ],
                self.controls.curve_style,
                |x| x.map_or(CurveStyle(common::CurveStyle::Dots), CurveStyle),
            ))
            .push(
                row([
                    common::CurveDirection::OneSided,
                    common::CurveDirection::TwoSided,
                ]
                .iter()
                .cloned()
                .map(|d| {
                    radio(d, d, self.controls.curve_direction, CurveDirection)
                        .text_size(15)
                        .size(15)
                })
                .map(Element::from)
                .collect())
                .spacing(15),
            )
            .push(lpicklist::LPickList::new(
                "Flow Field".to_string(),
                vec![
                    Fbm, Billow, Ridged, Value, Cylinders, Worley, Curl, Magnet, Gravity,
                    Sinusoidal,
                ],
                self.controls.noise_controls.noise_function,
                |x| x.map_or(Null, Noise),
            ))
            .push(lpicklist::LPickList::new(
                "Curve Locations".to_string(),
                vec![
                    Location::Grid,
                    Location::Rand,
                    Location::Halton,
                    Location::Poisson,
                    Location::Circle,
                    Location::Lissajous,
                ],
                self.controls.location,
                |x| x.map_or(Null, Loc),
            ))
            .push(lpicklist::LPickList::new(
                "Background Style".to_string(),
                vec![LightGrain, LightClouds, DarkGrain, DarkClouds, ColorGrain],
                self.controls.background,
                |x| x.map_or(Null, Background),
            ))
            .push(
                LSlider::new(
                    "Density".to_string(),
                    self.controls.density,
                    5.0..=100.0,
                    5.0,
                    Density,
                    Draw(NotSet),
                )
                .decimals(0),
            )
            .push(
                LSlider::new(
                    "Point Spacing".to_string(),
                    self.controls.spacing,
                    1.0..=100.0,
                    1.0,
                    Space,
                    Draw(NotSet),
                )
                .decimals(0),
            )
            .push(LSlider::new(
                "Curve Length".to_string(),
                self.controls.curve_length,
                0..=500,
                1,
                CurveLength,
                Draw(NotSet),
            ));
        left_panel = left_panel
            .push(LSlider::new(
                "Noise Scale".to_string(),
                self.controls.noise_controls.noise_scale,
                0.5..=20.0,
                0.1,
                NoiseScale,
                Draw(NotSet),
            ))
            .push(LSlider::new(
                "Noise Factor".to_string(),
                self.controls.noise_controls.noise_factor,
                0.5..=20.0,
                0.1,
                Factor,
                Draw(NotSet),
            ))
            .push(
                LSlider::new(
                    "Convergence Speed".to_string(),
                    self.controls.speed,
                    0.01..=1.00,
                    0.01,
                    Speed,
                    Draw(NotSet),
                )
                .decimals(2),
            )
            .push(color_mode);
        if self.controls.curve_style == Some(crate::CurveStyle::Extrusion) {
            let extrusion = ExtrudeControls::new(
                self.controls.extrude_controls.size_controls,
                self.controls.extrude_controls.grad_style,
                self.controls.extrude_controls.dirty,
            );
            right_panel = right_panel.push(extrusion.view().map(Message::Extrude));
        } else if self.controls.curve_style == Some(crate::CurveStyle::Dots) {
            let dot = crate::DotControls::new(
                self.controls.dot_controls.dot_style,
                self.controls.dot_controls.size_controls,
                self.controls.dot_controls.pearl_sides,
                self.controls.dot_controls.pearl_smoothness,
                self.controls.dot_controls.show_color_picker,
                self.controls.dot_controls.dot_stroke_color,
                self.controls.dot_controls.dirty,
            );
            right_panel = right_panel.push(dot.view().map(Message::Dot))
        };
        if self.controls.noise_controls.noise_function == Some(Fbm)
            || self.controls.noise_controls.noise_function == Some(Billow)
            || self.controls.noise_controls.noise_function == Some(Ridged)
        {
            right_panel = right_panel.push(
                Fractal::new(
                    self.controls.fractal_controls.octaves,
                    self.controls.fractal_controls.persistence,
                    self.controls.fractal_controls.lacunarity,
                    self.controls.fractal_controls.frequency,
                )
                .show(),
            )
        }
        if self.controls.noise_controls.noise_function == Some(Sinusoidal) {
            right_panel = right_panel.push(
                SineControls::new(
                    self.controls.sin_controls.xfreq,
                    self.controls.sin_controls.yfreq,
                    self.controls.sin_controls.xexp,
                    self.controls.sin_controls.yexp,
                )
                .view()
                .map(Message::Sinusoid),
            )
        }
        if self.controls.background == Some(ColorGrain) {
            right_panel = right_panel.push(Rule::horizontal(15)).push(
                row![
                    grain_color_picker,
                    text(format!(
                        "{:3} {:3} {:3}",
                        (self.controls.grain_color.r * 255.0) as u8,
                        (self.controls.grain_color.g * 255.0) as u8,
                        (self.controls.grain_color.b * 255.0) as u8
                    ))
                    .size(15)
                ]
                .spacing(15)
                .align_items(Center),
            );
        }
        left_panel = left_panel
            .push(
                LSlider::new(
                    "Stroke Width".to_string(),
                    self.controls.stroke_width,
                    0.0..=25.0,
                    0.5,
                    StrokeWidth,
                    Draw(NotSet),
                )
                .decimals(1),
            )
            .push(Container::new(
                toggler("Border".to_owned(), self.controls.border, Border).text_size(TEXT_SIZE),
            ))
            .padding(20)
            .spacing(15)
            .width(250);

        let export_button = if self.controls.exporting {
            button(text("Export").size(15))
        } else {
            button(text("Export").size(15)).on_press(Export)
        };
        left_panel = left_panel.push(export_button).spacing(12);
        let img_container = Container::new(img_view).width(self.width).height(1000);
        let image_panel =
            iced::widget::column!(vertical_space(25), img_container, vertical_space(5),)
                .spacing(15)
                .align_items(Alignment::Center);

        right_panel = right_panel.padding(20).spacing(15).width(250);
        row![left_panel, image_panel, right_panel].into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
