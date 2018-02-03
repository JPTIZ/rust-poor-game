use super::game::Game;


pub trait GameScene {
    fn start(&mut self, &mut Game);
    fn update(&mut self, &mut Game);
    fn terminate(&mut self, &mut Game);
    fn running(&self) -> bool;
}


pub fn run(scene: &mut GameScene, mut game: &mut Game) {
    scene.start(&mut game);
    while scene.running() && game.running() {
        scene.update(&mut game);
    }
    scene.terminate(&mut game);
}
