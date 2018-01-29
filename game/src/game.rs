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

    pub fn update(&mut self) {
        println!("[Input] Updating input");
    }

    pub fn refresh(&mut self) {
        self.graphics.show();
    }
}

