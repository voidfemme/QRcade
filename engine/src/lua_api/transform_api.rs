use crate::game_state::component::GameState;
use crate::game_state::transform::Transform;
use mlua::{Lua, Result as LuaResult};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_transform_api(lua: &Lua, gamestate: Rc<RefCell<GameState>>) -> LuaResult<()> {
    // Register set_transform function
    let set_transform = {
        let state_ref = Rc::clone(&gamestate);
        lua.create_function(move |_, (entity_id, x, y, rotation, scale_x, scale_y): (u32, f32, f32, f32, f32, f32)| {
            let mut state = state_ref.borrow_mut();
            state.add_transform(entity_id, Transform::new(x, y, rotation, scale_x, scale_y));

            Ok(())
        })?
    };

    // Add get_transform function
    let get_transform = {
        let state_ref = Rc::clone(&gamestate);
        lua.create_function(move |_, entity_id: u32| {
            let state = state_ref.borrow();
            if let Some(transform) = state.transforms.get(&entity_id) {
                // Return multiple values to Lua
                Ok((transform.x, transform.y, transform.rotation))
            } else {
                // If entity doesn't have a transform, return nil
                Err(mlua::Error::runtime("Entity does not have a transform component"))
            }
        })?
    };

    // Add functions to lua globals
    lua.globals().set("set_transform", set_transform)?;
    lua.globals().set("get_transform", get_transform)?;

    Ok(())
}

