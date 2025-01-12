use crate::lua::runtime::state_manager::StateManager;
use mlua::{Lua, Result as LuaResult};
use std::rc::Rc;

pub fn register_velocity_api(lua: &Lua, state_manager: Rc<StateManager>) -> LuaResult<()> {
    // Register set_velocity
    let set_velocity = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_, (entity_id, velocity_x, velocity_y): (u32, f32, f32)| {
                manager
                    .set_velocity(entity_id, velocity_x, velocity_y)
                    .map_err(mlua::Error::runtime)
            },
        )?
    };

    // Register get_velocity
    let get_velocity = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, entity_id: u32| {
            manager
                .get_velocity(entity_id)
                .map_err(mlua::Error::runtime)
        })?
    };

    // Add functions to Lua global environment
    lua.globals().set("set_velocity", set_velocity)?;
    lua.globals().set("get_velocity", get_velocity)?;

    Ok(())
}
