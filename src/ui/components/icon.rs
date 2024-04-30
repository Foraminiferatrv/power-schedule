use iced::widget::{text, Text};
use iced::Font;

pub fn icon<'a>(icon_code: char) -> Text<'a> {
    const ICON_FONT: Font = Font::with_name("icon-font");

    text(icon_code).font(ICON_FONT)
}
