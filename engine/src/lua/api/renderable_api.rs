use crate::lua::runtime::state_manager::StateManager;
use mlua::{Lua, Result as LuaResult};
use std::rc::Rc;

pub fn register_renderable_api(lua: &Lua, state_manager: Rc<StateManager>) -> LuaResult<()> {
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
