extern crate sfml;

use self::sfml::window::Event;

use graphics::Graphics;
use input::Input;

#[derive(Default)]
pub struct Game {
    pub graphics: Graphics,
    pub input: Input,
}

impl Game {
    pub fn new() -> Game {
        Game{
            graphics: Graphics::new((640, 480)),
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        println!("[Game] Updating events...");
        let window = &mut self.graphics.window;
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed{code,..} => {
                    *self.input.time.get_mut(&code).unwrap() += 1;
                },

                _ => {},
            }
        }
    }

    pub fn refresh(&mut self) {
        self.graphics.refresh();
    }
}

