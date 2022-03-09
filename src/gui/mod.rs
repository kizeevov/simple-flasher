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

use crate::gui::component::Component;
use crate::gui::views::main_view::{MainView, Message as MainViewMessage};

use iced::{window::Settings as Window, Application, Column, Command, Element, Length, Settings};
use iced_native::Subscription;

// pub const ROBOTO_FONT: Font = Font::External {
//     name: "Roboto",
//     bytes: include_bytes!("../../../resources/fonts/Roboto-Regular.ttf"),
// };

pub struct SimpleFlasherApplication {
    main_view: MainView,
}

#[derive(Debug, Clone)]
pub enum Message {
    MainViewAction(MainViewMessage),
    DeviceChangedAction(device_listener::Event),
}

impl Application for SimpleFlasherApplication {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                main_view: MainView::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Simple Flasher".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::MainViewAction(message) => {
                self.main_view.update(message).map(Message::MainViewAction)
            }
            Message::DeviceChangedAction(_) => Command::none(),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        device_listener::listener().map(Message::DeviceChangedAction)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(self.main_view.view().map(Message::MainViewAction))
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
}
