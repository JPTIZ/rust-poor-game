/*
extern crate sfml;

use sfml::graphics::RenderWindow;

use sfml::window::{
    ContextSettings,
    Event,
    Style,
};
*/

extern crate game;

use self::game::Game;
use self::game::scenes::run_scene;

mod scenes;

use self::scenes::MainScene;


fn main() {
    let mut game = Game::new();
    let mut scene = MainScene::new(&mut game);
    run_scene(&mut scene);

    /*
    let context_settings = ContextSettings{
        antialiasing_level: 0,
        ..Default::default()
    };

    let mut window = RenderWindow::new(
        (800, 600),
        "RPG",
        Style::CLOSE,
        &context_settings
        );

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed { code, .. } => {
                    println!("pressed {key:?}", key=code);
                },
                _ => {},
            }
        }
        window.display();
    }
    */
}

