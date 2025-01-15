use crate::lua::runtime::state_manager::StateManager;
use mlua::{Lua, Result as LuaResult};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::rc::Rc;

pub fn register_drag_drop_api(lua: &Lua, state_manager: Rc<StateManager>) -> LuaResult<()> {
    // function to check if an entity can be dragged from a point
    let can_drag_entity = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_, (x, y): (f32, f32)| match manager.get_entity_at_point(x, y) {
                Ok(Some(entity_id)) => Ok(Some(entity_id)),
                Ok(None) => Ok(None),
                Err(e) => Err(mlua::Error::runtime(e)),
            },
        )?
    };

    // function to start dragging an entity
    let start_drag = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (entity_id, x, y): (u32, f32, f32)| {
            manager
                .start_dragging(entity_id, x, y)
                .map_err(mlua::Error::runtime)
        })?
    };

    // function to update the dragged entity position
    let update_drag = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (x, y): (f32, f32)| {
            manager
                .update_dragged_entity(x, y)
                .map_err(mlua::Error::runtime)
        })?
    };

    // function to end dragging
    let end_drag = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, ()| manager.end_dragging().map_err(mlua::Error::runtime))?
    };

    // function to check if an entity is being dragged
    let is_dragging = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, entity_id: u32| Ok(manager.is_entity_dragged(entity_id)))?
    };

    // register all functions in the Lua environment
    lua.globals().set("can_drag_entity", can_drag_entity)?;
    lua.globals().set("start_drag", start_drag)?;
    lua.globals().set("update_drag", update_drag)?;
    lua.globals().set("end_drag", end_drag)?;
    lua.globals().set("is_dragging", is_dragging)?;

    Ok(())
}
