extern crate game;

mod scenes;

use self::game::Game;
use self::game::scenes::run;
use self::scenes::MainScene;


fn main() {
    let mut game = Game::new();
    let mut scene = MainScene::default();
    run(&mut scene, &mut game);
}

