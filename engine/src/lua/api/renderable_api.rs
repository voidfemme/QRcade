use crate::engine::managers::state_manager::StateManager;
use mlua::{Lua, Result as LuaResult, Table};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_renderable_api(
    lua: &Lua,
    state_manager: Rc<RefCell<StateManager>>,
) -> LuaResult<()> {
    // Create the add_shape function that works with our asset system
    let add_shape = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_,
                  (entity_id, shape_name, r, g, b, params): (
                u32,
                String,
                u8,
                u8,
                u8,
                Option<Table>,
            )| {
                // We can now directly add the sprite using the asset name
                manager
                    .borrow_mut()
                    .add_sprite(entity_id, &shape_name, r, g, b, params)
                    .map_err(mlua::Error::runtime)
            },
        )?
    };

    // Register our primary shape-adding function
    lua.globals().set("add_shape", add_shape)?;
    println!("Registered shape rendering functions");
    Ok(())
}
