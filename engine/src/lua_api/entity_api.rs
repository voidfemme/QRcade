use super::state_manager::StateManager;
use mlua::{Lua, Result as LuaResult};
use std::rc::Rc;

pub fn register_entity_api(lua: &Lua, state_manager: Rc<StateManager>) -> LuaResult<()> {
    // create_entity
    let create_entity = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, ()| manager.create_entity().map_err(mlua::Error::runtime))?
    };

    // destroy_entity
    let destroy_entity = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, entity_id: u32| {
            manager
                .destroy_entity(entity_id)
                .map_err(mlua::Error::runtime)
        })?
    };

    lua.globals().set("create_entity", create_entity)?;
    lua.globals().set("destroy_entity", destroy_entity)?;

    Ok(())
}
