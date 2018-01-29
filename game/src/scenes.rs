use super::game::Game;


pub trait GameScene {
    fn start(&mut self);
    fn update(&mut self, &mut Game);
    fn terminate(&mut self);
    fn running(&self) -> bool;
}


pub fn run(scene: &mut GameScene, mut game: &mut Game) {
    scene.start();
    while scene.running() {
        scene.update(&mut game);
    }
    scene.terminate();
}
