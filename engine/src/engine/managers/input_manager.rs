use super::Manager;
use crate::ecs::components::component::GameState;
use crate::ecs::systems::input_system::InputSystem;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::cell::RefCell;
use std::rc::Rc;
use tracing::{debug, error};

#[derive(Debug)]
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
        debug!("Creating new InputManager with custom input system");
        Self {
            state,
            input_system,
        }
    }

    pub fn handle_key(&self, keycode: Keycode, pressed: bool) -> Result<(), &'static str> {
        match self.input_system.try_borrow_mut() {
            Ok(mut input) => {
                if pressed {
                    debug!(?keycode, "Key pressed");
                    input.set_key_pressed(keycode);
                } else {
                    debug!(?keycode, "Key released");
                    input.set_key_released(keycode);
                }
                Ok(())
            }
            Err(e) => {
                error!(?e, "Failed to borrow input system for key handling");
                Err("Failed to borrow input system")
            }
        }
    }

    pub fn is_key_pressed(&self, keycode: Keycode) -> Result<bool, &'static str> {
        match self.input_system.try_borrow() {
            Ok(input) => {
                let is_pressed = input.is_key_pressed(keycode);
                debug!(?keycode, is_pressed, "Checked key state");
                Ok(is_pressed)
            }
            Err(e) => {
                error!(?e, ?keycode, "Failed to borrow input system for key check");
                Err("Failed to borrow input system")
            }
        }
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> Result<bool, String> {
        match self.input_system.try_borrow() {
            Ok(input) => {
                let is_pressed = input.is_mouse_button_pressed(button);
                debug!(?button, is_pressed, "Checked mouse button state");
                Ok(is_pressed)
            }
            Err(e) => {
                error!(
                    ?e,
                    ?button,
                    "Failed to borrow input system for mouse button check"
                );
                Err("Failed to borrow input system".to_string())
            }
        }
    }

    pub fn set_mouse_button_pressed(&self, button: MouseButton) -> Result<(), &'static str> {
        match self.input_system.try_borrow_mut() {
            Ok(mut input) => {
                debug!(?button, "Mouse button pressed");
                input.set_mouse_button_pressed(button);
                Ok(())
            }
            Err(e) => {
                error!(
                    ?e,
                    ?button,
                    "Failed to borrow input system for mouse button press"
                );
                Err("Failed to borrow input system")
            }
        }
    }

    pub fn set_mouse_button_released(&self, button: MouseButton) -> Result<(), &'static str> {
        match self.input_system.try_borrow_mut() {
            Ok(mut input) => {
                debug!(?button, "Mouse button released");
                input.set_mouse_button_released(button);
                Ok(())
            }
            Err(e) => {
                error!(
                    ?e,
                    ?button,
                    "Failed to borrow input system for mouse button release"
                );
                Err("Failed to borrow input system")
            }
        }
    }

    pub fn update_mouse_position(&self, x: i32, y: i32) -> Result<(), &'static str> {
        match self.input_system.try_borrow_mut() {
            Ok(mut input) => {
                debug!(x, y, "Mouse position updated");
                input.update_mouse_position(x, y);
                Ok(())
            }
            Err(e) => {
                error!(
                    ?e,
                    x, y, "Failed to borrow input system for mouse position update"
                );
                Err("Failed to borrow input system")
            }
        }
    }

    pub fn get_mouse_position(&self) -> Result<(i32, i32), &'static str> {
        match self.input_system.try_borrow() {
            Ok(input) => {
                let position = input.get_mouse_position();
                debug!(x = position.0, y = position.1, "Retrieved mouse position");
                Ok(position)
            }
            Err(e) => {
                error!(?e, "Failed to borrow input system for mouse position check");
                Err("Failed to borrow input system")
            }
        }
    }

    pub fn clear_all_input(&self) -> Result<(), &'static str> {
        match self.input_system.try_borrow_mut() {
            Ok(mut input) => {
                debug!("Clearing all input state");
                input.clear_all_input();
                Ok(())
            }
            Err(e) => {
                error!(?e, "Failed to borrow input system for clearing input");
                Err("Failed to borrow input system")
            }
        }
    }
}

