use crate::ecs::components::draggable::Draggable;
use crate::engine::managers::state_manager::StateManager;
use mlua::{Lua, Result as LuaResult};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_drag_drop_api(
    lua: &Lua,
    state_manager: Rc<RefCell<StateManager>>,
) -> LuaResult<()> {
    // function to check if an entity can be dragged from a point
    let can_drag_entity = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (x, y): (f32, f32)| {
            match manager.borrow().get_entity_at_point(x, y) {
                Ok(Some(entity_id)) => Ok(Some(entity_id)),
                Ok(None) => Ok(None),
                Err(e) => Err(mlua::Error::runtime(e)),
            }
        })?
    };

    // function to start dragging an entity
    let start_drag = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (entity_id, x, y): (u32, f32, f32)| {
            manager
                .borrow_mut()
                .start_dragging(entity_id, x, y)
                .map_err(mlua::Error::runtime)
        })?
    };

    // function to update the dragged entity position
    let update_drag = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (x, y): (f32, f32)| {
            manager
                .borrow_mut()
                .update_dragged_entity(x, y)
                .map_err(mlua::Error::runtime)
        })?
    };

    // function to end dragging
    let end_drag = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, ()| {
            manager
                .borrow_mut()
                .end_dragging()
                .map_err(mlua::Error::runtime)
        })?
    };

    // function to check if an entity is being dragged
    let is_dragging = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, entity_id: u32| {
            Ok(manager.borrow().is_entity_dragged(entity_id))
        })?
    };

    // Function to make an entity draggable
    let make_entity_draggable = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, entity_id: u32| {
            if let Ok(mut state) = manager.borrow_mut().state.try_borrow_mut() {
                if state.entities.contains(&entity_id) {
                    state.add_draggable(entity_id, Draggable::new());
                    Ok(())
                } else {
                    Err(mlua::Error::runtime("Entity does not exist"))
                }
            } else {
                Err(mlua::Error::runtime("Failed to borrow game state"))
            }
        })?
    };

    // Function to remove a draggable component
    let remove_entity_draggable = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, entity_id: u32| {
            if let Ok(mut state) = manager.borrow_mut().state.try_borrow_mut() {
                state.remove_draggable(entity_id);
                Ok(())
            } else {
                Err(mlua::Error::runtime("Failed to borrow game state"))
            }
        })?
    };

    // Function to enable/disable dragging for an entity
    let set_draggable_enabled = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (entity_id, enabled): (u32, bool)| {
            if let Ok(mut state) = manager.borrow_mut().state.try_borrow_mut() {
                if let Some(draggable) = state.draggables.get_mut(&entity_id) {
                    draggable.set_enabled(enabled);
                    Ok(())
                } else {
                    Err(mlua::Error::runtime("Entity is not draggable"))
                }
            } else {
                Err(mlua::Error::runtime("Failed to borrow game state"))
            }
        })?
    };

    // register all functions in the Lua environment
    lua.globals().set("can_drag_entity", can_drag_entity)?;
    lua.globals().set("start_drag", start_drag)?;
    lua.globals().set("update_drag", update_drag)?;
    lua.globals().set("end_drag", end_drag)?;
    lua.globals().set("is_dragging", is_dragging)?;
    lua.globals()
        .set("make_entity_draggable", make_entity_draggable)?;
    lua.globals()
        .set("remove_entity_draggable", remove_entity_draggable)?;
    lua.globals()
        .set("set_draggable_enabled", set_draggable_enabled)?;

    Ok(())
}
