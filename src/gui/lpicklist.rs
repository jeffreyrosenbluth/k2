use iced::widget::{button, column, container, pick_list, row, scrollable, text};
use iced_lazy::{self, Component};
use iced_native::{overlay, Element};

pub struct LPickList<T, Message>
where
    T: Clone,
{
    label: String,
    choices: Vec<T>,
    value: Option<T>,
    text_size: u16,
    width: u16,
    spacing: u16,
    on_change: Box<dyn Fn(Option<T>) -> Message>,
    on_rand: Message,
}

#[derive(Debug, Clone)]
pub enum Event<T> {
    RandPressed,
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
        on_rand: Message,
    ) -> Self {
        Self {
            label,
            choices,
            value,
            text_size: 15,
            width: 175,
            spacing: 5,
            on_change: Box::new(on_change),
            on_rand,
        }
    }

    pub fn text_size(self, text_size: u16) -> Self {
        Self { text_size, ..self }
    }

    pub fn width(self, width: u16) -> Self {
        Self { width, ..self }
    }

    pub fn spacing(self, spacing: u16) -> Self {
        Self { spacing, ..self }
    }
}

impl<T, Message, Renderer> Component<Message, Renderer> for LPickList<T, Message>
where
    T: Clone + std::fmt::Display + Eq + 'static,
    Message: Clone,
    Renderer: iced_native::text::Renderer + 'static,
    <<Renderer as iced_native::Renderer>::Theme as iced::overlay::menu::StyleSheet>::Style: From<
        <<Renderer as iced_native::Renderer>::Theme as iced::widget::pick_list::StyleSheet>::Style,
    >,
    Renderer::Theme: button::StyleSheet
        + pick_list::StyleSheet
        + text::StyleSheet
        + scrollable::StyleSheet
        + container::StyleSheet
        + overlay::menu::StyleSheet,
{
    type State = ();
    type Event = Event<T>;

    fn update(&mut self, _state: &mut Self::State, event: Event<T>) -> Option<Message> {
        match event {
            Event::RandPressed => Some(self.on_rand.clone()),
            Event::PickListChanged(v) => Some((self.on_change)(Some(v))),
        }
    }

    fn view(&self, _state: &Self::State) -> iced_native::Element<'_, Self::Event, Renderer> {
        column![
            row![
                text(self.label.clone()).size(self.text_size),
                button(text("Rand").size(self.text_size * 5 / 8))
                    .on_press(Event::RandPressed)
                    .height(self.text_size * 5 / 4)
            ]
            .spacing(self.text_size),
            pick_list(
                self.choices.clone(),
                self.value.clone(),
                Event::PickListChanged
            )
            .text_size(self.text_size)
            .width(self.width),
        ]
        .spacing(self.spacing)
        .into()
    }
}
impl<'a, T, Message, Renderer> From<LPickList<T, Message>> for Element<'a, Message, Renderer>
where
    T: Clone + std::fmt::Display + Eq + 'static,
    Renderer: iced_native::text::Renderer + 'static,
    <<Renderer as iced_native::Renderer>::Theme as iced::overlay::menu::StyleSheet>::Style: From<
        <<Renderer as iced_native::Renderer>::Theme as iced::widget::pick_list::StyleSheet>::Style,
    >,
    Renderer::Theme: button::StyleSheet
        + pick_list::StyleSheet
        + text::StyleSheet
        + scrollable::StyleSheet
        + container::StyleSheet
        + overlay::menu::StyleSheet,
    Message: 'a + Clone,
{
    fn from(numeric_input: LPickList<T, Message>) -> Self {
        iced_lazy::component(numeric_input)
    }
}
