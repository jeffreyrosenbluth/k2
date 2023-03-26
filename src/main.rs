use iced::{
    widget::{button, image, row, text, text_input, toggler, vertical_space, Container},
    Alignment::{self, Center},
    Application, Color, Command, Element, Settings, Theme,
};
use iced_aw::ColorPicker;
use rand::prelude::*;

mod art;
mod background;
mod color;
mod common;
mod field;
mod gradient;
mod gui;
mod location;
mod noise;
mod size;

use crate::common::*;
use crate::gradient::GradStyle;
use crate::gui::{dot::Dot, extrude::Extrude, fractal::Fractal, lpicklist, lslider::LSlider};
use crate::location::Location;
use crate::noise::NoiseFunction;
use crate::size::{Dir, SizeFn};
use crate::{art::*, background::Background};

const TEXT_SIZE: u16 = 15;

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (1480, 1100);
    Xtrusion::run(settings)
}

#[derive(Debug, Clone)]
pub enum ColorChooser {
    Color1,
    Color2,
}

#[derive(Debug, Clone)]
pub enum RandomMessage {
    RandomLength,
    RandomNoiseFactor,
    RandomNoiseScale,
    RandomOctaves,
    RandomColor(ColorChooser),
    RandomSpeed,
    RandomPersistence,
    RandomLenSize,
    RandomNoiseFunction,
    RandomLocation,
    RandomLenType,
    RandomLenDir,
    RandomHighlight,
    RandomCurveStyle,
    RandomBackground,
    RandomSpacing,
    RandomDensity,
    RandomStrokeWidth,
    RandomLacunarity,
    RandomFrequency,
}

#[derive(Debug, Clone)]
pub enum ColorMessage {
    Choose,
    Submit(Color),
    Cancel,
}

#[derive(Debug, Clone)]
pub enum Message {
    HiRes(bool),
    CurveStyle(CurveStyle),
    Space(f32),
    CurveLength(u32),
    Export,
    Draw,
    Loc(Location),
    Density(f32),
    Octaves(u8),
    Factor(f32),
    Scale(f32),
    Persistence(f32),
    Lacunarity(f32),
    Frequency(f32),
    Noise(NoiseFunction),
    Speed(f32),
    Length(SizeFn),
    LengthSize(f32),
    LengthDir(Dir),
    Grad(GradStyle),
    Randomize,
    ExportComplete(()),
    StrokeWidth(f32),
    ExportWidth(String),
    ExportHeight(String),
    Rand(RandomMessage),
    Color1(ColorMessage),
    Color2(ColorMessage),
    Background(Background),
    Border(bool),
    Null,
}

fn rand_message(message: RandomMessage, controls: &mut Controls) {
    use RandomMessage::*;
    let mut rng = SmallRng::from_entropy();
    let random_controls: Controls = rng.gen();
    match message {
        RandomLength => {
            controls.curve_length = rng.gen_range(140..=360) / controls.spacing as u32;
        }
        RandomNoiseFactor => {
            controls.noise_factor = random_controls.noise_factor;
        }
        RandomNoiseScale => {
            controls.noise_scale = random_controls.noise_scale;
        }
        RandomOctaves => {
            controls.octaves = random_controls.octaves;
        }
        RandomSpeed => {
            controls.speed = rng.gen_range(0.0..=1.0);
        }
        RandomPersistence => {
            controls.persistence = rng.gen_range(0.05..=0.95);
        }
        RandomLenSize => {
            controls.size = random_controls.size;
        }
        RandomNoiseFunction => {
            controls.noise_function = random_controls.noise_function;
        }
        RandomLocation => {
            controls.location = random_controls.location;
        }
        RandomLenType => {
            controls.size_fn = random_controls.size_fn;
        }
        RandomLenDir => {
            controls.direction = random_controls.direction;
        }
        RandomHighlight => {
            controls.grad_style = random_controls.grad_style;
        }
        RandomColor(c) => match c {
            ColorChooser::Color1 => {
                controls.color1 = random_controls.color1;
            }
            ColorChooser::Color2 => {
                controls.color2 = random_controls.color2;
            }
        },
        RandomCurveStyle => {
            controls.curve_style = random_controls.curve_style;
        }
        RandomBackground => {
            controls.background = random_controls.background;
        }
        RandomSpacing => {
            controls.spacing = random_controls.spacing;
        }
        RandomDensity => {
            controls.density = random_controls.density;
        }
        RandomStrokeWidth => {
            controls.stroke_width = random_controls.stroke_width;
        }
        RandomLacunarity => {
            controls.lacunarity = random_controls.lacunarity;
        }
        RandomFrequency => {
            controls.frequency = random_controls.frequency;
        }
    }
}

impl Application for Xtrusion {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Xtrusion, Command<Message>) {
        (Self::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Xtrusion")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        use Message::*;
        let controls = self.controls.clone();
        match message {
            HiRes(b) => {
                self.controls.hi_res = b;
                if b {
                    self.controls.spacing = 1.0;
                    self.controls.curve_length = 200;
                    self.controls.stroke_width = 2.0;
                } else {
                    self.controls.spacing = 4.0;
                    self.controls.curve_length = 50;
                    self.controls.stroke_width = 8.0;
                }
                self.draw();
            }
            CurveStyle(cs) => {
                self.controls.curve_style = Some(cs);
                if cs == common::CurveStyle::Dots {
                    self.controls.stroke_width = 1.0;
                }
                if cs == common::CurveStyle::Line {
                    self.controls.spacing = 1.0;
                }
                self.draw();
            }
            Space(b) => self.controls.spacing = b,
            CurveLength(l) => self.controls.curve_length = l,
            Export => {
                self.controls.exporting = true;
                return Command::perform(print(controls, 1.0), ExportComplete);
            }
            Loc(loc) => {
                self.controls.location = Some(loc);
                self.draw();
            }
            Density(s) => self.controls.density = s,
            Draw => {
                self.draw();
            }
            Octaves(o) => self.controls.octaves = o,
            Persistence(p) => self.controls.persistence = p,
            Lacunarity(l) => self.controls.lacunarity = l,
            Frequency(f) => self.controls.frequency = f,
            Factor(f) => self.controls.noise_factor = f,
            Scale(s) => self.controls.noise_scale = s,
            Noise(n) => {
                self.controls.noise_function = Some(n);
                if n == NoiseFunction::Cylinders {
                    self.controls.noise_scale = 1.0;
                    self.controls.noise_factor = 1.0;
                    self.controls.octaves = 2;
                }
                self.draw();
            }
            Speed(s) => self.controls.speed = s,
            Length(l) => {
                self.controls.size_fn = Some(l);
                self.draw();
            }
            LengthSize(s) => self.controls.size = s,
            LengthDir(d) => {
                self.controls.direction = Some(d);
                self.draw();
            }
            Grad(c) => {
                self.controls.grad_style = Some(c);
                self.draw();
            }
            Randomize => {
                let w = self.controls.export_width.clone();
                let h = self.controls.export_height.clone();
                self.controls.randomize(); // = rng.gen();
                self.controls.export_width = w;
                self.controls.export_height = h;
                self.draw();
            }
            ExportComplete(_) => self.controls.exporting = false,
            StrokeWidth(w) => self.controls.stroke_width = w,
            ExportWidth(w) => self.controls.export_width = w,
            ExportHeight(h) => self.controls.export_height = h,
            Rand(rnd) => {
                rand_message(rnd, &mut self.controls);
                self.draw();
            }
            Message::Color1(c) => match c {
                ColorMessage::Choose => self.controls.show_color_picker1 = true,
                ColorMessage::Submit(k) => {
                    self.controls.color1 = k;
                    self.controls.show_color_picker1 = false;
                    self.draw()
                }
                ColorMessage::Cancel => self.controls.show_color_picker1 = false,
            },
            Message::Color2(c) => match c {
                ColorMessage::Choose => self.controls.show_color_picker2 = true,
                ColorMessage::Submit(k) => {
                    self.controls.color2 = k;
                    self.controls.show_color_picker2 = false;
                    self.draw()
                }
                ColorMessage::Cancel => self.controls.show_color_picker2 = false,
            },
            Message::Background(b) => {
                self.controls.background = Some(b);
                self.draw();
            }
            Null => {}
            Border(b) => {
                self.controls.border = b;
                self.draw();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        use crate::Background::*;
        use crate::NoiseFunction::*;
        use Message::*;
        use RandomMessage::*;
        let img_view = image::viewer(self.image.clone()).min_scale(1.0);
        let mut left_panel = iced::widget::column![];
        let mut right_panel = iced::widget::column![];
        let color_button1 =
            button(text("Color 1").size(15)).on_press(Message::Color1(ColorMessage::Choose));
        let color_button2 =
            button(text("Color 2").size(15)).on_press(Message::Color2(ColorMessage::Choose));

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
            .push(Container::new(
                toggler("Hi Res".to_owned(), self.controls.hi_res, HiRes).text_size(TEXT_SIZE),
            ))
            .push(lpicklist::LPickList::new(
                "Curve Style".to_string(),
                vec![
                    crate::CurveStyle::Line,
                    crate::CurveStyle::Dots,
                    crate::CurveStyle::Extrusion,
                ],
                self.controls.curve_style,
                |x| x.map_or(CurveStyle(common::CurveStyle::Dots), |v| CurveStyle(v)),
                Rand(RandomCurveStyle),
            ))
            .push(
                LSlider::new(
                    "Density".to_string(),
                    self.controls.density,
                    5.0..=100.0,
                    1.0,
                    Density,
                    Some(Rand(RandomDensity)),
                    Draw,
                )
                .decimals(0),
            )
            .push(
                LSlider::new(
                    "Point Spacing".to_string(),
                    self.controls.spacing,
                    1.0..=50.0,
                    1.0,
                    Space,
                    Some(Rand(RandomSpacing)),
                    Draw,
                )
                .decimals(0),
            )
            .push(LSlider::new(
                "Curve Length".to_string(),
                self.controls.curve_length,
                10..=1000,
                1,
                CurveLength,
                Some(Rand(RandomLength)),
                Draw,
            ))
            .push(lpicklist::LPickList::new(
                "Flow Field".to_string(),
                vec![
                    Fbm, Billow, Ridged, Value, Cylinders, Worley, Curl, Magnet, Gravity,
                ],
                self.controls.noise_function,
                |x| x.map_or(Noise(Fbm), |v| Noise(v)),
                Rand(RandomNoiseFunction),
            ));
        left_panel = left_panel
            .push(LSlider::new(
                "Noise Scale".to_string(),
                self.controls.noise_scale,
                0.5..=20.0,
                0.1,
                Scale,
                Some(Rand(RandomNoiseScale)),
                Draw,
            ))
            .push(LSlider::new(
                "Noise Factor".to_string(),
                self.controls.noise_factor,
                0.5..=20.0,
                0.1,
                Factor,
                Some(Rand(RandomNoiseFactor)),
                Draw,
            ))
            .push(
                LSlider::new(
                    "Convergence Speed".to_string(),
                    self.controls.speed,
                    0.01..=1.00,
                    0.01,
                    Speed,
                    Some(Rand(RandomSpeed)),
                    Draw,
                )
                .decimals(2),
            )
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
                |x| x.map_or(Loc(Location::Grid), |x| Loc(x)),
                Rand(RandomLocation),
            ))
            .push(
                row![
                    color_picker1,
                    button(text("Random").size(15))
                        .on_press(Rand(RandomColor(ColorChooser::Color1))),
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
                    button(text("Random").size(15))
                        .on_press(Rand(RandomColor(ColorChooser::Color2))),
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
            let extrusion = Extrude::new(
                self.controls.size_fn,
                self.controls.size,
                self.controls.direction,
                self.controls.grad_style,
            );
            right_panel = right_panel.push(extrusion.show())
        } else if self.controls.curve_style == Some(crate::CurveStyle::Dots) {
            let dot = Dot::new(
                self.controls.size_fn,
                self.controls.size,
                self.controls.direction,
            );
            right_panel = right_panel.push(dot.show())
        };
        if self.controls.noise_function == Some(Fbm)
            || self.controls.noise_function == Some(Billow)
            || self.controls.noise_function == Some(Ridged)
        {
            right_panel = right_panel.push(
                Fractal::new(
                    self.controls.octaves,
                    self.controls.persistence,
                    self.controls.lacunarity,
                    self.controls.frequency,
                )
                .show(),
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
                    Some(Rand(RandomStrokeWidth)),
                    Draw,
                )
                .decimals(1),
            )
            .push(lpicklist::LPickList::new(
                "Background Style".to_string(),
                vec![Grain, Clouds, DarkGrain, DarkClouds],
                self.controls.background,
                |x| x.map_or(Background(Grain), |v| Background(v)),
                Rand(RandomBackground),
            ))
            .push(Container::new(
                toggler("Border".to_owned(), self.controls.border, Border).text_size(TEXT_SIZE),
            ))
            .padding(20)
            .spacing(15)
            .width(250);

        let rand_button = button(text("Random").size(15)).on_press(Randomize);
        let export_button = if self.controls.exporting {
            button(text("Export").size(15))
        } else {
            button(text("Export").size(15)).on_press(Export)
        };
        left_panel = left_panel
            .push(row!(rand_button, export_button).spacing(20))
            .push(
                row!(
                    text_input("Export Width", &self.controls.export_width, ExportWidth)
                        .size(15)
                        .width(90),
                    text_input("Export Height", &self.controls.export_height, ExportHeight)
                        .size(15)
                        .width(90)
                )
                .spacing(15),
            );
        let image_panel = iced::widget::column!(vertical_space(25), img_view, vertical_space(5),)
            .spacing(15)
            .align_items(Alignment::Center);

        right_panel = right_panel.padding(20).spacing(15).width(250);
        row![left_panel, image_panel, right_panel].into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
