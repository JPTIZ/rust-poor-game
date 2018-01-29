pub struct Graphics {
    pub size: (i32, i32),
}

impl Default for Graphics {
    fn default() -> Graphics {
        Graphics{size: (800, 600)}
    }
}

impl Graphics {
    pub fn new(size: (i32, i32)) -> Graphics {
        Graphics{size}
    }

    pub fn show(&self) {
        println!("pretending I'm showing");
    }

    pub fn hide(&self) {
        println!("pretending I'm hiding");
    }
}
