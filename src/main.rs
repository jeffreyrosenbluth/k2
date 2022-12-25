use iced::{
    widget::{button, column, image, pick_list, row, text, toggler, vertical_space, Container},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};
use rand::prelude::*;

mod art;
mod common;
mod field;
mod gui;

use crate::art::*;
use crate::common::*;
use crate::gui::*;

const TEXT_SIZE: u16 = 15;

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (1080, 900);
    Xtrusion::run(settings)
}

#[derive(Debug, Clone)]
pub enum Message {
    SpaceMessage(bool),
    PaletteMessage(u8),
    HueMessage(f32),
    ExportMessage,
    DrawMessage,
    LocMessage(Location),
    GridSepMessage(f32),
    OctavesMessage(u8),
    FactorMessage(f32),
    ScaleMessage(f32),
    NoiseMessage(NoiseFunction),
    LenMessage(Len),
    LenSizeMessage(f32),
    LenFreqMessage(f32),
    LenDirMessage(Dir),
    CapMessage(Cap),
    RandMessage,
    ExportCompleteMessage(()),
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
            Message::SpaceMessage(b) => {
                self.controls.spaced = b;
                let canvas = draw(&self.controls, 1.0);
                self.image =
                    image::Handle::from_pixels(canvas.width, canvas.height, canvas.pixmap.take());
            }
            Message::ExportMessage => {
                self.controls.exporting = true;
                return Command::perform(print(controls, 4.8), Message::ExportCompleteMessage);
            }

            Message::HueMessage(hue) => self.controls.hue = hue,
            Message::PaletteMessage(p) => self.controls.palette_num = p,
            Message::LocMessage(loc) => {
                self.controls.location = Some(loc);
                let canvas = draw(&self.controls, 1.0);
                self.image =
                    image::Handle::from_pixels(canvas.width, canvas.height, canvas.pixmap.take());
            }
            Message::GridSepMessage(s) => self.controls.grid_sep = s,
            Message::DrawMessage => {
                let canvas = draw(&self.controls, 1.0);
                self.image =
                    image::Handle::from_pixels(canvas.width, canvas.height, canvas.pixmap.take());
            }
            Message::OctavesMessage(o) => self.controls.octaves = o,
            Message::FactorMessage(f) => self.controls.noise_factor = f,
            Message::ScaleMessage(s) => self.controls.noise_scale = s,
            Message::NoiseMessage(n) => {
                self.controls.noise_function = Some(n);
                let canvas = draw(&self.controls, 1.0);
                self.image =
                    image::Handle::from_pixels(canvas.width, canvas.height, canvas.pixmap.take());
            }
            Message::LenMessage(l) => {
                self.controls.len_type = Some(l);
                let canvas = draw(&self.controls, 1.0);
                self.image =
                    image::Handle::from_pixels(canvas.width, canvas.height, canvas.pixmap.take());
            }
            Message::LenSizeMessage(s) => self.controls.len_size = s,
            Message::LenFreqMessage(f) => self.controls.len_freq = f,
            Message::LenDirMessage(d) => {
                self.controls.len_dir = Some(d);
                let canvas = draw(&self.controls, 1.0);
                self.image =
                    image::Handle::from_pixels(canvas.width, canvas.height, canvas.pixmap.take());
            }
            Message::CapMessage(c) => {
                self.controls.cap = Some(c);
                let canvas = draw(&self.controls, 1.0);
                self.image =
                    image::Handle::from_pixels(canvas.width, canvas.height, canvas.pixmap.take());
            }
            Message::RandMessage => {
                let mut rng = SmallRng::from_entropy();
                self.controls = rng.gen();
                // self.controls.dirty = false;
                let canvas = draw(&self.controls, 1.0);
                self.image =
                    image::Handle::from_pixels(canvas.width, canvas.height, canvas.pixmap.take());
            }
            Message::ExportCompleteMessage(_) => self.controls.exporting = false,
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let img_view = image::viewer(self.image.clone()).min_scale(1.0);
        let control_panel = column![
            Container::new(
                toggler(
                    "Spaced".to_owned(),
                    self.controls.spaced,
                    Message::SpaceMessage
                )
                .text_size(TEXT_SIZE)
            ),
            column![
                text("Noise Function").size(TEXT_SIZE),
                pick_list(
                    vec![
                        NoiseFunction::Fbm,
                        NoiseFunction::Billow,
                        NoiseFunction::Ridged,
                        NoiseFunction::Value,
                        NoiseFunction::Checkerboard,
                        NoiseFunction::Cylinders,
                        NoiseFunction::Worley,
                    ],
                    self.controls.noise_function,
                    Message::NoiseMessage
                )
                .text_size(TEXT_SIZE),
            ]
            .spacing(5),
            column![
                text("Location").size(TEXT_SIZE),
                pick_list(
                    vec![
                        Location::Grid,
                        Location::Rand,
                        Location::Halton,
                        Location::Poisson,
                        Location::Circle,
                        Location::Trig,
                    ],
                    self.controls.location,
                    Message::LocMessage
                )
                .text_size(TEXT_SIZE),
            ]
            .spacing(5),
            wslider(
                "Palette".to_string(),
                Message::PaletteMessage,
                Message::DrawMessage,
                0..=9,
                self.controls.palette_num,
                1
            ),
            wslider(
                "Hue".to_string(),
                Message::HueMessage,
                Message::DrawMessage,
                0.0..=360.0,
                self.controls.hue,
                1.0
            ),
            wslider(
                "Grid Spacing".to_string(),
                Message::GridSepMessage,
                Message::DrawMessage,
                25.0..=100.0,
                self.controls.grid_sep,
                1.0
            ),
            wslider(
                "Octaves".to_string(),
                Message::OctavesMessage,
                Message::DrawMessage,
                1..=8,
                self.controls.octaves,
                1
            ),
            wslider(
                "Noise Scale".to_string(),
                Message::ScaleMessage,
                Message::DrawMessage,
                0.5..=25.0,
                self.controls.noise_scale,
                0.5
            ),
            wslider(
                "Noise Factor".to_string(),
                Message::FactorMessage,
                Message::DrawMessage,
                0.5..=25.0,
                self.controls.noise_factor,
                0.5
            ),
            column![
                text("Extrusion Length").size(TEXT_SIZE),
                pick_list(
                    vec![
                        Len::Constant,
                        Len::Expanding,
                        Len::Contracting,
                        Len::Varying,
                    ],
                    self.controls.len_type,
                    Message::LenMessage
                )
                .text_size(TEXT_SIZE),
            ]
            .spacing(5),
            wslider(
                "Extrusion Size".to_string(),
                Message::LenSizeMessage,
                Message::DrawMessage,
                75.0..=350.0,
                self.controls.len_size,
                1.0
            ),
            wslider(
                "Varying Freq".to_string(),
                Message::LenFreqMessage,
                Message::DrawMessage,
                1.0..=20.0,
                self.controls.len_freq,
                1.0
            ),
            column![
                text("Extrusion Direction").size(TEXT_SIZE),
                pick_list(
                    vec![Dir::Circle, Dir::Horizontal, Dir::Vertical,],
                    self.controls.len_dir,
                    Message::LenDirMessage
                )
                .text_size(TEXT_SIZE),
            ]
            .spacing(5),
            column![
                text("Highlight").size(TEXT_SIZE),
                pick_list(
                    vec![Cap::None, Cap::Light, Cap::Dark],
                    self.controls.cap,
                    Message::CapMessage
                )
                .text_size(TEXT_SIZE),
            ]
            .spacing(5),
        ]
        .padding(20)
        .spacing(15)
        .width(Length::Units(200));

        let rand_button = button("Random").on_press(Message::RandMessage);
        let export_button = if self.controls.exporting {
            button("Export")
        } else {
            button("Export").on_press(Message::ExportMessage)
        };
        let image_panel = column!(
            img_view,
            vertical_space(Length::Units(25)),
            row!(rand_button, export_button).spacing(100)
        )
        .padding(20)
        .align_items(Alignment::Center);
        row![control_panel, image_panel,].padding(20).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
