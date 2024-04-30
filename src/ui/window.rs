use std::path::Path;

use iced::widget::column;
use iced::window::{Icon, Settings as WindowSettings};
use iced::{Element, Sandbox, Settings as AppSettings};

use super::components::icon_buttons::pause_button;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ToggleTimer,
}
#[derive(Debug)]
pub struct MainWindow {
    is_timer_active: bool,
}

impl Default for MainWindow {
    fn default() -> Self {
        MainWindow {
            is_timer_active: false,
        }
    }
}

impl Sandbox for MainWindow {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Test window")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ToggleTimer => {
                self.is_timer_active = !self.is_timer_active;
                println!("{:?}", self);
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![pause_button().on_press(Message::ToggleTimer)].into()
    }
}

fn load_icon(path: &str) -> Option<Icon> {
    let icon_res = iced::window::icon::from_file(path);

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
            icon: load_icon("src/assets/icons/icon.png"),
            ..WindowSettings::default()
        },
        fonts: vec![include_bytes!("../assets/fonts/icon-font.ttf")
            .as_slice()
            .into()],
        ..AppSettings::default()
    };

    MainWindow::run(window_settings)
}
