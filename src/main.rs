use iced::{
    widget::{button, image, row, text_input, toggler, vertical_space, Container},
    Alignment, Application, Command, Element, Settings, Theme,
};
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

use crate::art::*;
use crate::common::*;
use crate::gradient::GradStyle;
use crate::gui::*;
use crate::length::{Dir, Len};
use crate::location::Location;
use crate::noise::NoiseFunction;

const TEXT_SIZE: u16 = 15;

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (1300, 1300);
    Xtrusion::run(settings)
}

#[derive(Debug, Clone)]
pub enum RandomMessage {
    RandomLength,
    RandomNoiseFactor,
    RandomNoiseScale,
    RandomOctaves,
    RandomSpeed,
    RandomPersistence,
    RandomPalette,
    RandomHue,
    RandomLenSize,
    RandomNoiseFunction,
    RandomLocation,
    RandomLenType,
    RandomLenDir,
    RandomHighlight,
}

#[derive(Debug, Clone)]
pub enum Message {
    HiRes(bool),
    Xtrude(bool),
    Space(f32),
    CurveLength(u32),
    Palette(u8),
    Hue(u16),
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
    Length(Len),
    LengthSize(f32),
    LengthDir(Dir),
    Grad(GradStyle),
    Randomize,
    ExportComplete(()),
    StrokeWidth(f32),
    ExportWidth(String),
    ExportHeight(String),
    Rand(RandomMessage),
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
        RandomPalette => {
            controls.palette_num = random_controls.palette_num;
        }
        RandomHue => {
            controls.hue = rng.gen_range(0..360);
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
        let mut rng = SmallRng::from_entropy();
        let controls = self.controls.clone();
        match message {
            HiRes(b) => {
                self.controls.hi_res = b;
                if b {
                    self.controls.spacing /= 4.0;
                    self.controls.curve_length *= 4;
                    self.controls.stroke_width /= 4.0;
                } else {
                    self.controls.spacing *= 4.0;
                    self.controls.curve_length /= 4;
                    self.controls.stroke_width *= 4.0;
                }
                self.draw();
            }
            Xtrude(b) => {
                self.controls.xtrude = b;
                if !b {
                    self.controls.stroke_width = 2.0
                };
                self.draw();
            }
            Space(b) => self.controls.spacing = b,
            CurveLength(l) => self.controls.curve_length = l,
            Export => {
                self.controls.exporting = true;
                return Command::perform(print(controls, 1.0), ExportComplete);
            }

            Hue(hue) => self.controls.hue = hue,
            Palette(p) => self.controls.palette_num = p,
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
                self.controls = rng.gen();
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
            Null => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        use Message::*;
        use RandomMessage::*;
        let img_view = image::viewer(self.image.clone()).min_scale(1.0);
        let mut control_panel = iced::widget::column![];
        control_panel = control_panel
            .push(Container::new(
                toggler("Hi Res".to_owned(), self.controls.hi_res, HiRes).text_size(TEXT_SIZE),
            ))
            .push(Container::new(
                toggler("Xtrude".to_owned(), self.controls.xtrude, Xtrude).text_size(TEXT_SIZE),
            ))
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
                    "Max Length".to_string(),
                    CurveLength,
                    Draw,
                    Some(Rand(RandomLength)),
                    self.controls.curve_length,
                )
                .range(10..=350)
                .build(),
            )
            .push(
                PickListBuilder::new(
                    "Noise Function".to_string(),
                    vec![
                        NoiseFunction::Fbm,
                        NoiseFunction::Billow,
                        NoiseFunction::Ridged,
                        NoiseFunction::Value,
                        NoiseFunction::Checkerboard,
                        NoiseFunction::Cylinders,
                        NoiseFunction::Worley,
                        NoiseFunction::Curl,
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
                SliderBuilder::new(
                    "Palette".to_string(),
                    Palette,
                    Draw,
                    Some(Rand(RandomPalette)),
                    self.controls.palette_num,
                )
                .range(0..=10)
                .step(1)
                .decimals(0)
                .build(),
            )
            .push(
                SliderBuilder::new(
                    "Hue".to_string(),
                    Hue,
                    Draw,
                    Some(Rand(RandomHue)),
                    self.controls.hue,
                )
                .range(0..=360)
                .step(5)
                .decimals(0)
                .build(),
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
            )
            .push(
                PickListBuilder::new(
                    "Extrusion Length".to_string(),
                    vec![
                        Len::Constant,
                        Len::Expanding,
                        Len::Contracting,
                        Len::Varying,
                        Len::Noisy,
                    ],
                    self.controls.len_type,
                    Length,
                    Rand(RandomLenType),
                )
                .build(),
            )
            .push(
                SliderBuilder::new(
                    "Extrusion Size".to_string(),
                    LengthSize,
                    Draw,
                    Some(Rand(RandomLenSize)),
                    self.controls.len_size,
                )
                .range(1.0..=350.0)
                .decimals(0)
                .build(),
            )
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
                    "Extrusion Direction".to_string(),
                    vec![Dir::Both, Dir::Horizontal, Dir::Vertical],
                    self.controls.len_dir,
                    LengthDir,
                    Rand(RandomLenDir),
                )
                .build(),
            )
            .push(
                PickListBuilder::new(
                    "Highlight".to_string(),
                    vec![
                        GradStyle::None,
                        GradStyle::Light,
                        GradStyle::Dark,
                        GradStyle::Fiber,
                        GradStyle::LightFiber,
                    ],
                    self.controls.grad_style,
                    Grad,
                    Rand(RandomHighlight),
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
