use iced::{
    widget::{button, image, row, text, text_input, toggler, vertical_space, Container},
    Alignment::{self, Center},
    Application, Color, Command, Element, Settings, Theme,
};
use iced_aw::ColorPicker;

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

use crate::art::print;
use crate::background::Background;
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

#[derive(Debug, Clone)]
pub enum ColorChooser {
    Color1,
    Color2,
}

#[derive(Debug, Clone)]
pub enum ColorMessage {
    Choose,
    Submit(Color),
    Cancel,
}

#[derive(Debug, Clone)]
pub enum Message {
    Preset(Preset),
    CurveStyle(CurveStyle),
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
    Color1(ColorMessage),
    Color2(ColorMessage),
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
        use crate::common::Preset::*;
        use Message::*;
        use PresetState::*;
        match message {
            Preset(p) => {
                self.controls = match p {
                    RustyRibbons => rusty_ribbons(),
                    Solar => solar(),
                    RiverStones => river_stones(),
                    Purple => purple(),
                    Canyon => canyon(),
                    Stripes => stripes(),
                    Splat => splat(),
                    Tubes => tubes(),
                    Ducts => ducts(),
                    Ridges => ridges(),
                };
                self.controls.preset = Some(p);
                self.draw(Set);
            }
            CurveStyle(cs) => {
                self.controls.curve_style = Some(cs);
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
            Message::Color1(c) => match c {
                ColorMessage::Choose => self.controls.show_color_picker1 = true,
                ColorMessage::Submit(k) => {
                    self.controls.color1 = k;
                    self.controls.show_color_picker1 = false;
                    self.draw(NotSet)
                }
                ColorMessage::Cancel => self.controls.show_color_picker1 = false,
            },
            Message::Color2(c) => match c {
                ColorMessage::Choose => self.controls.show_color_picker2 = true,
                ColorMessage::Submit(k) => {
                    self.controls.color2 = k;
                    self.controls.show_color_picker2 = false;
                    self.draw(NotSet)
                }
                ColorMessage::Cancel => self.controls.show_color_picker2 = false,
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
        let color_button1 =
            button(text("Anchor 1 Color").size(15)).on_press(Message::Color1(ColorMessage::Choose));
        let color_button2 =
            button(text("Anchor 2 Color").size(15)).on_press(Message::Color2(ColorMessage::Choose));

        let color_picker1 = ColorPicker::new(
            self.controls.show_color_picker1,
            self.controls.color1,
            color_button1,
            Message::Color1(ColorMessage::Cancel),
            |c| Message::Color1(ColorMessage::Submit(c)),
        );
        let color_picker2 = ColorPicker::new(
            self.controls.show_color_picker2,
            self.controls.color2,
            color_button2,
            Message::Color2(ColorMessage::Cancel),
            |c| Message::Color2(ColorMessage::Submit(c)),
        );

        right_panel = right_panel.push(vertical_space(5.0));
        left_panel = left_panel
            .push(vertical_space(5.0))
            .push(
                row!(
                    text_input("Width", &self.controls.width, WidthSet)
                        .size(15)
                        .width(90)
                        .on_submit(Width),
                    text_input("Height", &self.controls.height, HeightSet)
                        .size(15)
                        .width(90)
                        .on_submit(Height),
                )
                .spacing(15),
            )
            .push(lpicklist::LPickList::new(
                "Preset".to_string(),
                vec![
                    RustyRibbons,
                    Solar,
                    RiverStones,
                    Purple,
                    Canyon,
                    Stripes,
                    Splat,
                    Tubes,
                    Ducts,
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
                vec![LightGrain, LightClouds, DarkGrain, DarkClouds],
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
            .push(
                row![
                    color_picker1,
                    text(format!(
                        "{:3} {:3} {:3}",
                        (self.controls.color1.r * 255.0) as u8,
                        (self.controls.color1.g * 255.0) as u8,
                        (self.controls.color1.b * 255.0) as u8
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
                        (self.controls.color2.r * 255.0) as u8,
                        (self.controls.color2.g * 255.0) as u8,
                        (self.controls.color2.b * 255.0) as u8
                    ))
                    .size(15)
                ]
                .spacing(15)
                .align_items(Center),
            );

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
        left_panel = left_panel.push(export_button).spacing(15);
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
