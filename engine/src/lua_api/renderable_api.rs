use super::state_manager::StateManager;
use crate::GameState;
use mlua::{Lua, Result as LuaResult};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_renderable_api(lua: &Lua, gamestate: Rc<RefCell<GameState>>) -> LuaResult<()> {
    let state_manager = Rc::new(StateManager::new(gamestate));

    let add_rectangle = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_, (entity_id, width, height, r, g, b): (u32, f32, f32, u8, u8, u8)| {
                manager
                    .add_sprite(entity_id, width, height, r, g, b)
                    .map_err(mlua::Error::runtime)
            },
        )?
    };

    lua.globals().set("add_rectangle", add_rectangle)?;
    println!("Registered add_rectangle function");
    Ok(())
}

