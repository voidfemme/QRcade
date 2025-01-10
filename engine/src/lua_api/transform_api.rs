use super::state_manager::StateManager;
use mlua::{Lua, Result as LuaResult};
use std::rc::Rc;

pub fn register_transform_api(lua: &Lua, state_manager: Rc<StateManager>) -> LuaResult<()> {
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
