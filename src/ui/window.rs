use iced::widget::{column, container};
use iced::window::{Icon, Settings as WindowSettings};
use iced::{alignment, Element, Length, Sandbox, Settings as AppSettings};

use super::components::icon_buttons::pause_button;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ToggleTimer,
}
#[derive(Debug, Default)]
pub struct PowerSchedule {
    is_timer_active: bool,
}

impl PowerSchedule {
    fn default() -> Self {
        PowerSchedule {
            is_timer_active: false,
        }
    }
}

impl Sandbox for PowerSchedule {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Power Schedule")
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
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
        // column![pause_button().on_press(Message::ToggleTimer)].into()
        container(pause_button().on_press(Message::ToggleTimer))
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
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

pub fn app_init() -> iced::Result {
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

    PowerSchedule::run(window_settings)
}
