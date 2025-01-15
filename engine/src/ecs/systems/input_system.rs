use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::collections::HashSet;

#[derive(Default)]
pub struct InputSystem {
    pressed_keys: HashSet<Keycode>,
    pressed_buttons: HashSet<MouseButton>,
    // mouse state tracking
    mouse_position: (i32, i32),
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            pressed_buttons: HashSet::new(),
            mouse_position: (0, 0),
        }
    }

    // mouse position updates
    pub fn update_mouse_position(&mut self, x: i32, y: i32) {
        self.mouse_position = (x, y);
    }

    // mouse button state
    pub fn set_mouse_button_pressed(&mut self, button: MouseButton) {
        self.pressed_buttons.insert(button);
    }

    pub fn set_mouse_button_released(&mut self, button: MouseButton) {
        self.pressed_buttons.remove(&button);
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_buttons.contains(&button)
    }

    pub fn get_mouse_position(&self) -> (i32, i32) {
        // return the current mouse position
        self.mouse_position
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
