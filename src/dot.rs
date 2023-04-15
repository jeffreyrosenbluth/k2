use crate::color::ColorMessage;
use crate::gui::{lpicklist::LPickList, lslider::LSlider};
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
    DotStrokeColor(ColorMessage),
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
    pub dirty: bool,
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
            dirty: false,
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
        dirty: bool,
    ) -> Self {
        Self {
            dot_style,
            size_controls,
            pearl_sides,
            pearl_smoothness,
            show_color_picker,
            dot_stroke_color,
            dirty,
        }
    }

    pub fn update(&mut self, message: DotMessage) {
        use self::DotMessage::*;
        match message {
            DotStyle(x) => {
                self.dot_style = Some(x);
                self.dirty = true
            }
            Size(x) => {
                self.size_controls.update(x);
                self.dirty = self.size_controls.dirty
            }
            PearlSides(x) => {
                self.pearl_sides = x;
                self.dirty = false
            }
            PearlSmoothness(x) => {
                self.pearl_smoothness = x;
                self.dirty = false
            }
            DotStrokeColor(x) => match x {
                ColorMessage::Choose => self.show_color_picker = true,
                ColorMessage::Submit(c) => {
                    self.dot_stroke_color = c;
                    self.show_color_picker = false;
                    self.dirty = true;
                }
                ColorMessage::Cancel => self.show_color_picker = false,
            },
            Null => self.dirty = true,
        }
    }

    pub fn view(&self) -> Element<'a, DotMessage> {
        use self::DotStyle::*;
        use DotMessage::*;
        let color_button = button(text("Dot Stroke Color").size(15))
            .on_press(DotStrokeColor(ColorMessage::Choose));
        let color_picker = ColorPicker::new(
            self.show_color_picker,
            self.dot_stroke_color,
            color_button,
            DotStrokeColor(ColorMessage::Cancel),
            |c| DotStrokeColor(ColorMessage::Submit(c)),
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
                    self.size_controls.dirty,
                )
                .view()
                .map(DotMessage::Size),
            )
            .spacing(15);
        if self.dot_style == Some(Pearl) {
            col = col
                .push(LSlider::new(
                    "Pearl Sides".to_string(),
                    self.pearl_sides,
                    3..=8,
                    1,
                    PearlSides,
                    Null,
                ))
                .push(LSlider::new(
                    "Pearl Smoothness".to_string(),
                    self.pearl_smoothness,
                    0..=5,
                    1,
                    PearlSmoothness,
                    Null,
                ))
        }
        col.into()
    }
}
