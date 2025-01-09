use crate::game_state::component::GameState;
use mlua::{Lua, Result as LuaResult};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_entity_api(lua: &Lua, gamestate: Rc<RefCell<GameState>>) -> LuaResult<()> {
    // create_entity
    let create_entity = {
        // Capture a reference to your `GameState`
        let state_ref = Rc::clone(&gamestate);
        lua.create_function(move |_, ()| {
            let mut state = state_ref.borrow_mut();
            let entity_id = state.create_entity();
            // Return the new entity ID to Lua
            Ok(entity_id)
        })?
    };

    lua.globals().set("create_entity", create_entity)?;

    // destroy_entity
    let destroy_entity = {
        let state_ref = gamestate;
        lua.create_function(move |_, entity_id: u32| {
            // If you have a `destroy_entity` method in GameState, call it here
            // For now, pretend we do:
            // state_ref.destroy_entity(entity_id);
            Ok(())
        })?
    };

    lua.globals().set("destroy_entity", destroy_entity)?;

    Ok(())
}
