use super::Manager;
use crate::ecs::components::component::GameState;
use crate::ecs::components::text::{HorizontalAlign, Text, TextId, VerticalAlign};
use std::cell::RefCell;
use std::rc::Rc;
use tracing::{debug, error, warn};

#[derive(Debug)]
pub struct TextManager {
    state: Rc<RefCell<GameState>>,
}

impl Manager for TextManager {
    fn new(state: Rc<RefCell<GameState>>) -> Self {
        debug!("Creating new TextManager");
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
                    error!(entity_id, "Attempted to add text to non-existent entity");
                    return Err("Entity does not exist");
                }

                let mut text = Text::new(text_id);
                debug!(entity_id, ?text_id, "Creating new text component");

                if let Some(color) = color {
                    debug!(
                        entity_id,
                        r = color.0,
                        g = color.1,
                        b = color.2,
                        "Setting text color"
                    );
                    text = text.with_color(color.0, color.1, color.2);
                }

                if let Some(scale) = scale {
                    debug!(entity_id, scale, "Setting text scale");
                    text = text.with_scale(scale);
                }

                if let Some(h) = h_align {
                    if let Some(v) = v_align {
                        debug!(entity_id, ?h, ?v, "Setting text alignment");
                        text = text.with_alignment(h, v);
                    }
                }

                state.add_text(entity_id, text);
                debug!(entity_id, "Text component added successfully");
                Ok(())
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state while adding text"
                );
                Err("Failed to borrow game state")
            }
        }
    }

    pub fn update_text(&self, entity_id: u32, text_id: TextId) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if let Some(text) = state.texts.get_mut(&entity_id) {
                    debug!(entity_id, ?text_id, "Updating text content");
                    text.text_id = text_id;
                    Ok(())
                } else {
                    warn!(
                        entity_id,
                        "Attempted to update text on entity without text component"
                    );
                    Err("Entity has no text component")
                }
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state while updating text"
                );
                Err("Failed to borrow game state")
            }
        }
    }

    pub fn set_text_color(&self, entity_id: u32, color: (u8, u8, u8)) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if let Some(text) = state.texts.get_mut(&entity_id) {
                    debug!(
                        entity_id,
                        r = color.0,
                        g = color.1,
                        b = color.2,
                        "Setting text color"
                    );
                    text.color = color;
                    Ok(())
                } else {
                    warn!(
                        entity_id,
                        "Attempted to set color on entity without text component"
                    );
                    Err("Entity has no text component")
                }
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state while setting text color"
                );
                Err("Failed to borrow game state")
            }
        }
    }

    pub fn set_text_scale(&self, entity_id: u32, scale: f32) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if let Some(text) = state.texts.get_mut(&entity_id) {
                    debug!(entity_id, scale, "Setting text scale");
                    text.scale = scale;
                    Ok(())
                } else {
                    warn!(
                        entity_id,
                        "Attempted to set scale on entity without text component"
                    );
                    Err("Entity has no text component")
                }
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state while setting text scale"
                );
                Err("Failed to borrow game state")
            }
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
                    debug!(entity_id, ?h_align, ?v_align, "Setting text alignment");
                    text.alignment.h_align = h_align;
                    text.alignment.v_align = v_align;
                    Ok(())
                } else {
                    warn!(
                        entity_id,
                        "Attempted to set alignment on entity without text component"
                    );
                    Err("Entity has no text component")
                }
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state while setting text alignment"
                );
                Err("Failed to borrow game state")
            }
        }
    }

    pub fn set_text_visibility(&self, entity_id: u32, visible: bool) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if let Some(text) = state.texts.get_mut(&entity_id) {
                    debug!(entity_id, visible, "Setting text visibility");
                    text.set_visibility(visible);
                    Ok(())
                } else {
                    warn!(
                        entity_id,
                        "Attempted to set visibility on entity without text component"
                    );
                    Err("Entity has no text component")
                }
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state while setting text visibility"
                );
                Err("Failed to borrow game state")
            }
        }
    }

    pub fn remove_text(&self, entity_id: u32) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                debug!(entity_id, "Removing text component");
                state.texts.remove(&entity_id);
                Ok(())
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state while removing text"
                );
                Err("Failed to borrow game state")
            }
        }
    }

    pub fn get_text(&self, entity_id: u32) -> Result<Option<Text>, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                let text = state.texts.get(&entity_id).cloned();
                debug!(
                    entity_id,
                    text_exists = text.is_some(),
                    "Retrieved text component"
                );
                Ok(text)
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state while getting text"
                );
                Err("Failed to borrow game state")
            }
        }
    }
}

