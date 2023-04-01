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
    pub size_scale: f32,
    pub min_size: f32,
}

impl<'a> Dot {
    pub fn new(style: Option<SizeFn>, size: f32, direction: Option<Dir>, size_scale: f32, min_size: f32) -> Self {
        Self {
            style,
            size,
            direction,
            size_scale,
            min_size
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
                    SizeFn::Periodic,
                ],
                self.style,
                |x| x.map_or(Length(SizeFn::Constant), Length),
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
                |x| x.map_or(LengthDir(Dir::Both), LengthDir),
                Rand(RandomLenDir),
            ));
        } else if self.style == Some(SizeFn::Periodic) {
            col = col.push(LSlider::new(
                "Size Scale".to_string(),
                self.size_scale,
                1.0..=30.0,
                1.0,
                SizeScale,
                Some(Rand(RandomSizeScale)),
                Draw,
            )).push(
                LSlider::new(
                    "Min Size".to_string(),
                    self.min_size,
                    1.0..=50.0,
                    1.0,
                    MinSize,
                    Some(Rand(RandomMinSize)),
                    Draw,
                )
            )
        }
        col
    }
}
