use crate::ecs::components::tilemap::{TilemapQuery, TilemapQueryResult};
use crate::engine::managers::state_manager::StateManager;
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

    // Get tilemap
    let get_tilemap = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |lua, entity_id: u32| {
            match manager.get_tilemap(entity_id) {
                Ok(Some(tilemap)) => {
                    // Create a Lua table to represent the tilemap
                    let tilemap_table = lua.create_table()?;

                    // Set basic properties
                    tilemap_table.set("width", tilemap.width)?;
                    tilemap_table.set("height", tilemap.height)?;
                    tilemap_table.set("tile_size", tilemap.tile_size)?;

                    // Create a table for tiles
                    let tiles_table = lua.create_table()?;
                    for (index, tile_option) in tilemap.tiles.iter().enumerate() {
                        if let Some(tile) = tile_option {
                            let tile_table = lua.create_table()?;
                            tile_table.set("tile_id", tile.tile_id)?;
                            tile_table.set("walkable", tile.walkable)?;

                            // Create color table
                            let color_table = lua.create_table()?;
                            color_table.set(1, tile.color.0)?;
                            color_table.set(2, tile.color.1)?;
                            color_table.set(3, tile.color.2)?;
                            tile_table.set("color", color_table)?;

                            tiles_table.set(index + 1, tile_table)?;
                        } else {
                            tiles_table.set(index + 1, lua.create_table()?)?;
                        }
                    }
                    tilemap_table.set("tiles", tiles_table)?;

                    Ok(tilemap_table)
                }
                Ok(None) => Ok(lua.create_table()?),
                Err(e) => Err(mlua::Error::runtime(e)),
            }
        })?
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

    // Query tilemap data
    let query_tilemap = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |lua, (entity_id, query_type, args): (u32, String, mlua::Table)| {
                let query = match query_type.as_str() {
                    "dimensions" => TilemapQuery::Dimensions,
                    "tile" => {
                        let x: u32 = args.get("x")?;
                        let y: u32 = args.get("y")?;
                        TilemapQuery::TileAt(x, y)
                    }
                    "area" => {
                        let x: u32 = args.get("x")?;
                        let y: u32 = args.get("y")?;
                        let width: u32 = args.get("width")?;
                        let height: u32 = args.get("height")?;
                        TilemapQuery::Area {
                            x,
                            y,
                            width,
                            height,
                        }
                    }
                    "row" => {
                        let y: u32 = args.get("y")?;
                        TilemapQuery::Row(y)
                    }
                    "column" => {
                        let x: u32 = args.get("x")?;
                        TilemapQuery::Column(x)
                    }
                    _ => return Err(mlua::Error::runtime("Invalid query type")),
                };

                match manager
                    .query_tilemap(entity_id, query)
                    .map_err(mlua::Error::runtime)?
                {
                    TilemapQueryResult::Dimensions(view) => {
                        let result = lua.create_table()?;
                        result.set("width", view.width)?;
                        result.set("height", view.height)?;
                        result.set("tile_size", view.tile_size)?;
                        Ok(result)
                    }
                    TilemapQueryResult::Tile(tile) => {
                        match tile {
                            Some(t) => {
                                let result = lua.create_table()?;
                                result.set("tile_id", t.tile_id)?;
                                result.set("walkable", t.walkable)?;
                                let color_table = lua.create_table()?;
                                color_table.set(1, t.color.0)?;
                                color_table.set(2, t.color.1)?;
                                color_table.set(3, t.color.2)?;
                                result.set("color", color_table)?;
                                Ok(result)
                            }
                            None => {
                                // Create an empty table instead of returning nil
                                Ok(lua.create_table()?)
                            }
                        }
                    }
                    TilemapQueryResult::Area(tiles) => {
                        let result = lua.create_table()?;
                        for (i, (x, y, tile)) in tiles.into_iter().enumerate() {
                            let tile_table = lua.create_table()?;
                            tile_table.set("x", x)?;
                            tile_table.set("y", y)?;
                            tile_table.set("tile_id", tile.tile_id)?;
                            tile_table.set("walkable", tile.walkable)?;
                            let color_table = lua.create_table()?;
                            color_table.set(1, tile.color.0)?;
                            color_table.set(2, tile.color.1)?;
                            color_table.set(3, tile.color.2)?;
                            tile_table.set("color", color_table)?;
                            result.set(i + 1, tile_table)?;
                        }
                        Ok(result)
                    }
                    TilemapQueryResult::Row(tiles) | TilemapQueryResult::Column(tiles) => {
                        let result = lua.create_table()?;
                        for (i, (pos, tile)) in tiles.into_iter().enumerate() {
                            let tile_table = lua.create_table()?;
                            tile_table.set("position", pos)?;
                            tile_table.set("tile_id", tile.tile_id)?;
                            tile_table.set("walkable", tile.walkable)?;
                            let color_table = lua.create_table()?;
                            color_table.set(1, tile.color.0)?;
                            color_table.set(2, tile.color.1)?;
                            color_table.set(3, tile.color.2)?;
                            tile_table.set("color", color_table)?;
                            result.set(i + 1, tile_table)?;
                        }
                        Ok(result)
                    }
                }
            },
        )?
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

    // Position walkable check
    let check_position_walkable = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_, (entity_id, tilemap_id, x, y): (u32, u32, f32, f32)| {
                manager
                    .check_position_walkable(entity_id, tilemap_id, x, y)
                    .map_err(mlua::Error::runtime)
            },
        )?
    };

    let get_dimensions = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, entity_id: u32| {
            manager
                .get_dimensions(entity_id)
                .map_err(mlua::Error::runtime)
        })?
    };

    let get_tile_size = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, entity_id: u32| {
            manager
                .get_tile_size(entity_id)
                .map_err(mlua::Error::runtime)
        })?
    };

    // Register functions
    lua.globals().set("create_tilemap", create_tilemap)?;
    lua.globals().set("set_tile", set_tile)?;
    lua.globals().set("clear_tile", clear_tile)?;
    lua.globals().set("is_walkable", is_walkable)?;
    lua.globals().set("query_tilemap", query_tilemap)?;
    lua.globals()
        .set("check_position_walkable", check_position_walkable)?; // Add this line!
    lua.globals().set("get_tilemap", get_tilemap)?;
    lua.globals().set("get_dimensions", get_dimensions)?;
    lua.globals().set("get_tile_size", get_tile_size)?;

    Ok(())
}
