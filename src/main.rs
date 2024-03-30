mod ui;

use ui::window::spawn_main_window;

fn main() -> iced::Result {
    spawn_main_window()
}
