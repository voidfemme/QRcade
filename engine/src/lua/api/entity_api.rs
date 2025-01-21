use crate::engine::managers::state_manager::StateManager;
use mlua::{Lua, Result as LuaResult};
use std::rc::Rc;
use std::cell::RefCell;

pub fn register_entity_api(lua: &Lua, state_manager: Rc<RefCell<StateManager>>) -> LuaResult<()> {
    // create_entity
    let create_entity = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, ()| manager.borrow_mut().create_entity().map_err(mlua::Error::runtime))?
    };

    // destroy_entity
    let destroy_entity = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, entity_id: u32| {
            manager
                .borrow_mut()
                .destroy_entity(entity_id)
                .map_err(mlua::Error::runtime)
        })?
    };

    lua.globals().set("create_entity", create_entity)?;
    lua.globals().set("destroy_entity", destroy_entity)?;

    Ok(())
}
