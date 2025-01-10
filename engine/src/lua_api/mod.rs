pub mod component_api;
pub mod entity_api;
pub mod renderable_api;
pub mod transform_api;
pub mod state_manager;
pub mod input_api;
pub mod collision_api;

use mlua::{Function, Lua, Result as LuaResult};

pub fn call_on_start(lua: &Lua) -> LuaResult<()> {
    if let Ok(on_start) = lua.globals().get::<Function>("on_start") {
        on_start.call::<()>(())?;
    }
    Ok(())
}

pub fn call_on_frame(lua: &Lua, delta_time: f32) -> LuaResult<()> {
    if let Ok(on_frame) = lua.globals().get::<Function>("on_frame") {
        on_frame.call::<()>(delta_time)?;
    }
    Ok(())
}
pub fn call_on_end(lua: &Lua) -> LuaResult<()> {
    if let Ok(on_end) = lua.globals().get::<Function>("on_end") {
        on_end.call::<()>(())?;
    }
    Ok(())
}
