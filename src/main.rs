use iced::{
    widget::{button, column, image, row, text_input, toggler, vertical_space, Container},
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
    settings.window.size = (900, 900);
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
    WorleyDistMessage(bool),
    ExportWidthMessage(String),
    ExportHeightMessage(String),
    NullMessage,
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
                self.draw();
            }
            Message::ExportMessage => {
                self.controls.exporting = true;
                return Command::perform(print(controls, 4.8), Message::ExportCompleteMessage);
            }

            Message::HueMessage(hue) => self.controls.hue = hue,
            Message::PaletteMessage(p) => self.controls.palette_num = p,
            Message::LocMessage(loc) => {
                self.controls.location = Some(loc);
                self.draw();
            }
            Message::GridSepMessage(s) => self.controls.grid_sep = s,
            Message::DrawMessage => {
                self.draw();
            }
            Message::OctavesMessage(o) => self.controls.octaves = o,
            Message::FactorMessage(f) => self.controls.noise_factor = f,
            Message::ScaleMessage(s) => self.controls.noise_scale = s,
            Message::NoiseMessage(n) => {
                self.controls.noise_function = Some(n);
                self.draw();
            }
            Message::LenMessage(l) => {
                self.controls.len_type = Some(l);
                self.draw();
            }
            Message::LenSizeMessage(s) => self.controls.len_size = s,
            Message::LenFreqMessage(f) => self.controls.len_freq = f,
            Message::LenDirMessage(d) => {
                self.controls.len_dir = Some(d);
                self.draw();
            }
            Message::CapMessage(c) => {
                self.controls.cap = Some(c);
                self.draw();
            }
            Message::RandMessage => {
                let mut rng = SmallRng::from_entropy();
                self.controls = rng.gen();
                self.draw();
            }
            Message::ExportCompleteMessage(_) => self.controls.exporting = false,
            Message::WorleyDistMessage(b) => {
                self.controls.worley_dist = b;
                self.draw();
            }
            Message::ExportWidthMessage(w) => self.controls.export_width = w,
            Message::ExportHeightMessage(h) => self.controls.export_height = h,
            Message::NullMessage => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let img_view = image::viewer(self.image.clone()).min_scale(1.0);
        let mut control_panel = column![];
        control_panel = control_panel
            .push(Container::new(
                toggler(
                    "Spaced".to_owned(),
                    self.controls.spaced,
                    Message::SpaceMessage,
                )
                .text_size(TEXT_SIZE),
            ))
            .push(wpick_list(
                "Noise Function".to_string(),
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
                Message::NoiseMessage,
            ));
        if self.controls.noise_function == Some(NoiseFunction::Worley) {
            control_panel = control_panel.push(Container::new(
                toggler(
                    "Distance Function".to_owned(),
                    self.controls.worley_dist,
                    Message::WorleyDistMessage,
                )
                .text_size(TEXT_SIZE),
            ))
        } else {
            control_panel = control_panel.push(wslider(
                "Octaves".to_string(),
                Message::OctavesMessage,
                Message::DrawMessage,
                1..=8,
                self.controls.octaves,
                1,
            ))
        };
        control_panel = control_panel
            .push(wslider(
                "Noise Scale".to_string(),
                Message::ScaleMessage,
                Message::DrawMessage,
                0.5..=25.0,
                self.controls.noise_scale,
                0.5,
            ))
            .push(wslider(
                "Noise Factor".to_string(),
                Message::FactorMessage,
                Message::DrawMessage,
                0.5..=25.0,
                self.controls.noise_factor,
                0.5,
            ))
            .push(wpick_list(
                "Location".to_string(),
                vec![
                    Location::Grid,
                    Location::Rand,
                    Location::Halton,
                    Location::Poisson,
                    Location::Circle,
                    Location::Trig,
                ],
                self.controls.location,
                Message::LocMessage,
            ))
            .push(wslider(
                "Palette".to_string(),
                Message::PaletteMessage,
                Message::DrawMessage,
                0..=9,
                self.controls.palette_num,
                1,
            ))
            .push(wslider(
                "Hue".to_string(),
                Message::HueMessage,
                Message::DrawMessage,
                0.0..=360.0,
                self.controls.hue,
                1.0,
            ))
            .push(wslider(
                "Grid Spacing".to_string(),
                Message::GridSepMessage,
                Message::DrawMessage,
                25.0..=100.0,
                self.controls.grid_sep,
                1.0,
            ))
            .push(wpick_list(
                "Extrusion Length".to_string(),
                vec![
                    Len::Constant,
                    Len::Expanding,
                    Len::Contracting,
                    Len::Varying,
                ],
                self.controls.len_type,
                Message::LenMessage,
            ))
            .push(wslider(
                "Extrusion Size".to_string(),
                Message::LenSizeMessage,
                Message::DrawMessage,
                75.0..=350.0,
                self.controls.len_size,
                1.0,
            ))
            .push(wslider(
                "Varying Freq".to_string(),
                Message::LenFreqMessage,
                Message::DrawMessage,
                1.0..=20.0,
                self.controls.len_freq,
                1.0,
            ))
            .push(wpick_list(
                "Extrusion Direction".to_string(),
                vec![Dir::Circle, Dir::Horizontal, Dir::Vertical],
                self.controls.len_dir,
                Message::LenDirMessage,
            ))
            .push(wpick_list(
                "Highlight".to_string(),
                vec![Cap::None, Cap::Light, Cap::Dark],
                self.controls.cap,
                Message::CapMessage,
            ))
            .padding(20)
            .spacing(15)
            .width(Length::Units(250));

        let rand_button = button("Random").on_press(Message::RandMessage);
        let export_button = if self.controls.exporting {
            button("Export")
        } else {
            button("Export").on_press(Message::ExportMessage)
        };
        let image_panel = column!(
            vertical_space(Length::Units(25)),
            img_view,
            vertical_space(Length::Units(25)),
            row!(rand_button, export_button).spacing(100),
            text_input(
                "Export Width",
                &self.controls.export_width,
                Message::ExportWidthMessage
            )
            .width(Length::Units(200)),
            text_input(
                "Export Height",
                &self.controls.export_height,
                Message::ExportHeightMessage
            )
            .width(Length::Units(200)),
        )
        .spacing(20)
        .align_items(Alignment::Center);
        row![control_panel, image_panel,].into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
