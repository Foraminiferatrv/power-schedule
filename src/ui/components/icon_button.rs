use iced::{
    widget::{button, Button},
    Renderer, Theme,
};

use crate::ui::window::Message;

use super::icon::icon;

pub fn icon_button<'a>(icon_code: char) -> Button<'a, Message, Theme, Renderer> {
    button(icon('\u{E802}'))
}
