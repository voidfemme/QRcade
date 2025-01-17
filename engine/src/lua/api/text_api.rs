use crate::ecs::components::text::{HorizontalAlign, TextId, VerticalAlign};
use crate::engine::managers::state_manager::StateManager;
use mlua::{Lua, Result as LuaResult, Table};
use std::rc::Rc;

fn string_to_text_id(text: &str) -> Option<TextId> {
    match text.to_uppercase().as_str() {
        "GAME_OVER" => Some(TextId::GameOver),
        "PRESS_SPACE" => Some(TextId::PressSpace),
        "SCORE" => Some(TextId::Score),
        "YOU_WIN" => Some(TextId::YouWin),
        "GAME_PAUSED" => Some(TextId::GamePaused),
        "PRESS_ESC_TO_QUIT" => Some(TextId::PressEscToQuit),
        "LIVES" => Some(TextId::Lives),
        "LEVEL" => Some(TextId::Level),
        "READY" => Some(TextId::Ready),
        "GO" => Some(TextId::Go),
        "HIGH_SCORE" => Some(TextId::HighScore),
        "NEW_HIGH_SCORE" => Some(TextId::NewHighScore),
        "TRY_AGAIN" => Some(TextId::TryAgain),
        _ => None,
    }
}

fn string_to_horizontal_align(align: &str) -> Option<HorizontalAlign> {
    match align.to_uppercase().as_str() {
        "LEFT" => Some(HorizontalAlign::Left),
        "CENTER" => Some(HorizontalAlign::Center),
        "RIGHT" => Some(HorizontalAlign::Right),
        _ => None,
    }
}

fn string_to_vertical_align(align: &str) -> Option<VerticalAlign> {
    match align.to_uppercase().as_str() {
        "TOP" => Some(VerticalAlign::Top),
        "MIDDLE" => Some(VerticalAlign::Middle),
        "BOTTOM" => Some(VerticalAlign::Bottom),
        _ => None,
    }
}

pub fn register_text_api(lua: &Lua, state_manager: Rc<StateManager>) -> LuaResult<()> {
    // Create TEXT table to hold our text constants
    let text_constants = lua.create_table()?;
    text_constants.set("GAME_OVER", "GAME_OVER")?;
    text_constants.set("PRESS_SPACE", "PRESS_SPACE")?;
    text_constants.set("SCORE", "SCORE")?;
    text_constants.set("YOU_WIN", "YOU_WIN")?;
    text_constants.set("GAME_PAUSED", "GAME_PAUSED")?;
    text_constants.set("PRESS_ESC_TO_QUIT", "PRESS_ESC_TO_QUIT")?;
    text_constants.set("LIVES", "LIVES")?;
    text_constants.set("LEVEL", "LEVEL")?;
    text_constants.set("READY", "READY")?;
    text_constants.set("GO", "GO")?;
    text_constants.set("HIGH_SCORE", "HIGH_SCORE")?;
    text_constants.set("NEW_HIGH_SCORE", "NEW_HIGH_SCORE")?;
    text_constants.set("TRY_AGAIN", "TRY_AGAIN")?;

    // Add alignment constants
    let align = lua.create_table()?;
    // Horizontal
    align.set("LEFT", "LEFT")?;
    align.set("CENTER", "CENTER")?;
    align.set("RIGHT", "RIGHT")?;
    // Vertical
    align.set("TOP", "TOP")?;
    align.set("MIDDLE", "MIDDLE")?;
    align.set("BOTTOM", "BOTTOM")?;

    // Function to add text to an entity
    let add_text = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(
            move |_, (entity_id, text_id_str, params): (u32, String, Option<Table>)| {
                let text_id = string_to_text_id(&text_id_str)
                    .ok_or_else(|| mlua::Error::runtime("Invalid text ID"))?;

                let mut color = None;
                let mut scale = None;
                let mut h_align = None;
                let mut v_align = None;

                if let Some(params) = params {
                    // Get color if provided
                    if let Ok(color_table) = params.get::<Table>("color") {
                        let r: u8 = color_table.get(1)?;
                        let g: u8 = color_table.get(2)?;
                        let b: u8 = color_table.get(3)?;
                        color = Some((r, g, b));
                    }

                    // Get scale if provided
                    if let Ok(s) = params.get::<f32>("scale") {
                        scale = Some(s);
                    }

                    // Get alignment if provided
                    if let Ok(h) = params.get::<String>("h_align") {
                        h_align = string_to_horizontal_align(&h);
                    }
                    if let Ok(v) = params.get::<String>("v_align") {
                        v_align = string_to_vertical_align(&v);
                    }
                }

                manager
                    .add_text(entity_id, text_id, color, scale, h_align, v_align)
                    .map_err(mlua::Error::runtime)
            },
        )?
    };

    // Function to update text
    let update_text = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (entity_id, text_id_str): (u32, String)| {
            let text_id = string_to_text_id(&text_id_str)
                .ok_or_else(|| mlua::Error::runtime("Invalid text ID"))?;

            manager
                .update_text(entity_id, text_id)
                .map_err(mlua::Error::runtime)
        })?
    };

    // Function to set text color
    let set_text_color = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (entity_id, r, g, b): (u32, u8, u8, u8)| {
            manager
                .set_text_color(entity_id, (r, g, b))
                .map_err(mlua::Error::runtime)
        })?
    };

    // Function to set text scale
    let set_text_scale = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (entity_id, scale): (u32, f32)| {
            manager
                .set_text_scale(entity_id, scale)
                .map_err(mlua::Error::runtime)
        })?
    };

    // Function to set text visibility
    let set_text_visibility = {
        let manager = Rc::clone(&state_manager);
        lua.create_function(move |_, (entity_id, visible): (u32, bool)| {
            manager
                .set_text_visibility(entity_id, visible)
                .map_err(mlua::Error::runtime)
        })?
    };

    // Register the text constants table
    lua.globals().set("TEXT", text_constants)?;
    lua.globals().set("ALIGN", align)?;

    // Register all the functions
    lua.globals().set("add_text", add_text)?;
    lua.globals().set("update_text", update_text)?;
    lua.globals().set("set_text_color", set_text_color)?;
    lua.globals().set("set_text_scale", set_text_scale)?;
    lua.globals()
        .set("set_text_visibility", set_text_visibility)?;

    Ok(())
}
