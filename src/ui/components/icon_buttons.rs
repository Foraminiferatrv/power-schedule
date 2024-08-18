use iced::{
    color, theme,
    widget::{button, Button},
    Renderer, Theme,
};

use crate::ui::window::Message;

use super::icon::icon;

pub struct PauseButton;
impl button::StyleSheet for PauseButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: color!(235, 235, 235).into(),
            ..button::Appearance::default()
        }
    }
    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: color!(255, 255, 255).into(),
            ..button::Appearance::default()
        }
    }
    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: color!(205, 205, 205).into(),
            ..button::Appearance::default()
        }
    }
}

pub fn pause_button<'a>(is_timer_active: bool) -> Button<'a, Message, Theme, Renderer> {
    if is_timer_active {
        button(icon('\u{F00E}').size(200)).style(theme::Button::Custom(Box::new(PauseButton {})))
    } else {
        button(icon('\u{F00F}').size(200)).style(theme::Button::Custom(Box::new(PauseButton {})))
    }
}

