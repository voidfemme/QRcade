use crate::engine::managers::state_manager::StateManager;
use mlua::{Lua, Result as LuaResult};
use std::rc::Rc;

pub fn register_collision_api(lua: &Lua, state_manager: Rc<StateManager>) -> LuaResult<()> {
    let is_colliding = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (entity1, entity2): (u32, u32)| {
            manager
                .check_collision(entity1, entity2)
                .map_err(mlua::Error::runtime)
        })?
    };

    lua.globals().set("is_colliding", is_colliding)?;
    Ok(())
}
