extern crate game;
extern crate sfml;

use self::game::Game;
use self::game::scenes::GameScene;

#[allow(dead_code)]
pub struct MainScene {
    running: bool,
}

impl MainScene {
    pub fn default() -> MainScene {
        MainScene{running: true}
    }
}

impl GameScene for MainScene {
    fn start(&mut self, game: &mut Game) {
        println!("Starting MainScene");
        game.graphics.show();
    }

    fn update(&mut self, game: &mut Game) {
        println!("Updating MainScene");
        game.refresh();
    }

    fn terminate(&mut self, _game: &mut Game) {
        println!("Terminating MainScene");
    }

    fn running(&self) -> bool {
        self.running
    }
}
