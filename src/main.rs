extern crate sfml;

use sfml::graphics::RenderWindow;

use sfml::window::{
    ContextSettings,
    Style,
};

fn main() {
    let context_settings = ContextSettings{
        antialiasing_level: 0,
        ..Default::default()
    };

    let mut window = RenderWindow::new(
        (640, 480),
        "RPG",
        Style::CLOSE,
        &context_settings
        );

    while window.is_open() {
        window.display();
    }
}
