extern crate sfml;

use self::sfml::graphics::RenderWindow;
use self::sfml::window::ContextSettings;
use self::sfml::window::Style;


pub struct Graphics {
    pub size: (u32, u32),
    pub window: RenderWindow,
}

impl Default for Graphics {
    fn default() -> Graphics {
        let size = (800, 600);

        let context_settings = ContextSettings::default();

        let mut window = RenderWindow::new(
            size,
            "RPG",
            Style::DEFAULT,
            &context_settings
            );

        window.set_visible(false);

        Graphics{size,
                 window}
    }
}

impl Graphics {
    pub fn new(size: (u32, u32)) -> Graphics {
        Graphics{size, ..Default::default()}
    }

    pub fn show(&mut self) {
        println!("[Graphics] Showing");
        self.window.set_visible(true);
    }

    pub fn refresh(&mut self) {
        println!("[Graphics] Refreshing");
        self.window.display();
    }

    pub fn hide(&self) {
        println!("[Graphics] Hiding");
    }
}
