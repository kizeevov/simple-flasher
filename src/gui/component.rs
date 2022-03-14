use crate::gui::style::{PrimaryButtonStyle, SecondaryButtonStyle};
use iced::alignment::{Horizontal, Vertical};
use iced::window::Icon;
use iced::{button, Button, Command, Container, Element, Length, Text};

pub trait Component {
    type Message;

    fn new() -> Self;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message>;

    fn view(&mut self) -> Element<Self::Message>;
}

pub fn primary_button<'a, Message: Clone>(
    state: &'a mut button::State,
    label: &str,
    message: Message,
    enabled: bool,
) -> Button<'a, Message> {
    let button = Button::new(
        state,
        Text::new(label)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .size(20),
    )
    .width(Length::Fill)
    .height(Length::Units(40))
    .style(PrimaryButtonStyle);

    match enabled {
        true => button.on_press(message),
        false => button,
    }
}

pub fn secondary_button<'a, Message: Clone>(
    state: &'a mut button::State,
    label: &str,
    message: Message,
    enabled: bool,
) -> Button<'a, Message> {
    let button = Button::new(
        state,
        Text::new(label)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .size(17),
    )
    .width(Length::Fill)
    .height(Length::Units(40))
    .style(SecondaryButtonStyle);

    match enabled {
        true => button.on_press(message),
        false => button,
    }
}

pub fn message_text<Message>(label: &str) -> Container<Message> {
    Container::new(
        Text::new(label)
            .height(Length::Units(60))
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .size(18),
    )
    .width(Length::Fill)
}

pub fn application_icon() -> Icon {
    let icon_bytes = include_bytes!("../../resources/icons/icon.png");
    let icon = image::load_from_memory(icon_bytes).unwrap().to_rgba8();
    Icon::from_rgba(icon.to_vec(), icon.width(), icon.height()).unwrap()
}
