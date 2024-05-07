use iced::{
    color, theme,
    widget::{button, Button},
    Background, Renderer, Theme,
};

use crate::ui::window::Message;

use super::icon::icon;

// pub fn icon_button<'a>(icon_code: char) -> Button<'a, Message, Theme, Renderer> {
//     button(icon(icon_code))
// }

// pub fn play_button<'a>() -> Button<'a, Message, Theme, Renderer> {
//     button(icon('\u{E804}').size(200))
// }

pub struct PauseButton;
impl button::StyleSheet for PauseButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: color!(255, 255, 255).into(),
            ..button::Appearance::default()
        }
    }
}

pub fn pause_button<'a>() -> Button<'a, Message, Theme, Renderer> {
    button(icon('\u{F00E}').size(200)).style(theme::Button::Custom(Box::new(PauseButton {})))
}
