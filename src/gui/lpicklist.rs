#![allow(deprecated)]

use iced::widget::{column, component, pick_list, text, Component};
use iced::Element;

pub struct LPickList<T, Message>
where
    T: Clone,
{
    label: String,
    choices: Vec<T>,
    value: Option<T>,
    text_size: f32,
    width: f32,
    spacing: f32,
    on_change: Box<dyn Fn(Option<T>) -> Message>,
}

#[derive(Debug, Clone)]
pub enum Event<T> {
    PickListChanged(T),
}

impl<T, Message> LPickList<T, Message>
where
    T: Clone,
{
    pub fn new(
        label: String,
        choices: Vec<T>,
        value: Option<T>,
        on_change: impl Fn(Option<T>) -> Message + 'static,
    ) -> Self {
        Self {
            label,
            choices,
            value,
            text_size: 15.0,
            width: 175.0,
            spacing: 10.0,
            on_change: Box::new(on_change),
        }
    }

    pub fn text_size(self, text_size: f32) -> Self {
        Self { text_size, ..self }
    }

    pub fn width(self, width: f32) -> Self {
        Self { width, ..self }
    }

    pub fn spacing(self, spacing: f32) -> Self {
        Self { spacing, ..self }
    }
}

impl<T, Message> Component<Message> for LPickList<T, Message>
where
    T: Clone + std::fmt::Display + PartialEq + 'static,
    Message: Clone,
{
    type State = ();
    type Event = Event<T>;

    fn update(&mut self, _state: &mut Self::State, event: Event<T>) -> Option<Message> {
        match event {
            Event::PickListChanged(v) => Some((self.on_change)(Some(v))),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        column![
            text(self.label.clone()).size(self.text_size),
            pick_list(
                self.choices.clone(),
                self.value.clone(),
                Event::PickListChanged
            )
            .text_size(self.text_size)
            .width(self.width)
            .placeholder("None"),
        ]
        .spacing(self.spacing)
        .into()
    }
}

impl<'a, T, Message> From<LPickList<T, Message>> for Element<'a, Message>
where
    T: Clone + std::fmt::Display + PartialEq + 'static,
    Message: Clone + 'a,
{
    fn from(picklist: LPickList<T, Message>) -> Self {
        component(picklist)
    }
}
