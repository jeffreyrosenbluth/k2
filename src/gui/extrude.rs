use crate::gradient::GradStyle;
use crate::gui::helpers::*;
use crate::length::{Dir, ExtrusionStyle};
use crate::Message::{self, *};
use crate::RandomMessage::*;
use iced::widget::Column;

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
            .push(
                PickListBuilder::new(
                    "Extrusion Style".to_string(),
                    vec![
                        ExtrusionStyle::Constant,
                        ExtrusionStyle::Expanding,
                        ExtrusionStyle::Contracting,
                        ExtrusionStyle::Varying,
                        ExtrusionStyle::Noisy,
                    ],
                    self.style,
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
                    self.size,
                )
                .range(5.0..=500.0)
                .decimals(0)
                .step(5.0)
                .build(),
            )
            .push(
                PickListBuilder::new(
                    "Extrusion Direction".to_string(),
                    vec![Dir::Both, Dir::Horizontal, Dir::Vertical],
                    self.direction,
                    LengthDir,
                    Rand(RandomLenDir),
                )
                .build(),
            )
            .push(
                PickListBuilder::new(
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
                    Grad,
                    Rand(RandomHighlight),
                )
                .build(),
            )
            .spacing(15)
    }
}
