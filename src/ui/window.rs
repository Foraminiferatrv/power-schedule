use iced::widget::container;
use iced::window::{Icon, Settings as WindowSettings};
use iced::{alignment, Element, Length, Sandbox, Settings as AppSettings, Size};
use std::thread;
use std::time::Duration;

use super::components::icon_buttons::pause_button;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ToggleTimer,
}
#[derive(Debug, Default)]
pub struct PowerSchedule {
    is_timer_active: bool,
    shutdown_duration: Duration,
}

impl PowerSchedule {
    fn default() -> Self {
        PowerSchedule {
            is_timer_active: false,
            shutdown_duration: Duration::from_secs(500),
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
        println!("Message: {:?} || Self: {:?}", message, self);

        match message {
            Message::ToggleTimer => {
                self.is_timer_active = !self.is_timer_active;

                if self.is_timer_active {
                    for _i in 0..self.shutdown_duration.as_secs() {
                        self.shutdown_duration = self.shutdown_duration - Duration::from_secs(1);
                        println!("{:?}", self.shutdown_duration);
                        thread::sleep(Duration::from_secs(1));
                        //TODO: SPawn separate thread to count down the timer
                    }
                    // for
                } else {
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        // column![pause_button().on_press(Message::ToggleTimer)].into()
        container(pause_button(self.is_timer_active).on_press(Message::ToggleTimer))
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
            size: Size::new(600.0, 300.0),
            resizable: false,
            ..WindowSettings::default()
        },
        fonts: vec![include_bytes!("../assets/fonts/icon-font.ttf")
            .as_slice()
            .into()],
        ..AppSettings::default()
    };

    PowerSchedule::run(window_settings)
}
