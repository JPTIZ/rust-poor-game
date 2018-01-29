use graphics::Graphics;

#[derive(Default)]
pub struct Game {
    pub graphics: Graphics,
}

impl Game {
    pub fn new() -> Game {
        Game{
            graphics: Graphics::new((640, 480)),
        }
    }
}

