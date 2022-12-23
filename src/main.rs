use iced::{
    theme,
    widget::{button, column, image, pick_list, row, slider, text, toggler, Container},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};
use rand::prelude::*;

mod art;
mod common;
mod field;

use crate::art::*;
use crate::common::*;

const TEXT_SIZE: u16 = 15;

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (1100, 1000);
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
    CurlMessage(bool),
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
            Message::CurlMessage(b) => {
                self.controls.curl = b;
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
        let mut control_panel = column![
            Container::new(
                toggler(
                    "Spaced".to_owned(),
                    self.controls.spaced,
                    Message::SpaceMessage
                )
                .text_size(TEXT_SIZE)
            ),
            Container::new(
                toggler("Curl".to_owned(), self.controls.curl, Message::CurlMessage,)
                    .text_size(TEXT_SIZE)
            ),
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
            column![
                text(format!("Palette:  {:.0}", self.controls.palette_num)).size(TEXT_SIZE),
                slider(0..=9, self.controls.palette_num, Message::PaletteMessage)
                    .on_release(Message::DrawMessage),
            ]
            .spacing(5),
            column![
                text(format!("Hue:  {:.0}", self.controls.hue)).size(TEXT_SIZE),
                slider(0.0..=360.0, self.controls.hue, Message::HueMessage)
                    .on_release(Message::DrawMessage),
            ]
            .spacing(5),
            column![
                text(format!("Grid Spacing:  {:.0}", self.controls.grid_sep)).size(TEXT_SIZE),
                slider(
                    25.0..=100.0,
                    self.controls.grid_sep,
                    Message::GridSepMessage
                )
                .on_release(Message::DrawMessage),
            ]
            .spacing(5),
            column![
                text(format!("Octaves:  {:.0}", self.controls.octaves)).size(TEXT_SIZE),
                slider(1..=8, self.controls.octaves, Message::OctavesMessage)
                    .on_release(Message::DrawMessage),
            ]
            .spacing(5),
            column![
                text(format!("Noise Scale:  {:.1}", self.controls.noise_scale)).size(TEXT_SIZE),
                slider(0.5..=50.0, self.controls.noise_scale, Message::ScaleMessage)
                    .step(0.5)
                    .on_release(Message::DrawMessage),
            ]
            .spacing(5),
            column![
                text(format!("Noise Factor:  {:.1}", self.controls.noise_factor)).size(TEXT_SIZE),
                slider(
                    0.5..=25.0,
                    self.controls.noise_factor,
                    Message::FactorMessage,
                )
                .on_release(Message::DrawMessage)
                .step(0.5),
            ]
            .spacing(5),
            column![
                text("Xtrusion Length").size(TEXT_SIZE),
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
            column![
                text(format!("Length Size:  {:.0}", self.controls.len_size)).size(TEXT_SIZE),
                slider(
                    10.0..=500.0,
                    self.controls.len_size,
                    Message::LenSizeMessage,
                )
                .on_release(Message::DrawMessage)
                .step(1.0),
            ]
            .spacing(5),
            column![
                text(format!("Length Freq:  {:.0}", self.controls.len_freq)).size(TEXT_SIZE),
                slider(1.0..=25.0, self.controls.len_freq, Message::LenFreqMessage,)
                    .step(1.0)
                    .on_release(Message::DrawMessage)
            ]
            .spacing(5),
            column![
                text("Xtrusion Direction").size(TEXT_SIZE),
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

        let button_style = if self.controls.exporting {
            theme::Button::Secondary
        } else {
            theme::Button::Primary
        };

        control_panel = control_panel.push(
            button("Random")
                .width(Length::Units(75))
                .on_press(Message::RandMessage),
        );
        control_panel = control_panel.push(
            button("Export")
                .width(Length::Units(75))
                .on_press(Message::ExportMessage)
                .style(button_style),
        );
        row![
            control_panel,
            column![img_view].padding(20).align_items(Alignment::Center)
        ]
        .padding(20)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
