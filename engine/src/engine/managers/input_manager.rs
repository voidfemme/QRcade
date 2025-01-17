use super::Manager;
use crate::ecs::components::component::GameState;
use crate::ecs::systems::input_system::InputSystem;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::cell::RefCell;
use std::rc::Rc;

pub struct InputManager {
    state: Rc<RefCell<GameState>>,
    input_system: Rc<RefCell<InputSystem>>,
}

impl Manager for InputManager {
    fn new(state: Rc<RefCell<GameState>>) -> Self {
        // This implementation is just to satisfy the trait.
        // We'll use the new_with_input_system constructor instead.
        Self {
            state,
            input_system: Rc::new(RefCell::new(InputSystem::new())),
        }
    }

    fn get_state(&self) -> &Rc<RefCell<GameState>> {
        &self.state
    }
}

impl InputManager {
    pub fn new_with_input_system(
        state: Rc<RefCell<GameState>>,
        input_system: Rc<RefCell<InputSystem>>,
    ) -> Self {
        Self {
            state,
            input_system,
        }
    }

    pub fn handle_key(&self, keycode: Keycode, pressed: bool) -> Result<(), &'static str> {
        match self.input_system.try_borrow_mut() {
            Ok(mut input) => {
                if pressed {
                    input.set_key_pressed(keycode);
                } else {
                    input.set_key_released(keycode);
                }
                Ok(())
            }
            Err(_) => Err("Failed to borrow input system"),
        }
    }

    pub fn is_key_pressed(&self, keycode: Keycode) -> Result<bool, &'static str> {
        match self.input_system.try_borrow() {
            Ok(input) => Ok(input.is_key_pressed(keycode)),
            Err(_) => Err("Failed to borrow input system"),
        }
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> Result<bool, String> {
        match self.input_system.try_borrow() {
            Ok(input) => Ok(input.is_mouse_button_pressed(button)),
            Err(_) => Err("Failed to borrow input system".to_string()),
        }
    }

    pub fn set_mouse_button_pressed(&self, button: MouseButton) -> Result<(), &'static str> {
        match self.input_system.try_borrow_mut() {
            Ok(mut input) => {
                input.set_mouse_button_pressed(button);
                Ok(())
            }
            Err(_) => Err("Failed to borrow input system"),
        }
    }

    pub fn set_mouse_button_released(&self, button: MouseButton) -> Result<(), &'static str> {
        match self.input_system.try_borrow_mut() {
            Ok(mut input) => {
                input.set_mouse_button_released(button);
                Ok(())
            }
            Err(_) => Err("Failed to borrow input system"),
        }
    }

    pub fn update_mouse_position(&self, x: i32, y: i32) -> Result<(), &'static str> {
        match self.input_system.try_borrow_mut() {
            Ok(mut input) => {
                input.update_mouse_position(x, y);
                Ok(())
            }
            Err(_) => Err("Failed to borrow input system"),
        }
    }

    pub fn get_mouse_position(&self) -> Result<(i32, i32), &'static str> {
        match self.input_system.try_borrow() {
            Ok(input) => Ok(input.get_mouse_position()),
            Err(_) => Err("Failed to borrow input system"),
        }
    }

    pub fn clear_all_input(&self) -> Result<(), &'static str> {
        match self.input_system.try_borrow_mut() {
            Ok(mut input) => {
                input.clear_all_input();
                Ok(())
            }
            Err(_) => Err("Failed to borrow input system"),
        }
    }
}
