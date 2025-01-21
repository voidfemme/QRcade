use crate::engine::managers::state_manager::StateManager;
use mlua::{Lua, Result as LuaResult};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_gravity_api(lua: &Lua, state_manager: Rc<RefCell<StateManager>>) -> LuaResult<()> {
    // Add downward gravity to entity
    let add_downward_gravity = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_, (entity_id, force, terminal_velocity): (u32, f32, f32)| {
                manager
                    .borrow_mut()
                    .add_downward_gravity(entity_id, force, terminal_velocity)
                    .map_err(mlua::Error::runtime)
            },
        )?
    };

    // Make this entity attract other entities
    let add_attractive_gravity = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_, (entity_id, force, terminal_velocity): (u32, f32, f32)| {
                manager
                    .borrow_mut()
                    .add_attractive_gravity(entity_id, force, terminal_velocity)
                    .map_err(mlua::Error::runtime)
            },
        )?
    };

    // Make the entity repel other entities
    let add_repulsive_gravity = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_, (entity_id, force, terminal_velocity): (u32, f32, f32)| {
                manager
                    .borrow_mut()
                    .add_repulsive_gravity(entity_id, force, terminal_velocity)
                    .map_err(mlua::Error::runtime)
            },
        )?
    };

    // Enable/disable gravity for entity
    let set_gravity_enabled = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (entity_id, enabled): (u32, bool)| {
            manager
                .borrow_mut()
                .set_gravity_enabled(entity_id, enabled)
                .map_err(mlua::Error::runtime)
        })?
    };

    // Register functions
    lua.globals()
        .set("add_downward_gravity", add_downward_gravity)?;
    lua.globals()
        .set("set_gravity_enabled", set_gravity_enabled)?;
    lua.globals()
        .set("add_attractive_gravity", add_attractive_gravity)?;
    lua.globals()
        .set("add_repulsive_gravity", add_repulsive_gravity)?;

    Ok(())
}
