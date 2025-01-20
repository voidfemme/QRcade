use crate::ecs::components::gamestate::GameState;
use crate::ecs::systems::input_system::InputSystem;
use sdl2::mouse::MouseButton;

pub struct DragDropSystem {
    dragged_entity: Option<u32>,
    drag_offset_x: f32,
    drag_offset_y: f32,
}

impl DragDropSystem {
    pub fn new() -> Self {
        Self {
            dragged_entity: None,
            drag_offset_x: 0.0,
            drag_offset_y: 0.0,
        }
    }

    // Explicitly start a drag operation
    pub fn start_drag(
        &mut self,
        entity: u32,
        mouse_x: f32,
        mouse_y: f32,
        transform_x: f32,
        transform_y: f32,
    ) {
        self.dragged_entity = Some(entity);
        self.drag_offset_x = transform_x - mouse_x;
        self.drag_offset_y = transform_y - mouse_y;
        println!(
            "Started dragging entity {} with offset ({}, {})",
            entity, self.drag_offset_x, self.drag_offset_y
        );
    }

    pub fn end_drag(&mut self) {
        if self.dragged_entity.is_some() {
            println!("Stopped dragging entity {:?}", self.dragged_entity);
            self.dragged_entity = None;
        }
    }

    pub fn update(&mut self, input: &InputSystem, state: &mut GameState) {
        // Only update position if we're already dragging something
        if let Some(entity) = self.dragged_entity {
            if let Some(draggable) = state.draggables.get(&entity) {
                if !draggable.enabled {
                    self.end_drag();
                    return;
                }

                // Get current mouse position and convert to world coordinates
                let (mouse_x, mouse_y) = input.get_mouse_position();
                let world_x = mouse_x as f32;
                let world_y = mouse_y as f32;
                // Print debug info
                println!(
                    "DragDropSystem update - Mouse dragging entity {} at: ({}, {})",
                    entity, world_x, world_y
                );

                // update position of dragged entity
                if let Some(transform) = state.transforms.get_mut(&entity) {
                    let new_x = world_x + self.drag_offset_x;
                    let new_y = world_y + self.drag_offset_y;
                    println!("Moving entity to: ({}, {})", new_x, new_y);
                    transform.x = new_x;
                    transform.y = new_y;
                }

                if !input.is_mouse_button_pressed(MouseButton::Left) {
                    self.end_drag();
                }
            } else {
                self.end_drag();
            }
        }
    }

    pub fn find_entity_under_mouse(
        &self,
        state: &GameState,
        mouse_x: f32,
        mouse_y: f32,
    ) -> Option<u32> {
        // simple point-in-rectangle collision check
        for (&entity_id, transform) in &state.transforms {
            // you'll need to get the entity's size from somewhere -
            // perhaps add a Size component to your ecs
            let half_width = 16.0; // Example size
            let half_height = 16.0;

            if mouse_x >= transform.x - half_width
                && mouse_x <= transform.x + half_width
                && mouse_y >= transform.y - half_height
                && mouse_y <= transform.y + half_height
            {
                return Some(entity_id);
            }
        }
        None
    }
}
