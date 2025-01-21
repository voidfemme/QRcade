use crate::engine::managers::state_manager::StateManager;
use mlua::{Lua, Result as LuaResult};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_velocity_api(lua: &Lua, state_manager: Rc<RefCell<StateManager>>) -> LuaResult<()> {
    // Register set_velocity
    let set_velocity = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_, (entity_id, velocity_x, velocity_y): (u32, f32, f32)| {
                manager
                    .borrow_mut()
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
                .borrow()
                .get_velocity(entity_id)
                .map_err(mlua::Error::runtime)
        })?
    };

    // New zero velocity
    let set_zero_velocity = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, entity_id: u32| {
            manager
                .borrow_mut()
                .set_zero_velocity(entity_id)
                .map_err(mlua::Error::runtime)
        })?
    };

    // New horizontal velocity
    let set_horizontal_velocity = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (entity_id, speed): (u32, f32)| {
            manager
                .borrow_mut()
                .set_horizontal_velocity(entity_id, speed)
                .map_err(mlua::Error::runtime)
        })?
    };

    // New rotational velocity
    let set_rotation_velocity = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_, (entity_id, dx, dy, angular): (u32, f32, f32, f32)| {
                manager
                    .borrow_mut()
                    .set_rotation_velocity(entity_id, dx, dy, angular)
                    .map_err(mlua::Error::runtime)
            },
        )?
    };

    // New angular velocity
    let set_angular_velocity = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (entity_id, angular): (u32, f32)| {
            manager
                .borrow_mut()
                .set_angular_velocity(entity_id, angular)
                .map_err(mlua::Error::runtime)
        })?
    };

    // Add functions to Lua global environment
    lua.globals().set("set_velocity", set_velocity)?;
    lua.globals().set("get_velocity", get_velocity)?;
    lua.globals().set("set_zero_velocity", set_zero_velocity)?;
    lua.globals()
        .set("set_horizontal_velocity", set_horizontal_velocity)?;
    lua.globals()
        .set("set_rotation_velocity", set_rotation_velocity)?;
    lua.globals()
        .set("set_angular_velocity", set_angular_velocity)?;

    Ok(())
}
