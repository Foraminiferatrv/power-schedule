use iced::{
    widget::{button, Button},
    Renderer, Theme,
};

use crate::ui::window::Message;

use super::icon::icon;

pub fn icon_button<'a>(icon_code: char) -> Button<'a, Message, Theme, Renderer> {
    button(icon(icon_code))
}

pub fn play_button<'a>() -> Button<'a, Message, Theme, Renderer> {
    button(icon('\u{E804}').size(200))
}
pub fn pause_button<'a>() -> Button<'a, Message, Theme, Renderer> {
    button(icon('\u{F00E}').size(200))
}
