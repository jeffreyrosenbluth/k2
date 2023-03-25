use iced::widget::{button, row, slider, text};
use iced::{Alignment, Color};
use iced_lazy::{self, Component};
use iced_native::Element;
use std::ops::RangeInclusive;

pub struct LSlider<T, Message>
where
    T: Clone,
{
    label: String,
    value: T,
    range: RangeInclusive<T>,
    step: T,
    text_size: u16,
    width: u16,
    spacing: u16,
    decimals: u8,
    on_change: Box<dyn Fn(T) -> Message>,
    on_rand: Message,
    on_release: Message,
}

#[derive(Debug, Clone)]
pub enum Event<T> {
    RandPressed,
    SliderChanged(T),
    SliderReleased,
}

impl<T, Message> LSlider<T, Message>
where
    T: Clone,
{
    pub fn new(
        label: String,
        value: T,
        range: RangeInclusive<T>,
        step: T,
        on_change: impl Fn(T) -> Message + 'static,
        on_rand: Message,
        on_release: Message,
    ) -> Self {
        Self {
            label,
            value,
            range,
            step,
            text_size: 15,
            width: 150,
            spacing: 5,
            decimals: 1,
            on_change: Box::new(on_change),
            on_rand,
            on_release,
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

    pub fn decimals(self, decimals: u8) -> Self {
        Self { decimals, ..self }
    }
}

impl<'a, T, Message, Renderer> Component<Message, Renderer> for LSlider<T, Message>
where
    T: Copy + From<u8> + PartialOrd + num_traits::FromPrimitive + std::fmt::Display + 'static,
    f64: From<T>,
    Renderer: iced_native::text::Renderer + 'static,
    Renderer::Theme: button::StyleSheet + text::StyleSheet + slider::StyleSheet,
    <<Renderer as iced_native::Renderer>::Theme as iced::widget::text::StyleSheet>::Style:
        From<iced::Color>,
    Message: 'a + Clone,
{
    type State = ();
    type Event = Event<T>;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::RandPressed => Some(self.on_rand.clone()),
            Event::SliderChanged(v) => Some((self.on_change)(v)),
            Event::SliderReleased => Some(self.on_release.clone()),
        }
    }

    fn view(&self, _state: &Self::State) -> iced_native::Element<'_, Self::Event, Renderer> {
        let n = match self.decimals {
            0 => format!("{:7.0}", self.value),
            1 => format!("{:7.1}", self.value),
            _ => format!("{:7.2}", self.value),
        };

        iced::widget::column![
            row![
                text(self.label.clone()).size(self.text_size),
                button(text("Rand").size(self.text_size * 5 / 8))
                    .on_press(Event::RandPressed)
                    .height(self.text_size * 5 / 4)
            ]
            .spacing(self.text_size),
            row![
                slider(self.range.clone(), self.value, Event::SliderChanged)
                    .on_release(Event::SliderReleased)
                    .step(self.step)
                    .width(self.width),
                text(n)
                    .size(self.text_size)
                    .style(Color::from_rgb8(0x5E, 0x7C, 0xE2))
            ]
            .align_items(Alignment::Center)
        ]
        .spacing(self.spacing)
        .into()
    }
}

impl<'a, T, Message, Renderer> From<LSlider<T, Message>> for Element<'a, Message, Renderer>
where
    T: Copy + From<u8> + PartialOrd + num_traits::FromPrimitive + std::fmt::Display + 'static,
    f64: From<T>,
    Renderer: iced_native::text::Renderer + 'static,
    Renderer::Theme: button::StyleSheet + text::StyleSheet + slider::StyleSheet,
    <<Renderer as iced_native::Renderer>::Theme as iced::widget::text::StyleSheet>::Style:
        From<iced::Color>,
    Message: 'a + Clone,
{
    fn from(lslider: LSlider<T, Message>) -> Self {
        iced_lazy::component(lslider)
    }
}
