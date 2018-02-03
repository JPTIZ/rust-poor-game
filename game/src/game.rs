extern crate sfml;

use self::sfml::window::Event;

use graphics::Graphics;
use input::Input;

#[derive(Default)]
pub struct Game {
    pub graphics: Graphics,
    pub input: Input,
    running: bool,
}

impl Game {
    pub fn new() -> Game {
        Game{
            graphics: Graphics::new((640, 480)),
            running: true,
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        println!("[Game] Updating events...");
        let window = &mut self.graphics.window;
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => {
                    println!("[Game] Closing...");
                    window.close();
                    self.running = false;
                },
                Event::KeyPressed{code,..} => {
                    *self.input.time.entry(code).or_insert(0) += 1;
                },
                _ => {},
            }
        }
    }

    pub fn refresh(&mut self) {
        self.graphics.refresh();
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn quit() {
    }
}

