extern crate game;

use self::game::Game;
use self::game::scenes::GameScene;
use self::game::scenes::run_scene;

struct TestScene<'game> {
    pub game: &'game mut Game,
}

impl<'game> GameScene for TestScene<'game> {
    fn start(&mut self) {
        println!("Starting TestScene");
    }

    fn update(&mut self) {
        println!("Updating TestScene");
    }

    fn terminate(&mut self) {
        println!("Terminating TestScene");
    }
}

pub struct MainScene<'game> {
    game: &'game mut Game,
}

impl<'game> MainScene<'game> {
    pub fn new(game: &mut Game) -> MainScene {
        MainScene{game}
    }
}

impl<'game> GameScene for MainScene<'game> {
    fn start(&mut self) {
        println!("Starting MainScene");
    }

    fn update(&mut self) {
        println!("Updating MainScene");
        let mut subscene = TestScene{game: &mut self.game};
        run_scene(&mut subscene);
    }

    fn terminate(&mut self) {
        println!("Terminating MainScene");
    }
}
