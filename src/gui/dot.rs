use crate::common::DotStyle;
use crate::gui::lpicklist::LPickList;
use crate::gui::lslider::LSlider;
use crate::size::{Dir, SizeFn};
use crate::Message::{self, *};
use iced::widget::Column;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dot {
    pub dot_style: Option<DotStyle>,
    pub size_fn: Option<SizeFn>,
    pub size: f32,
    pub direction: Option<Dir>,
    pub size_scale: f32,
    pub min_size: f32,
    pub pearl_sides: u32,
    pub pearl_smoothness: u32,
}

impl<'a> Dot {
    pub fn new(
        dot_style: Option<DotStyle>,
        size_fn: Option<SizeFn>,
        size: f32,
        direction: Option<Dir>,
        size_scale: f32,
        min_size: f32,
        pearl_sides: u32,
        pearl_smoothness: u32,
    ) -> Self {
        Self {
            dot_style,
            size_fn,
            size,
            direction,
            size_scale,
            min_size,
            pearl_sides,
            pearl_smoothness,
        }
    }

    pub fn show(&self) -> Column<'a, Message> {
        let mut col = Column::new()
            .push(LPickList::new(
                "Dot Style".to_string(),
                vec![DotStyle::Circle, DotStyle::Square, DotStyle::Pearl],
                self.dot_style,
                |x| x.map_or(Dot(DotStyle::Circle), Dot),
            ))
            .push(LPickList::new(
                "Size Function".to_string(),
                vec![
                    SizeFn::Constant,
                    SizeFn::Expanding,
                    SizeFn::Contracting,
                    SizeFn::Periodic,
                ],
                self.size_fn,
                |x| x.map_or(Length(SizeFn::Constant), Length),
            ))
            .push(
                LSlider::new(
                    "Size".to_string(),
                    self.size,
                    5.0..=500.0,
                    5.0,
                    LengthSize,
                    Draw,
                )
                .decimals(0),
            )
            .spacing(15);
        if self.size_fn == Some(SizeFn::Expanding) || self.size_fn == Some(SizeFn::Contracting) {
            col = col.push(LPickList::new(
                "Direction".to_string(),
                vec![Dir::Both, Dir::Horizontal, Dir::Vertical],
                self.direction,
                |x| x.map_or(LengthDir(Dir::Both), LengthDir),
            ));
        } else if self.size_fn == Some(SizeFn::Periodic) {
            col = col
                .push(LSlider::new(
                    "Size Scale".to_string(),
                    self.size_scale,
                    1.0..=30.0,
                    1.0,
                    SizeScale,
                    Draw,
                ))
                .push(LSlider::new(
                    "Min Size".to_string(),
                    self.min_size,
                    1.0..=50.0,
                    1.0,
                    MinSize,
                    Draw,
                ))
        }
        if self.dot_style == Some(DotStyle::Pearl) {
            col = col
                .push(LSlider::new(
                    "Pearl Sides".to_string(),
                    self.pearl_sides,
                    3..=8,
                    1,
                    PearlSides,
                    Draw,
                ))
                .push(LSlider::new(
                    "Pearl Smoothness".to_string(),
                    self.pearl_smoothness,
                    0..=5,
                    1,
                    PearlSmoothness,
                    Draw,
                ))
        }
        col
    }
}
