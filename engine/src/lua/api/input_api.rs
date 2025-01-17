use crate::engine::managers::state_manager::StateManager;
use mlua::{Lua, Result as LuaResult};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::rc::Rc;

fn string_to_keycode(key: &str) -> Option<Keycode> {
    match key.to_uppercase().as_str() {
        "A" => Some(Keycode::A),
        "B" => Some(Keycode::B),
        "C" => Some(Keycode::C),
        "D" => Some(Keycode::D),
        "E" => Some(Keycode::E),
        "F" => Some(Keycode::F),
        "G" => Some(Keycode::G),
        "H" => Some(Keycode::H),
        "I" => Some(Keycode::I),
        "J" => Some(Keycode::J),
        "K" => Some(Keycode::K),
        "L" => Some(Keycode::L),
        "M" => Some(Keycode::M),
        "N" => Some(Keycode::N),
        "O" => Some(Keycode::O),
        "P" => Some(Keycode::P),
        "Q" => Some(Keycode::Q),
        "R" => Some(Keycode::R),
        "S" => Some(Keycode::S),
        "T" => Some(Keycode::T),
        "U" => Some(Keycode::U),
        "V" => Some(Keycode::V),
        "W" => Some(Keycode::W),
        "X" => Some(Keycode::X),
        "Y" => Some(Keycode::Y),
        "Z" => Some(Keycode::Z),
        "SPACE" => Some(Keycode::Space),
        "LEFT" => Some(Keycode::Left),
        "RIGHT" => Some(Keycode::Right),
        "UP" => Some(Keycode::Up),
        "DOWN" => Some(Keycode::Down),
        "RETURN" => Some(Keycode::Return),
        "TAB" => Some(Keycode::Tab),
        _ => None,
    }
}

// helper function to convert string to mouse button
fn string_to_mousebutton(button: &str) -> Option<MouseButton> {
    match button.to_uppercase().as_str() {
        "LEFT" => Some(MouseButton::Left),
        "RIGHT" => Some(MouseButton::Right),
        "MIDDLE" => Some(MouseButton::Middle),
        _ => None,
    }
}

pub fn register_input_api(lua: &Lua, state_manager: Rc<StateManager>) -> LuaResult<()> {
    // register keyboard input function
    let is_key_pressed = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, key: String| match string_to_keycode(&key) {
            Some(keycode) => manager
                .is_key_pressed(keycode)
                .map_err(mlua::Error::runtime),
            None => Err(mlua::Error::runtime(format!("Invalid key name: {}", key))),
        })?
    };

    // register mouse button function
    let is_mouse_pressed = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_, button: String| match string_to_mousebutton(&button) {
                Some(mouse_btn) => manager
                    .is_mouse_button_pressed(mouse_btn)
                    .map_err(mlua::Error::runtime),
                None => Err(mlua::Error::runtime(format!(
                    "Invalid mouse button: {}",
                    button
                ))),
            },
        )?
    };

    lua.globals().set("is_key_pressed", is_key_pressed)?;
    lua.globals().set("is_mouse_pressed", is_mouse_pressed)?;
    Ok(())
}
