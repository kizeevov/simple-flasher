use crate::gui::style::PrimaryButtonStyle;
use iced::alignment::{Horizontal, Vertical};
use iced::{button, Button, Command, Element, Length, Text};

pub trait Component {
    type Message;

    fn new() -> Self;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message>;

    fn view(&mut self) -> Element<Self::Message>;
}

pub fn primary_button<'a, Message: Clone>(
    state: &'a mut button::State,
    label: &str,
) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .size(20),
    )
    .width(Length::Fill)
    .height(Length::Units(40))
    .style(PrimaryButtonStyle)
}
