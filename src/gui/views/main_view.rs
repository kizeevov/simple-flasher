use crate::gui::component::{primary_button, Component};
use crate::gui::widgets::device_connection_indicator::{
    DeviceConnectionIndicator, Message as DeviceConnectionIndicatorMessage,
};
use iced::{button, Column, Command, Element, Length};

pub struct MainView {
    update_button: button::State,
    device_connection_indicator: DeviceConnectionIndicator,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    UpdatePressed,
    ConnectIconAction(DeviceConnectionIndicatorMessage),
}

impl Component for MainView {
    type Message = Message;

    fn new() -> Self {
        Self {
            update_button: Default::default(),
            device_connection_indicator: DeviceConnectionIndicator::new(),
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::UpdatePressed => {
                let message = match self.device_connection_indicator.message {
                    DeviceConnectionIndicatorMessage::Connected => {
                        DeviceConnectionIndicatorMessage::Disconnected
                    }
                    DeviceConnectionIndicatorMessage::Disconnected => {
                        DeviceConnectionIndicatorMessage::Error
                    }
                    DeviceConnectionIndicatorMessage::Error => {
                        DeviceConnectionIndicatorMessage::Connected
                    }
                };
                Command::perform(async {}, move |_| Message::ConnectIconAction(message))
            }
            Message::ConnectIconAction(message) => self
                .device_connection_indicator
                .update(message)
                .map(Message::ConnectIconAction),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([24, 26])
            .push(
                self.device_connection_indicator
                    .view()
                    .map(Message::ConnectIconAction),
            )
            .push(
                primary_button(&mut self.update_button, "Update").on_press(Message::UpdatePressed),
            )
            .into()
    }
}
