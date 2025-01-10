use crate::assets::asset_manager::PrimitiveShape;
use crate::lua::runtime::state_manager::StateManager;
use mlua::{Lua, Result as LuaResult};
use std::rc::Rc;

pub fn register_renderable_api(lua: &Lua, state_manager: Rc<StateManager>) -> LuaResult<()> {
    let add_shape = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_, (entity_id, asset_name, r, g, b): (u32, String, u8, u8, u8)| {
                // Get the asset definition
                if let Some(asset) = manager.get_asset(&asset_name) {
                    match asset.shape {
                        PrimitiveShape::Rectangle { width, height } => manager
                            .add_sprite(entity_id, width, height, r, g, b)
                            .map_err(mlua::Error::runtime),
                        PrimitiveShape::Circle { radius } => {
                            let diameter = radius * 2.0;
                            manager
                                .add_sprite(entity_id, diameter, diameter, r, g, b)
                                .map_err(mlua::Error::runtime)
                        }
                    }
                } else {
                    Err(mlua::Error::runtime(format!(
                        "Unknown asset: {}",
                        asset_name
                    )))
                }
            },
        )?
    };

    lua.globals().set("add_shape", add_shape)?;
    Ok(())
}
