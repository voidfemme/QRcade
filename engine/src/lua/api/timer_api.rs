use crate::engine::managers::state_manager::StateManager;
use mlua::{Function, Lua, Result as LuaResult};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_timer_api(lua: &Lua, state_manager: Rc<RefCell<StateManager>>) -> LuaResult<()> {
    // Register set_interval function
    let set_interval = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_, (callback, interval, repeat): (Function, f32, bool)| {
                manager
                    .borrow_mut()
                    .set_interval(callback, interval, repeat)
                    .map_err(mlua::Error::runtime)
            },
        )?
    };

    // Register clear_timer function
    let clear_timer = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, timer_id: f64| {
            manager
                .borrow_mut()
                .clear_timer(crate::ecs::components::timer::TimerId(timer_id as u32))
                .map_err(mlua::Error::runtime)
        })?
    };

    // Create the timer table
    let timer_table = lua.create_table()?;
    timer_table.set("set_interval", set_interval)?;
    timer_table.set("clear", clear_timer)?;

    // Set the timer table in Lua globals
    lua.globals().set("timer", timer_table)?;

    Ok(())
}
