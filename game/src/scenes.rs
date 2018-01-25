pub trait GameScene {
    fn start(&mut self);
    fn update(&mut self);
    fn terminate(&mut self);
}

pub fn run_scene(scene: &mut GameScene) {
    scene.start();
    scene.update();
    scene.terminate();
}
