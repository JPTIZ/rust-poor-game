extern crate game;

use self::game::Game;
use self::game::scenes::GameScene;
use self::game::scenes::run;

struct TestScene {
}

impl GameScene for TestScene {
    fn start(&mut self) {
        println!("Starting TestScene");
    }

    fn update(&mut self, _: &mut Game) {
        println!("Updating TestScene");
    }

    fn terminate(&mut self) {
        println!("Terminating TestScene");
    }

    fn running(&self) -> bool {
        false
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct MainScene {
}

impl GameScene for MainScene {
    fn start(&mut self) {
        println!("Starting MainScene");
    }

    fn update(&mut self, mut game: &mut Game) {
        println!("Updating MainScene");
        let mut subscene = TestScene{};
        run(&mut subscene, &mut game);
    }

    fn terminate(&mut self) {
        println!("Terminating MainScene");
    }

    fn running(&self) -> bool {
        true
    }
}
