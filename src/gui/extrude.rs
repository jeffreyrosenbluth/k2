use crate::gradient::GradStyle;
use crate::gui::lpicklist::LPickList;
use crate::gui::lslider::LSlider;
use crate::size::{Dir, SizeFn};
use crate::Message::{self, *};
use iced::widget::Column;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Extrude {
    pub style: Option<SizeFn>,
    pub size: f32,
    pub direction: Option<Dir>,
    pub size_scale: f32,
    pub min_size: f32,
    pub grad_style: Option<GradStyle>,
}

impl<'a> Extrude {
    pub fn new(
        style: Option<SizeFn>,
        size: f32,
        direction: Option<Dir>,
        size_scale: f32,
        min_size: f32,
        grad_style: Option<GradStyle>,
    ) -> Self {
        Self {
            style,
            size,
            direction,
            size_scale,
            min_size,
            grad_style,
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
            );
        if self.style == Some(SizeFn::Expanding) || self.style == Some(SizeFn::Contracting) {
            col = col
                .push(LPickList::new(
                    "Direction".to_string(),
                    vec![Dir::Both, Dir::Horizontal, Dir::Vertical],
                    self.direction,
                    |x| x.map_or(LengthDir(Dir::Both), LengthDir),
                ))
                .push(LSlider::new(
                    "Min Size".to_string(),
                    self.min_size,
                    1.0..=50.0,
                    1.0,
                    MinSize,
                    Draw,
                ))
        } else if self.style == Some(SizeFn::Periodic) {
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
        col = col
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
                |x| x.map_or(Grad(GradStyle::None), Grad),
            ))
            .spacing(15);
        col
    }
}
