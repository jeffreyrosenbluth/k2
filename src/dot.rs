use crate::color::ColorPickerMessage;
use crate::gui::{lpicklist::LPickList, numeric_input::NumericInput};
use crate::size::{SizeControls, SizeMessage};
use iced::widget::{button, row, text, Column};
use iced::Element;
use iced::{Alignment::Center, Color};
use iced_aw::ColorPicker;

#[derive(Debug, Clone)]
pub enum DotMessage {
    DotStyle(DotStyle),
    Size(SizeMessage),
    PearlSides(u32),
    PearlSmoothness(u32),
    DotStrokeColor(ColorPickerMessage),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DotStyle {
    Circle,
    Square,
    Pearl,
}

impl std::fmt::Display for DotStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DotStyle::Circle => "Circle",
                DotStyle::Square => "Square",
                DotStyle::Pearl => "Pearl",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DotControls {
    pub dot_style: Option<DotStyle>,
    pub size_controls: SizeControls,
    pub pearl_sides: u32,
    pub pearl_smoothness: u32,
    pub show_color_picker: bool,
    pub dot_stroke_color: Color,
}

impl Default for DotControls {
    fn default() -> Self {
        Self {
            dot_style: Some(DotStyle::Circle),
            size_controls: SizeControls::default(),
            pearl_sides: 4,
            pearl_smoothness: 3,
            show_color_picker: false,
            dot_stroke_color: Color::WHITE,
        }
    }
}

impl<'a> DotControls {
    pub fn new(
        dot_style: Option<DotStyle>,
        size_controls: SizeControls,
        pearl_sides: u32,
        pearl_smoothness: u32,
        show_color_picker: bool,
        dot_stroke_color: Color,
    ) -> Self {
        Self {
            dot_style,
            size_controls,
            pearl_sides,
            pearl_smoothness,
            show_color_picker,
            dot_stroke_color,
        }
    }

    pub fn update(&mut self, message: DotMessage) {
        use self::DotMessage::*;
        match message {
            DotStyle(x) => self.dot_style = Some(x),
            Size(x) => self.size_controls.update(x),
            PearlSides(x) => self.pearl_sides = x,
            PearlSmoothness(x) => self.pearl_smoothness = x,
            DotStrokeColor(x) => match x {
                ColorPickerMessage::Choose => self.show_color_picker = true,
                ColorPickerMessage::Submit(c) => {
                    self.dot_stroke_color = c;
                    self.show_color_picker = false;
                }
                ColorPickerMessage::Cancel => self.show_color_picker = false,
            },
            Null => (),
        }
    }

    pub fn view(&self) -> Element<'a, DotMessage> {
        use self::DotStyle::*;
        use DotMessage::*;
        let color_button = button(text("Dot Stroke Color").size(15))
            .on_press(DotStrokeColor(ColorPickerMessage::Choose));
        let color_picker = ColorPicker::new(
            self.show_color_picker,
            self.dot_stroke_color,
            color_button,
            DotStrokeColor(ColorPickerMessage::Cancel),
            |c| DotStrokeColor(ColorPickerMessage::Submit(c)),
        );
        let mut col = Column::new()
            .push(LPickList::new(
                "Dot Style".to_string(),
                vec![Circle, Square, Pearl],
                self.dot_style,
                |x| x.map_or(Null, DotStyle),
            ))
            .push(
                row![
                    color_picker,
                    text(format!(
                        "{:3} {:3} {:3}",
                        (self.dot_stroke_color.r * 255.0) as u8,
                        (self.dot_stroke_color.g * 255.0) as u8,
                        (self.dot_stroke_color.b * 255.0) as u8
                    ))
                    .size(15)
                ]
                .spacing(15)
                .align_items(Center),
            )
            .push(
                SizeControls::new(
                    self.size_controls.size_fn,
                    self.size_controls.size,
                    self.size_controls.direction,
                    self.size_controls.size_scale,
                    self.size_controls.min_size,
                )
                .view()
                .map(DotMessage::Size),
            )
            .spacing(15);
        if self.dot_style == Some(Pearl) {
            col = col
                .push(NumericInput::new(
                    "Pearl Sides".to_string(),
                    self.pearl_sides,
                    3..=8,
                    1,
                    0,
                    PearlSides,
                ))
                .push(NumericInput::new(
                    "Pearl Smoothness".to_string(),
                    self.pearl_smoothness,
                    0..=5,
                    1,
                    0,
                    PearlSmoothness,
                ))
        }
        col.into()
    }
}
