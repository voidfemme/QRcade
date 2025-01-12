use crate::lua::runtime::state_manager::StateManager;
use mlua::{Lua, Result as LuaResult};
use std::rc::Rc;

pub fn register_tilemap_api(lua: &Lua, state_manager: Rc<StateManager>) -> LuaResult<()> {
    // Create tilemap
    let create_tilemap = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_, (entity_id, width, height, tile_size): (u32, u32, u32, u32)| {
                manager
                    .create_tilemap(entity_id, width, height, tile_size)
                    .map_err(mlua::Error::runtime)
            },
        )?
    };

    // Set tile
    let set_tile = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_,
                  (entity_id, x, y, tile_id, walkable, r, g, b): (
                u32,
                u32,
                u32,
                u32,
                bool,
                u8,
                u8,
                u8,
            )| {
                manager
                    .set_tile(entity_id, x, y, tile_id, walkable, (r, g, b))
                    .map_err(mlua::Error::runtime)
            },
        )?
    };

    // Clear tile
    let clear_tile = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (entity_id, x, y): (u32, u32, u32)| {
            manager
                .clear_tile(entity_id, x, y)
                .map_err(mlua::Error::runtime)
        })?
    };

    // Check if position is walkable
    let is_walkable = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (entity_id, x, y): (u32, u32, u32)| {
            manager
                .is_tile_walkable(entity_id, x, y)
                .map_err(mlua::Error::runtime)
        })?
    };

    // Register functions
    lua.globals().set("create_tilemap", create_tilemap)?;
    lua.globals().set("set_tile", set_tile)?;
    lua.globals().set("clear_tile", clear_tile)?;
    lua.globals().set("is_walkable", is_walkable)?;

    Ok(())
}

