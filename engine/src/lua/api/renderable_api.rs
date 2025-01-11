use crate::assets::asset_manager::PrimitiveShape;
use crate::lua::runtime::state_manager::StateManager;
use mlua::{Lua, Result as LuaResult};
use std::rc::Rc;

pub fn register_renderable_api(lua: &Lua, state_manager: Rc<StateManager>) -> LuaResult<()> {
    // Create the add_shape function that works with our asset system
    let add_shape = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_, (entity_id, shape_name, r, g, b): (u32, String, u8, u8, u8)| {
                // We can now directly add the sprite using the asset name
                manager
                    .add_sprite(entity_id, &shape_name, r, g, b)
                    .map_err(mlua::Error::runtime)
            },
        )?
    };

    // Register our primary shape-adding function
    lua.globals().set("add_shape", add_shape)?;

    // For backwards compatibility with old code that uses add_rectangle
    let add_rectangle = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_, (entity_id, width, height, r, g, b): (u32, f32, f32, u8, u8, u8)| {
                // For backward compatibility, we'll use the "square" asset
                // This maintains compatibility while moving to the asset system
                manager
                    .add_sprite(entity_id, "square", r, g, b)
                    .map_err(mlua::Error::runtime)
            },
        )?
    };

    lua.globals().set("add_rectangle", add_rectangle)?;
    println!("Registered shape rendering functions");
    Ok(())
}
