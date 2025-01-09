use super::state_manager::StateManager;
use crate::game_state::component::GameState;
use mlua::{Lua, Result as LuaResult};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_entity_api(lua: &Lua, gamestate: Rc<RefCell<GameState>>) -> LuaResult<()> {
    let state_manager = Rc::new(StateManager::new(gamestate));

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
