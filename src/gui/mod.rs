mod component;
mod device_listener;
mod style;
mod views;
mod widgets;

// let connect_option = ConnectOpts {
//     serial: None,//Some(String::from("/dev/tty.usbserial-110")),
//     speed: None
// };
// let mut config = Config::default();
// //config.connection.serial = Some(String::from("/dev/tty.usbserial-0001"));
// config.usb_device = vec![UsbDevice{
//     vid: 6790,
//     pid: 29987,
// }];
//
// let mut flasher = connect(&connect_option, &config).unwrap();
// let chip = flasher.chip();
// flasher.board_info();
// println!("{:?}", chip);
//}

use crate::gui::component::{primary_button, Component};
use crate::gui::device_listener::Event;
// use crate::gui::views::main_view::{DeviceInfoMessage, MainView, Message as MainViewMessage};
use crate::gui::widgets::device_connection_indicator::{
    DeviceConnectionIndicator, Message as ConnectionIndicatorMessage,
};
use std::arch::aarch64::vreinterpret_u8_f64;

use crate::core::flasher::flash;
use iced::{
    button, window::Settings as Window, Application, Column, Command, Element, Length, Settings,
    Text,
};
use iced_native::Subscription;
use miette::Result;
use parking_lot::lock_api::RwLockWriteGuard;
use parking_lot::{Mutex, RawRwLock, RwLock};
use usb_enumeration::UsbDevice;

// pub const ROBOTO_FONT: Font = Font::External {
//     name: "Roboto",
//     bytes: include_bytes!("../../../resources/fonts/Roboto-Regular.ttf"),
// };

pub struct SimpleFlasherApplication {
    // main_view: MainView,
    device: Mutex<Option<usb_enumeration::UsbDevice>>,

    update_button: button::State,
    device_connection_indicator: DeviceConnectionIndicator,
    is_device_connected: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdatePressed,
    UpdateRunning,
    UpdateFinished,
    // DeviceInfo(DeviceInfoMessage),
    ConnectIconAction(ConnectionIndicatorMessage),
    DeviceChangedAction(device_listener::Event),
}

impl Application for SimpleFlasherApplication {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                device: Mutex::new(None),
                update_button: Default::default(),
                device_connection_indicator: DeviceConnectionIndicator::new(),
                is_device_connected: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Simple Flasher".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::DeviceChangedAction(message) => match message {
                Event::Connect(device) => {
                    self.on_device_connection(device);
                    Command::none()
                    // Command::perform(async {}, move |_| {
                    //     Message::MainViewAction(MainViewMessage::DeviceInfo(
                    //         DeviceInfoMessage::Connected,
                    //     ))
                    // })
                }
                Event::Disconnect(device) => {
                    self.on_device_disconnection(device);
                    Command::none()
                    // Command::perform(async {}, move |_| {
                    //     Message::MainViewAction(MainViewMessage::DeviceInfo(
                    //         DeviceInfoMessage::Disconnected,
                    //     ))
                    // })
                }
            },
            Message::UpdatePressed => {
                //let device = self.device.lock().as_ref();
                if let Some(device) = self.device.lock().as_ref() {
                    flash(device).unwrap();
                }

                //flash(&*(self.device.lock()));
                Command::none()
            }
            Message::UpdateRunning => {
                todo!()
            }
            Message::UpdateFinished => {
                todo!()
            }
            // Message::DeviceInfo(_) => {
            //     todo!()
            // }
            Message::ConnectIconAction(message) => self
                .device_connection_indicator
                .update(message)
                .map(Message::ConnectIconAction),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        device_listener::listener().map(Message::DeviceChangedAction)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([24, 26])
            .push(Text::new("Device not connect").height(Length::Units(40)))
            .push(
                self.device_connection_indicator
                    .view()
                    .map(Message::ConnectIconAction),
            )
            .push(match self.is_device_connected {
                true => primary_button(&mut self.update_button, "Update")
                    .on_press(Message::UpdatePressed),
                false => primary_button(&mut self.update_button, "Update"),
            })
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
                ..iced::window::Settings::default()
            },
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
        self.is_device_connected = true;
    }

    fn on_device_disconnection(&mut self, device: UsbDevice) {
        self.remove_device(device);

        self.device_connection_indicator
            .update(ConnectionIndicatorMessage::Disconnected);
        self.is_device_connected = false;
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
}
