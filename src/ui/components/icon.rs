use iced::{widget::text, Element, Font};

pub fn icon<'a, Message>(icon_code: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("icon-font");

    text(icon_code).font(ICON_FONT).size(100).into()
}
