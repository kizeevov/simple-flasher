mod component;
mod device_listener;
mod localize;
mod style;
mod widgets;

#[allow(unused)]
use crate::gui::component::secondary_button;

use crate::gui::component::{application_icon, message_text, primary_button, Component};
use crate::gui::device_listener::Event;
use crate::gui::widgets::device_connection_indicator::{
    DeviceConnectionIndicator, Message as ConnectionIndicatorMessage,
};

use crate::core::flasher::{flash_device, FlashError};

use crate::core::drivers::install_drivers;
use crate::fl;
use iced::{
    button, window::Settings as Window, Application, Column, Command, Element, Length, Settings,
};
use iced_native::Subscription;
use parking_lot::Mutex;
use usb_enumeration::UsbDevice;

pub struct SimpleFlasherApplication {
    current_message: Message,
    device: Mutex<Option<usb_enumeration::UsbDevice>>,
    update_button: button::State,
    #[cfg(target_os = "windows")]
    driver_install_button: button::State,
    device_connection_indicator: DeviceConnectionIndicator,
    message: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    UpdateRunning,
    UpdateFinished(Result<(), FlashError>),
    ConnectIconAction(ConnectionIndicatorMessage),
    DeviceChangedAction(device_listener::Event),
    #[cfg(target_os = "windows")]
    DriverInstallingStart,
    #[cfg(target_os = "windows")]
    DriverInstalling(Result<(), ()>),
}

impl Application for SimpleFlasherApplication {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                current_message: Message::None,
                device: Mutex::new(None),
                update_button: Default::default(),
                #[cfg(target_os = "windows")]
                driver_install_button: Default::default(),
                device_connection_indicator: DeviceConnectionIndicator::new(),
                message: fl!("device-disabled-please-connect"),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Simple Flasher".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.current_message = message.clone();
        match message {
            Message::None => unimplemented!(),
            Message::DeviceChangedAction(message) => match message {
                Event::Connect(device) => {
                    self.on_device_connection(device);
                    Command::none()
                }
                Event::Disconnect(device) => {
                    self.on_device_disconnection(device);
                    Command::none()
                }
            },
            Message::UpdateRunning => match self.device.lock().as_ref() {
                None => Command::none(),
                Some(device) => {
                    self.message = fl!("update-please-wait");
                    Command::perform(flash_device(device.clone()), Message::UpdateFinished)
                }
            },
            Message::UpdateFinished(Ok(_)) => {
                self.message = fl!("update-success");
                Command::none()
            }
            Message::UpdateFinished(Err(err)) => {
                self.message = match err {
                    FlashError::ConnectError => fl!("device-connection-error"),
                    FlashError::FileError => fl!("file-not-found-or-corrupted"),
                    FlashError::SpawnError => fl!("failed-start-task"),
                    FlashError::BoardInfoError => fl!("device-info-error"),
                    FlashError::FlashError => fl!("device-update-error"),
                };

                Command::none()
            }
            Message::ConnectIconAction(message) => self
                .device_connection_indicator
                .update(message)
                .map(Message::ConnectIconAction),
            #[cfg(target_os = "windows")]
            Message::DriverInstallingStart => {
                Command::perform(async { install_drivers() }, Message::DriverInstalling)
            }
            #[cfg(target_os = "windows")]
            Message::DriverInstalling(Ok(_)) => {
                self.message = fl!("driver-install-success");
                Command::none()
            }
            #[cfg(target_os = "windows")]
            Message::DriverInstalling(Err(_)) => {
                self.message = fl!("driver-install-error");
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        device_listener::listener().map(Message::DeviceChangedAction)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let is_update_button_enabled = self.is_update_button_enabled();

        Column::new()
            .spacing(6)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([24, 26])
            .push(
                self.device_connection_indicator
                    .view()
                    .map(Message::ConnectIconAction),
            )
            .push(message_text(&self.message))
            .push({
                #[cfg(target_os = "windows")]
                {
                    secondary_button(
                        &mut self.driver_install_button,
                        &fl!("driver-install"),
                        Message::DriverInstallingStart,
                        true,
                    )
                }

                #[cfg(not(any(windows)))]
                {
                    Column::new()
                }
            })
            .push(primary_button(
                &mut self.update_button,
                &fl!("update"),
                Message::UpdateRunning,
                is_update_button_enabled,
            ))
            .into()
    }
}

impl SimpleFlasherApplication {
    pub fn start() -> iced::Result {
        let settings: Settings<()> = Settings {
            window: Window {
                size: (400, 560),
                resizable: false,
                decorations: true,
                icon: Some(application_icon()),
                ..iced::window::Settings::default()
            },
            default_font: Some(include_bytes!("../../resources/fonts/Roboto-Regular.ttf")),
            default_text_size: 17,
            antialiasing: true,
            ..iced::Settings::default()
        };

        Self::run(settings)
    }

    fn on_device_connection(&mut self, device: UsbDevice) {
        *(self.device.lock()) = Some(device);

        self.device_connection_indicator
            .update(ConnectionIndicatorMessage::Connected);

        self.message = fl!("device-connected");
    }

    fn on_device_disconnection(&mut self, device: UsbDevice) {
        self.remove_device(device);

        self.device_connection_indicator
            .update(ConnectionIndicatorMessage::Disconnected);

        self.message = fl!("device-disabled-please-connect");
    }

    fn remove_device(&mut self, device: UsbDevice) {
        let mut guard = self.device.lock();

        let device_option = guard.as_ref();
        let device_old = match device_option {
            None => return,
            Some(device) => device,
        };

        if device.id == device_old.id {
            *guard = None;
        }
    }

    fn is_update_button_enabled(&mut self) -> bool {
        self.device.lock().is_some()
            && matches!(
                self.current_message,
                Message::DeviceChangedAction(_)
                    | Message::UpdateFinished(_)
                    | Message::DriverInstalling(_)
                    | Message::ConnectIconAction(_)
            )
    }
}
