use iced::{
    widget::{button, image, row, text_input, toggler, vertical_space, Container},
    Alignment, Application, Command, Element, Length, Settings, Theme,
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
pub enum Message {
    HiRes(bool),
    Space(f32),
    MaxLength(u32),
    Palette(u8),
    Hue(u16),
    Export,
    Draw,
    Location(Location),
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
    WorleyDistance(bool),
    StrokeWidth(f32),
    ExportWidth(String),
    ExportHeight(String),
    Null,
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
        let controls = self.controls.clone();
        match message {
            Message::HiRes(b) => {
                self.controls.hi_res = b;
                if b {
                    self.controls.spacing /= 4.0;
                    self.controls.max_length *= 4;
                    self.controls.stroke_width /= 4.0;
                } else {
                    self.controls.spacing *= 4.0;
                    self.controls.max_length /= 4;
                    self.controls.stroke_width *= 4.0;
                }
                self.draw();
            }
            Message::Space(b) => self.controls.spacing = b,
            Message::MaxLength(l) => self.controls.max_length = l,
            Message::Export => {
                self.controls.exporting = true;
                return Command::perform(print(controls, 1.0), Message::ExportComplete);
            }

            Message::Hue(hue) => self.controls.hue = hue,
            Message::Palette(p) => self.controls.palette_num = p,
            Message::Location(loc) => {
                self.controls.location = Some(loc);
                self.draw();
            }
            Message::GridSep(s) => self.controls.grid_sep = s,
            Message::Draw => {
                self.draw();
            }
            Message::Octaves(o) => self.controls.octaves = o,
            Message::Persistence(p) => self.controls.persistence = p,
            Message::Factor(f) => self.controls.noise_factor = f,
            Message::Scale(s) => self.controls.noise_scale = s,
            Message::Noise(n) => {
                self.controls.noise_function = Some(n);
                self.draw();
            }
            Message::Speed(s) => self.controls.speed = s,
            Message::Length(l) => {
                self.controls.len_type = Some(l);
                self.draw();
            }
            Message::LengthSize(s) => self.controls.len_size = s,
            Message::LengthDir(d) => {
                self.controls.len_dir = Some(d);
                self.draw();
            }
            Message::Grad(c) => {
                self.controls.grad_style = Some(c);
                self.draw();
            }
            Message::Randomize => {
                let mut rng = SmallRng::from_entropy();
                let w = self.controls.export_width.clone();
                let h = self.controls.export_height.clone();
                self.controls = rng.gen();
                self.controls.export_width = w;
                self.controls.export_height = h;
                self.draw();
            }
            Message::ExportComplete(_) => self.controls.exporting = false,
            Message::WorleyDistance(b) => {
                self.controls.worley_dist = b;
                self.draw();
            }
            Message::StrokeWidth(w) => self.controls.stroke_width = w,
            Message::ExportWidth(w) => self.controls.export_width = w,
            Message::ExportHeight(h) => self.controls.export_height = h,
            Message::Null => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let img_view = image::viewer(self.image.clone()).min_scale(1.0);
        let mut control_panel = iced::widget::column![];
        control_panel = control_panel
            .push(Container::new(
                toggler("Hi Res".to_owned(), self.controls.hi_res, Message::HiRes)
                    .text_size(TEXT_SIZE),
            ))
            .push(
                SliderBuilder::new(
                    "Spacing".to_string(),
                    Message::Space,
                    Message::Draw,
                    self.controls.spacing,
                )
                .range(1.0..=50.0)
                .decimals(0)
                .build(),
            )
            .push(
                SliderBuilder::new(
                    "Max Length".to_string(),
                    Message::MaxLength,
                    Message::Draw,
                    self.controls.max_length,
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
                    Message::Noise,
                )
                .build(),
            );
        if self.controls.noise_function == Some(NoiseFunction::Worley) {
            control_panel = control_panel.push(Container::new(
                toggler(
                    "Distance Function".to_string(),
                    self.controls.worley_dist,
                    Message::WorleyDistance,
                )
                .text_size(TEXT_SIZE),
            ))
        } else {
            control_panel = control_panel.push(
                SliderBuilder::new(
                    "Octaves".to_string(),
                    Message::Octaves,
                    Message::Draw,
                    self.controls.octaves,
                )
                .range(1..=8)
                .decimals(0)
                .build(),
            )
        };
        control_panel = control_panel
            .push(
                SliderBuilder::new(
                    "Noise Scale".to_string(),
                    Message::Scale,
                    Message::Draw,
                    self.controls.noise_scale,
                )
                .range(0.5..=20.0)
                .step(0.1)
                .build(),
            )
            .push(
                SliderBuilder::new(
                    "Noise Factor".to_string(),
                    Message::Factor,
                    Message::Draw,
                    self.controls.noise_factor,
                )
                .range(0.5..=20.0)
                .step(0.1)
                .build(),
            )
            .push(
                SliderBuilder::new(
                    "Persistence".to_string(),
                    Message::Persistence,
                    Message::Draw,
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
                    Message::Speed,
                    Message::Draw,
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
                    Message::Location,
                )
                .build(),
            )
            .push(
                SliderBuilder::new(
                    "Palette".to_string(),
                    Message::Palette,
                    Message::Draw,
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
                    Message::Hue,
                    Message::Draw,
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
                    Message::GridSep,
                    Message::Draw,
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
                    Message::Length,
                )
                .build(),
            )
            .push(
                SliderBuilder::new(
                    "Extrusion Size".to_string(),
                    Message::LengthSize,
                    Message::Draw,
                    self.controls.len_size,
                )
                .range(1.0..=350.0)
                .decimals(0)
                .build(),
            )
            .push(
                SliderBuilder::new(
                    "Stroke Width".to_string(),
                    Message::StrokeWidth,
                    Message::Draw,
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
                    Message::LengthDir,
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
                    Message::Grad,
                )
                .build(),
            )
            .padding(20)
            .spacing(15)
            .width(Length::Fixed(250.0));

        let rand_button = button("Random").on_press(Message::Randomize);
        let export_button = if self.controls.exporting {
            button("Export")
        } else {
            button("Export").on_press(Message::Export)
        };
        let image_panel = iced::widget::column!(
            vertical_space(Length::Fixed(25.0)),
            img_view,
            vertical_space(Length::Fixed(25.0)),
            row!(rand_button, export_button).spacing(100),
            text_input(
                "Export Width",
                &self.controls.export_width,
                Message::ExportWidth
            )
            .width(Length::Fixed(200.0)),
            text_input(
                "Export Height",
                &self.controls.export_height,
                Message::ExportHeight
            )
            .width(Length::Fixed(200.0)),
        )
        .spacing(20)
        .align_items(Alignment::Center);
        row![control_panel, image_panel,].into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
