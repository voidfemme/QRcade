use crate::game_state::component::GameState;
use crate::game_state::sprite::Sprite;
use mlua::{Lua, Result as LuaResult};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_renderable_api(lua: &Lua, gamestate: Rc<RefCell<GameState>>) -> LuaResult<()> {
    let add_rectangle = {
        let state_ref = Rc::clone(&gamestate);
        lua.create_function(
            move |_, (entity_id, width, height, r, g, b): (u32, f32, f32, u8, u8, u8)| {
                let mut state = state_ref.borrow_mut();
                let sprite = Sprite::new_rectangle(width, height, r, g, b);
                state.add_sprite(entity_id, sprite);
                Ok(())
            },
        )?
    };

    lua.globals().set("add_rectangle", add_rectangle)?;
    Ok(())
}
