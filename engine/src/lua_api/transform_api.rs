use super::state_manager::StateManager;
use crate::GameState;
use mlua::{Lua, Result as LuaResult};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_transform_api(lua: &Lua, gamestate: Rc<RefCell<GameState>>) -> LuaResult<()> {
    let state_manager = Rc::new(StateManager::new(gamestate));

    let set_transform = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (entity_id, x, y, rotation, scale_x, scale_y): (u32, f32, f32, f32, f32, f32)| {
            manager.set_transform(entity_id, x, y, rotation, scale_x, scale_y)
                .map_err(mlua::Error::runtime)
        })?
    };

    let get_transform = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, entity_id: u32| {
            manager
                .get_transform(entity_id)
                .map_err(mlua::Error::runtime)
        })?
    };

    lua.globals().set("set_transform", set_transform)?;
    lua.globals().set("get_transform", get_transform)?;
    Ok(())
}

