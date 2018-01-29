extern crate game;
extern crate sfml;

use self::game::Game;
use self::game::scenes::GameScene;

#[allow(dead_code)]
#[derive(Default)]
pub struct MainScene {
    running: bool,
}

impl GameScene for MainScene {
    fn start(&mut self) {
        println!("Starting MainScene");
    }

    fn update(&mut self, mut _game: &mut Game) {
        println!("Updating MainScene");
    }

    fn terminate(&mut self) {
        println!("Terminating MainScene");
    }

    fn running(&self) -> bool {
        self.running
    }
}
