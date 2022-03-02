use crate::gui::component::Component;
use iced::svg::Handle;
use iced::{Command, Container, Element, Font, Length, Svg};
use std::io::ErrorKind::InvalidData;

static CONNEECT_ICON_DATA: &'static [u8] =
    include_bytes!("../../../resources/icons/connected_icon.svg");
static DISCONNECT_ICON_DATA: &'static [u8] =
    include_bytes!("../../../resources/icons/disconnected_icon.svg");
static ERROR_ICON_DATA: &'static [u8] =
    include_bytes!("../../../resources/icons/connect_error_icon.svg");

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Connected,
    Disconnected,
    Error,
}

pub struct DeviceConnectionIndicator {
    connect_svg: Svg,
    disconnect_svg: Svg,
    error_svg: Svg,
    pub message: Message,
}

impl Component for DeviceConnectionIndicator {
    type Message = Message;

    fn new() -> Self {
        Self {
            connect_svg: Svg::new(Handle::from_memory(CONNEECT_ICON_DATA))
                .width(Length::Fill)
                .height(Length::Fill),
            disconnect_svg: Svg::new(Handle::from_memory(DISCONNECT_ICON_DATA))
                .width(Length::Fill)
                .height(Length::Fill),
            error_svg: Svg::new(Handle::from_memory(ERROR_ICON_DATA))
                .width(Length::Fill)
                .height(Length::Fill),
            message: Message::Connected,
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.message = message;
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        println!("view device connection");

        let svg = match self.message {
            Message::Connected => &self.connect_svg,
            Message::Disconnected => &self.disconnect_svg,
            Message::Error => &self.error_svg,
        };

        Container::new(svg.clone())
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(50)
            .center_x()
            .center_y()
            .into()
    }
}
