use crate::gui::lpicklist::LPickList;
use crate::gui::lslider::LSlider;
use crate::size::{Dir, SizeFn};
use crate::Message::{self, *};
use crate::RandomMessage::*;
use iced::widget::Column;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dot {
    pub style: Option<SizeFn>,
    pub size: f32,
    pub direction: Option<Dir>,
}

impl<'a> Dot {
    pub fn new(style: Option<SizeFn>, size: f32, direction: Option<Dir>) -> Self {
        Self {
            style,
            size,
            direction,
        }
    }

    pub fn show(&self) -> Column<'a, Message> {
        let mut col = Column::new()
            .push(LPickList::new(
                "Size Function".to_string(),
                vec![
                    SizeFn::Constant,
                    SizeFn::Expanding,
                    SizeFn::Contracting,
                    SizeFn::Varying,
                    SizeFn::Noisy,
                ],
                self.style,
                |x| x.map_or(Length(SizeFn::Constant), |v| Length(v)),
                Rand(RandomLenType),
            ))
            .push(
                LSlider::new(
                    "Size".to_string(),
                    self.size,
                    5.0..=1000.0,
                    5.0,
                    LengthSize,
                    Some(Rand(RandomLenSize)),
                    Draw,
                )
                .decimals(0),
            )
            .spacing(15);
        if self.style == Some(SizeFn::Expanding) || self.style == Some(SizeFn::Contracting) {
            col = col.push(LPickList::new(
                "Direction".to_string(),
                vec![Dir::Both, Dir::Horizontal, Dir::Vertical],
                self.direction,
                |x| x.map_or(LengthDir(Dir::Both), |v| LengthDir(v)),
                Rand(RandomLenDir),
            ));
        }
        col
    }
}
