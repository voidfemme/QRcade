use super::Manager;
use crate::ecs::components::component::GameState;
use crate::ecs::components::text::{HorizontalAlign, Text, TextId, VerticalAlign};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct TextManager {
    state: Rc<RefCell<GameState>>,
}

impl Manager for TextManager {
    fn new(state: Rc<RefCell<GameState>>) -> Self {
        Self { state }
    }

    fn get_state(&self) -> &Rc<RefCell<GameState>> {
        &self.state
    }
}

impl TextManager {
    pub fn add_text(
        &self,
        entity_id: u32,
        text_id: TextId,
        color: Option<(u8, u8, u8)>,
        scale: Option<f32>,
        h_align: Option<HorizontalAlign>,
        v_align: Option<VerticalAlign>,
    ) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if !state.entities.contains(&entity_id) {
                    return Err("Entity does not exist");
                }

                let mut text = Text::new(text_id);

                if let Some(color) = color {
                    text = text.with_color(color.0, color.1, color.2);
                }

                if let Some(scale) = scale {
                    text = text.with_scale(scale);
                }

                if let Some(h) = h_align {
                    if let Some(v) = v_align {
                        text = text.with_alignment(h, v);
                    }
                }

                state.add_text(entity_id, text);
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn update_text(&self, entity_id: u32, text_id: TextId) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if let Some(text) = state.texts.get_mut(&entity_id) {
                    text.text_id = text_id;
                    Ok(())
                } else {
                    Err("Entity has no text component")
                }
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn set_text_color(&self, entity_id: u32, color: (u8, u8, u8)) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if let Some(text) = state.texts.get_mut(&entity_id) {
                    text.color = color;
                    Ok(())
                } else {
                    Err("Entity has no text component")
                }
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn set_text_scale(&self, entity_id: u32, scale: f32) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if let Some(text) = state.texts.get_mut(&entity_id) {
                    text.scale = scale;
                    Ok(())
                } else {
                    Err("Entity has no text component")
                }
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn set_text_alignment(
        &self,
        entity_id: u32,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
    ) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if let Some(text) = state.texts.get_mut(&entity_id) {
                    text.alignment.h_align = h_align;
                    text.alignment.v_align = v_align;
                    Ok(())
                } else {
                    Err("Entity has no text component")
                }
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn set_text_visibility(&self, entity_id: u32, visible: bool) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if let Some(text) = state.texts.get_mut(&entity_id) {
                    text.set_visibility(visible);
                    Ok(())
                } else {
                    Err("Entity has no text component")
                }
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn remove_text(&self, entity_id: u32) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                state.texts.remove(&entity_id);
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn get_text(&self, entity_id: u32) -> Result<Option<Text>, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => Ok(state.texts.get(&entity_id).cloned()),
            Err(_) => Err("Failed to borrow game state"),
        }
    }
}
