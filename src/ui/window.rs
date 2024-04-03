use iced::theme;
use iced::widget::{button, column};
use iced::window::{Icon, Settings as WindowSettings};
use iced::{Background, Element, Sandbox, Settings as AppSettings};

use super::components::icon::icon;
use super::components::icon_button::icon_button;

pub struct MainWindow {}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ShowPressed,
}

impl Sandbox for MainWindow {
    type Message = Message;

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        String::from("Test window")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ShowPressed => {
                println!("TEST");
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![icon_button('\u{F00E}')
            // .style(button::StyleSheet)
            .width(100)
            .height(100)
            .on_press(Message::ShowPressed)]
        .into()
    }
}

fn load_icon() -> Option<Icon> {
    let icon_res = iced::window::icon::from_file("src/assets/icons/icon.png");

    match icon_res {
        Ok(file) => Some(file),
        Err(error) => {
            println!("{:?}", error);
            None
        }
    }
}

pub fn spawn_main_window() -> iced::Result {
    let window_settings = AppSettings {
        window: WindowSettings {
            icon: load_icon(),
            ..WindowSettings::default()
        },
        fonts: vec![include_bytes!("../assets/fonts/icon-font.ttf")
            .as_slice()
            .into()],
        ..AppSettings::default()
    };

    MainWindow::run(window_settings)
}
