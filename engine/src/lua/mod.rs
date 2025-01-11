pub mod api;
pub mod runtime;

use mlua::{Function, Lua, Result as LuaResult};

pub use api::{
    collision_api::register_collision_api, entity_api::register_entity_api,
    input_api::register_input_api, renderable_api::register_renderable_api,
    transform_api::register_transform_api,
};
pub use runtime::state_manager::StateManager;

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
