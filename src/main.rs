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
mod length;
mod location;
mod noise;

use crate::common::*;
use crate::gradient::GradStyle;
use crate::gui::{extrude::Extrude, helpers::*};
use crate::length::{Dir, ExtrusionStyle};
use crate::location::Location;
use crate::noise::NoiseFunction;
use crate::{art::*, background::Background};

const TEXT_SIZE: u16 = 15;

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (1300, 1300);
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
    GridSep(f32),
    Octaves(u8),
    Factor(f32),
    Scale(f32),
    Persistence(f32),
    Noise(NoiseFunction),
    Speed(f32),
    Length(ExtrusionStyle),
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
            controls.len_size = random_controls.len_size;
        }
        RandomNoiseFunction => {
            controls.noise_function = random_controls.noise_function;
        }
        RandomLocation => {
            controls.location = random_controls.location;
        }
        RandomLenType => {
            controls.len_type = random_controls.len_type;
        }
        RandomLenDir => {
            controls.len_dir = random_controls.len_dir;
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
                    self.controls.spacing /= 4.0;
                    // self.controls.curve_length *= 4;
                    self.controls.stroke_width /= 4.0;
                } else {
                    self.controls.spacing *= 4.0;
                    // self.controls.curve_length /= 4;
                    self.controls.stroke_width *= 4.0;
                }
                self.draw();
            }
            CurveStyle(cs) => {
                self.controls.curve_style = Some(cs);
                match cs {
                    crate::common::CurveStyle::Line => {
                        self.controls.stroke_width = 2.0;
                        self.controls.spacing = 1.0;
                        self.controls.curve_length = 250;
                        self.controls.grid_sep = 25.0;
                    }
                    crate::common::CurveStyle::Dots => {
                        self.controls.spacing = 30.0;
                        self.controls.curve_length = 250;
                        self.controls.grid_sep = 50.0;
                    }
                    crate::common::CurveStyle::Extrusion => {
                        self.controls.stroke_width = 8.0;
                        self.controls.spacing = 4.0;
                        self.controls.curve_length = 250;
                        self.controls.grid_sep = 50.0;
                    }
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
            GridSep(s) => self.controls.grid_sep = s,
            Draw => {
                self.draw();
            }
            Octaves(o) => self.controls.octaves = o,
            Persistence(p) => self.controls.persistence = p,
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
                self.controls.len_type = Some(l);
                self.draw();
            }
            LengthSize(s) => self.controls.len_size = s,
            LengthDir(d) => {
                self.controls.len_dir = Some(d);
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
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        use crate::Background::*;
        use crate::NoiseFunction::*;
        use Message::*;
        use RandomMessage::*;
        let img_view = image::viewer(self.image.clone()).min_scale(1.0);
        let mut control_panel = iced::widget::column![];
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

        let extrusion = Extrude::new(
            self.controls.len_type,
            self.controls.len_size,
            self.controls.len_dir,
            self.controls.grad_style,
        );

        control_panel = control_panel
            .push(Container::new(
                toggler("Hi Res".to_owned(), self.controls.hi_res, HiRes).text_size(TEXT_SIZE),
            ))
            .push(
                PickListBuilder::new(
                    "Curve Style".to_string(),
                    vec![
                        crate::CurveStyle::Line,
                        crate::CurveStyle::Dots,
                        crate::CurveStyle::Extrusion,
                    ],
                    self.controls.curve_style,
                    CurveStyle,
                    Rand(RandomCurveStyle),
                )
                .build(),
            )
            .push(
                SliderBuilder::new(
                    "Spacing".to_string(),
                    Space,
                    Draw,
                    None,
                    self.controls.spacing,
                )
                .range(1.0..=50.0)
                .decimals(0)
                .build(),
            )
            .push(
                SliderBuilder::new(
                    "Curve Length".to_string(),
                    CurveLength,
                    Draw,
                    Some(Rand(RandomLength)),
                    self.controls.curve_length,
                )
                .range(10..=1000)
                .build(),
            )
            .push(
                PickListBuilder::new(
                    "Flow Field".to_string(),
                    vec![
                        Fbm, Billow, Ridged, Value, Cylinders, Worley, Curl, Magnet, Gravity,
                    ],
                    self.controls.noise_function,
                    Noise,
                    Rand(RandomNoiseFunction),
                )
                .build(),
            );
        control_panel = control_panel.push(
            SliderBuilder::new(
                "Octaves".to_string(),
                Octaves,
                Draw,
                Some(Rand(RandomOctaves)),
                self.controls.octaves,
            )
            .range(1..=8)
            .decimals(0)
            .build(),
        );
        // };
        control_panel = control_panel
            .push(
                SliderBuilder::new(
                    "Noise Scale".to_string(),
                    Scale,
                    Draw,
                    Some(Rand(RandomNoiseScale)),
                    self.controls.noise_scale,
                )
                .range(0.5..=20.0)
                .step(0.1)
                .build(),
            )
            .push(
                SliderBuilder::new(
                    "Noise Factor".to_string(),
                    Factor,
                    Draw,
                    Some(Rand(RandomNoiseFactor)),
                    self.controls.noise_factor,
                )
                .range(0.5..=20.0)
                .step(0.1)
                .build(),
            )
            .push(
                SliderBuilder::new(
                    "Persistence".to_string(),
                    Persistence,
                    Draw,
                    Some(Rand(RandomPersistence)),
                    self.controls.persistence,
                )
                .range(0.05..=0.95)
                .step(0.05)
                .decimals(2)
                .build(),
            )
            .push(
                SliderBuilder::new(
                    "Speed".to_string(),
                    Speed,
                    Draw,
                    Some(Rand(RandomSpeed)),
                    self.controls.speed,
                )
                .range(0.01..=1.00)
                .step(0.01)
                .decimals(2)
                .build(),
            )
            .push(
                PickListBuilder::new(
                    "Location".to_string(),
                    vec![
                        Location::Grid,
                        Location::Rand,
                        Location::Halton,
                        Location::Poisson,
                        Location::Circle,
                        Location::Lissajous,
                    ],
                    self.controls.location,
                    Loc,
                    Rand(RandomLocation),
                )
                .build(),
            )
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
            )
            .push(
                SliderBuilder::new(
                    "Grid Spacing".to_string(),
                    GridSep,
                    Draw,
                    None,
                    self.controls.grid_sep,
                )
                .range(5.0..=100.0)
                .decimals(0)
                .build(),
            );

        if self.controls.curve_style == Some(crate::CurveStyle::Extrusion) {
            control_panel = control_panel.push(extrusion.show())
        };

        control_panel = control_panel
            .push(
                SliderBuilder::new(
                    "Stroke Width".to_string(),
                    StrokeWidth,
                    Draw,
                    None,
                    self.controls.stroke_width,
                )
                .range(0.5..=25.0)
                .step(0.5)
                .decimals(1)
                .build(),
            )
            .push(
                PickListBuilder::new(
                    "Background Style".to_string(),
                    vec![Grain, Clouds],
                    self.controls.background,
                    Background,
                    Rand(RandomBackground),
                )
                .build(),
            )
            .padding(20)
            .spacing(15)
            .width(250);

        let rand_button = button("Random").on_press(Randomize);
        let export_button = if self.controls.exporting {
            button("Export")
        } else {
            button("Export").on_press(Export)
        };
        let image_panel = iced::widget::column!(
            vertical_space(25),
            img_view,
            vertical_space(25),
            row!(rand_button, export_button).spacing(100),
            text_input("Export Width", &self.controls.export_width, ExportWidth).width(200),
            text_input("Export Height", &self.controls.export_height, ExportHeight).width(200),
        )
        .spacing(20)
        .align_items(Alignment::Center);
        row![control_panel, image_panel,].into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
