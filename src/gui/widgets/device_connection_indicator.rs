use crate::gui::component::Component;
use iced::svg::Handle;
use iced::{Command, Container, Element, Length, Svg};

static CONNECT_ICON_DATA: &[u8] = include_bytes!("../../../resources/icons/connected_icon.svg");
static DISCONNECT_ICON_DATA: &[u8] =
    include_bytes!("../../../resources/icons/disconnected_icon.svg");

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Connected,
    Disconnected,
}

pub struct DeviceConnectionIndicator {
    connect_svg: Svg,
    disconnect_svg: Svg,
    pub message: Message,
}

impl Component for DeviceConnectionIndicator {
    type Message = Message;

    fn new() -> Self {
        Self {
            connect_svg: Svg::new(Handle::from_memory(CONNECT_ICON_DATA))
                .width(Length::Fill)
                .height(Length::Fill),
            disconnect_svg: Svg::new(Handle::from_memory(DISCONNECT_ICON_DATA))
                .width(Length::Fill)
                .height(Length::Fill),
            message: Message::Disconnected,
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.message = message;
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let svg = match self.message {
            Message::Connected => &self.connect_svg,
            Message::Disconnected => &self.disconnect_svg,
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
