mod component;
mod style;
mod views;
mod widgets;

use crate::gui::component::Component;
use crate::gui::views::main_view::{MainView, Message as MainViewMessage};

use iced::{window::Settings as Window, Application, Column, Command, Element, Length, Settings};

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
        }
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
    pub fn start() {
        let settings: Settings<()> = Settings {
            window: Window {
                size: (400, 560),
                resizable: false,
                decorations: true,
                ..iced::window::Settings::default()
            },
            default_text_size: 17,
            ..iced::Settings::default()
        };
        Self::run(settings).unwrap_err();
    }
}
