use iced::widget::button;
use iced::{Sandbox, Settings};

fn main() -> iced::Result {
    TestWindow::run(Settings::default())
}

struct TestWindow {}

#[derive(Debug, Clone, Copy)]
enum Message {
    ShowPressed,
}

impl Sandbox for TestWindow {
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

    fn view(&self) -> iced::Element<'_, Self::Message> {
        button("Button").on_press(Message::ShowPressed).into()
    }
}
