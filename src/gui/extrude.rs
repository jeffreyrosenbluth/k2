use crate::gradient::GradStyle;
use crate::gui::lpicklist::LPickList;
use crate::gui::lslider::LSlider;
use crate::length::{Dir, ExtrusionStyle};
use crate::Message::{self, *};
use crate::RandomMessage::*;
use iced::widget::Column;
use wassily::prelude::Constant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Extrude {
    pub style: Option<ExtrusionStyle>,
    pub size: f32,
    pub direction: Option<Dir>,
    pub grad_style: Option<GradStyle>,
}

impl<'a> Extrude {
    pub fn new(
        style: Option<ExtrusionStyle>,
        size: f32,
        direction: Option<Dir>,
        grad_style: Option<GradStyle>,
    ) -> Self {
        Self {
            style,
            size,
            direction,
            grad_style,
        }
    }

    pub fn show(&self) -> Column<'a, Message> {
        Column::new()
            .push(LPickList::new(
                "Extrusion Style".to_string(),
                vec![
                    ExtrusionStyle::Constant,
                    ExtrusionStyle::Expanding,
                    ExtrusionStyle::Contracting,
                    ExtrusionStyle::Varying,
                    ExtrusionStyle::Noisy,
                ],
                self.style,
                |x| x.map_or(Length(ExtrusionStyle::Constant), |v| Length(v)),
                Rand(RandomLenType),
            ))
            .push(
                LSlider::new(
                    "Extrusion Size".to_string(),
                    self.size,
                    5.0..=500.0,
                    5.0,
                    LengthSize,
                    Rand(RandomLenSize),
                    Draw,
                )
                .decimals(0),
            )
            .push(LPickList::new(
                "Extrusion Direction".to_string(),
                vec![Dir::Both, Dir::Horizontal, Dir::Vertical],
                self.direction,
                |x| x.map_or(LengthDir(Dir::Both), |v| LengthDir(v)),
                Rand(RandomLenDir),
            ))
            .push(LPickList::new(
                "Gradient Style".to_string(),
                vec![
                    GradStyle::None,
                    GradStyle::Light,
                    GradStyle::Dark,
                    GradStyle::Fiber,
                    GradStyle::LightFiber,
                    GradStyle::DarkFiber,
                ],
                self.grad_style,
                |x| x.map_or(Grad(GradStyle::None), |v| Grad(v)),
                Rand(RandomHighlight),
            ))
            .spacing(15)
    }
}
