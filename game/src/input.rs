extern crate sfml;

use std::collections::HashMap;

use self::sfml::window::Key;


#[derive(Default)]
pub struct Input {
    pub time: HashMap<Key, i32>,
}

impl Input {
    pub fn pressed(&self, key: &Key) -> bool {
        self.time[&key] > 0
    }
}
