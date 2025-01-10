use sdl2::keyboard::Keycode;
use std::collections::HashSet;

#[derive(Default)]
pub struct InputSystem {
    pressed_keys: HashSet<Keycode>,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
        }
    }

    pub fn set_key_pressed(&mut self, keycode: Keycode) {
        self.pressed_keys.insert(keycode);
    }

    pub fn set_key_released(&mut self, keycode: Keycode) {
        self.pressed_keys.remove(&keycode);
    }

    pub fn is_key_pressed(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode)
    }
}
