use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use std::cell::RefCell;
use std::rc::Rc;

use crate::assets::asset_manager::{AssetManager, BuiltInAsset, PrimitiveShape};
use crate::ecs::components::component::GameState;
use crate::ecs::components::sprite::SpriteShapeData;
use crate::ecs::components::tilemap::{Tilemap, TilemapQuery, TilemapQueryResult};
use crate::ecs::systems::input_system::InputSystem;
use crate::engine::managers::{
    collision_manager::CollisionManager, drag_drop_manager::DragDropManager,
    entity_manager::EntityManager, gravity_manager::GravityManager, input_manager::InputManager,
    tilemap_manager::TilemapManager, transform_manager::TransformManager,
    velocity_manager::VelocityManager, Manager,
};
use crate::{Renderer, Sdl2Renderer};

pub struct StateManager {
    pub state: Rc<RefCell<GameState>>,
    assets: AssetManager,
    // Specialized managers
    entity_manager: EntityManager,
    transform_manager: TransformManager,
    velocity_manager: VelocityManager,
    gravity_manager: GravityManager,
    collision_manager: CollisionManager,
    tilemap_manager: TilemapManager,
    drag_drop_manager: DragDropManager,
    input_manager: InputManager,
}

impl StateManager {
    pub fn new(state: Rc<RefCell<GameState>>, input_system: Rc<RefCell<InputSystem>>) -> Self {
        Self {
            state: Rc::clone(&state),
            assets: AssetManager::new(),
            entity_manager: EntityManager::new(Rc::clone(&state)),
            transform_manager: TransformManager::new(Rc::clone(&state)),
            velocity_manager: VelocityManager::new(Rc::clone(&state)),
            gravity_manager: GravityManager::new(Rc::clone(&state)),
            collision_manager: CollisionManager::new(Rc::clone(&state)),
            tilemap_manager: TilemapManager::new(Rc::clone(&state)),
            drag_drop_manager: DragDropManager::new(Rc::clone(&state)),
            input_manager: InputManager::new_with_input_system(Rc::clone(&state), input_system),
        }
    }

    // ------------------------------------------------------------
    // Entity Management
    // ------------------------------------------------------------
    pub fn create_entity(&self) -> Result<u32, &'static str> {
        self.entity_manager.create_entity()
    }

    pub fn destroy_entity(&self, entity_id: u32) -> Result<(), &'static str> {
        self.entity_manager.destroy_entity(entity_id)
    }

    pub fn add_sprite(
        &self,
        entity_id: u32,
        asset_name: &str,
        r: u8,
        g: u8,
        b: u8,
        params: Option<mlua::Table>,
    ) -> Result<(), &'static str> {
        self.entity_manager
            .add_sprite(entity_id, asset_name, r, g, b, params, &self.assets)
    }

    // ------------------------------------------------------------
    // Transform Management
    // ------------------------------------------------------------
    pub fn set_transform(
        &self,
        entity_id: u32,
        x: f32,
        y: f32,
        rotation: f32,
    ) -> Result<(), &'static str> {
        self.transform_manager
            .set_transform(entity_id, x, y, rotation)
    }

    pub fn get_transform(&self, entity_id: u32) -> Result<(f32, f32, f32), &'static str> {
        self.transform_manager.get_transform(entity_id)
    }

    // ------------------------------------------------------------
    // Velocity Management
    // ------------------------------------------------------------
    pub fn set_velocity(&self, entity_id: u32, vx: f32, vy: f32) -> Result<(), &'static str> {
        self.velocity_manager.set_velocity(entity_id, vx, vy)
    }

    pub fn get_velocity(&self, entity_id: u32) -> Result<(f32, f32), &'static str> {
        self.velocity_manager.get_velocity(entity_id)
    }

    pub fn set_zero_velocity(&self, entity_id: u32) -> Result<(), &'static str> {
        self.velocity_manager.set_zero_velocity(entity_id)
    }

    pub fn set_horizontal_velocity(&self, entity_id: u32, speed: f32) -> Result<(), &'static str> {
        self.velocity_manager
            .set_horizontal_velocity(entity_id, speed)
    }

    pub fn set_rotation_velocity(
        &self,
        entity_id: u32,
        dx: f32,
        dy: f32,
        angular: f32,
    ) -> Result<(), &'static str> {
        self.velocity_manager
            .set_rotation_velocity(entity_id, dx, dy, angular)
    }

    pub fn set_angular_velocity(&self, entity_id: u32, angular: f32) -> Result<(), &'static str> {
        self.velocity_manager
            .set_angular_velocity(entity_id, angular)
    }

    // ------------------------------------------------------------
    // Gravity Management
    // ------------------------------------------------------------
    pub fn add_downward_gravity(
        &self,
        entity_id: u32,
        force: f32,
        terminal_velocity: f32,
    ) -> Result<(), &'static str> {
        self.gravity_manager
            .add_downward_gravity(entity_id, force, terminal_velocity)
    }

    pub fn add_attractive_gravity(
        &self,
        entity_id: u32,
        force: f32,
        terminal_velocity: f32,
    ) -> Result<(), &'static str> {
        self.gravity_manager
            .add_attractive_gravity(entity_id, force, terminal_velocity)
    }

    pub fn add_repulsive_gravity(
        &self,
        entity_id: u32,
        force: f32,
        terminal_velocity: f32,
    ) -> Result<(), &'static str> {
        self.gravity_manager
            .add_repulsive_gravity(entity_id, force, terminal_velocity)
    }

    pub fn set_gravity_enabled(&self, entity_id: u32, enabled: bool) -> Result<(), &'static str> {
        self.gravity_manager.set_gravity_enabled(entity_id, enabled)
    }

    // ------------------------------------------------------------
    // Collision Management
    // ------------------------------------------------------------
    pub fn check_collision(&self, entity1: u32, entity2: u32) -> Result<bool, &'static str> {
        self.collision_manager
            .check_collision(entity1, entity2, &self.assets)
    }

    pub fn check_entity_tilemap_collision(
        &self,
        entity_id: u32,
        tilemap_id: u32,
        x: f32,
        y: f32,
    ) -> Result<bool, &'static str> {
        self.collision_manager.check_entity_tilemap_collision(
            entity_id,
            tilemap_id,
            x,
            y,
            &self.assets,
        )
    }

    // ------------------------------------------------------------
    // Tilemap Management
    // ------------------------------------------------------------
    pub fn create_tilemap(
        &self,
        entity_id: u32,
        width: u32,
        height: u32,
        tile_size: u32,
    ) -> Result<(), &'static str> {
        self.tilemap_manager
            .create_tilemap(entity_id, width, height, tile_size)
    }

    pub fn set_tile(
        &self,
        entity_id: u32,
        x: u32,
        y: u32,
        tile_id: u32,
        walkable: bool,
        color: (u8, u8, u8),
    ) -> Result<(), &'static str> {
        self.tilemap_manager
            .set_tile(entity_id, x, y, tile_id, walkable, color)
    }

    pub fn clear_tile(&self, entity_id: u32, x: u32, y: u32) -> Result<bool, &'static str> {
        self.tilemap_manager.clear_tile(entity_id, x, y)
    }

    pub fn query_tilemap(
        &self,
        entity_id: u32,
        query: TilemapQuery,
    ) -> Result<TilemapQueryResult, &'static str> {
        self.tilemap_manager.query_tilemap(entity_id, query)
    }

    pub fn is_tile_walkable(&self, entity_id: u32, x: u32, y: u32) -> Result<bool, &'static str> {
        self.tilemap_manager.is_tile_walkable(entity_id, x, y)
    }

    pub fn check_position_walkable(
        &self,
        entity_id: u32,
        tilemap_id: u32,
        x: f32,
        y: f32,
    ) -> Result<bool, &'static str> {
        self.tilemap_manager.check_position_walkable(
            entity_id,
            tilemap_id,
            x,
            y,
            &self.collision_manager,
            &self.assets,
        )
    }

    pub fn get_tilemap(&self, entity_id: u32) -> Result<Option<Tilemap>, &'static str> {
        self.tilemap_manager.get_tilemap(entity_id)
    }

    // ------------------------------------------------------------
    // Drag and Drop Management
    // ------------------------------------------------------------
    pub fn get_entity_at_point(&self, x: f32, y: f32) -> Result<Option<u32>, &'static str> {
        self.drag_drop_manager.get_entity_at_point(x, y)
    }

    pub fn start_dragging(
        &self,
        entity_id: u32,
        mouse_x: f32,
        mouse_y: f32,
    ) -> Result<(), &'static str> {
        self.drag_drop_manager
            .start_dragging(entity_id, mouse_x, mouse_y)
    }

    pub fn update_dragged_entity(&self, mouse_x: f32, mouse_y: f32) -> Result<(), &'static str> {
        self.drag_drop_manager
            .update_dragged_entity(mouse_x, mouse_y)
    }

    pub fn end_dragging(&self) -> Result<(), &'static str> {
        self.drag_drop_manager.end_dragging()
    }

    pub fn is_entity_dragged(&self, entity_id: u32) -> bool {
        self.drag_drop_manager.is_entity_dragged(entity_id)
    }

    // Input Management
    pub fn handle_key(&self, keycode: Keycode, pressed: bool) -> Result<(), &'static str> {
        self.input_manager.handle_key(keycode, pressed)
    }

    pub fn is_key_pressed(&self, keycode: Keycode) -> Result<bool, &'static str> {
        self.input_manager.is_key_pressed(keycode)
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> Result<bool, String> {
        self.input_manager.is_mouse_button_pressed(button)
    }

    // Asset Management
    pub fn get_asset(&self, name: &str) -> Option<&BuiltInAsset> {
        self.assets.get_by_name(name)
    }

    // Rendering
    pub fn render_asset(
        &self,
        asset: &BuiltInAsset,
        shape_data: Option<&SpriteShapeData>,
        x: i32,
        y: i32,
        color: (u8, u8, u8),
        renderer: &mut Sdl2Renderer,
        scale: f32,
        debug: bool,
    ) {
        match &asset.shape {
            PrimitiveShape::Rectangle { width, height } => {
                let (final_width, final_height) = if let Some(SpriteShapeData::Rectangle {
                    width: w,
                    height: h,
                }) = shape_data
                {
                    (*w, *h)
                } else {
                    (*width, *height)
                };

                renderer.draw_rect(
                    x,
                    y,
                    (final_width * scale) as u32,
                    (final_height * scale) as u32,
                    Color::RGB(color.0, color.1, color.2),
                );
            }
            PrimitiveShape::Circle { radius } => {
                // Use custom radius if provided
                let final_radius = if let Some(SpriteShapeData::Circle { radius: r }) = shape_data {
                    *r
                } else {
                    *radius
                };
                let scaled_radius = (final_radius * scale) as u32;
                renderer.draw_circle(
                    x + scaled_radius as i32,
                    y + scaled_radius as i32,
                    scaled_radius,
                    Color::RGB(color.0, color.1, color.2),
                );

                if debug {
                    let diameter = final_radius * 2.0 * scale;
                    renderer.draw_bounding_box(
                        x + (diameter as i32 / 2),
                        y + (diameter as i32 / 2),
                        diameter as u32,
                        diameter as u32,
                        Color::RGB(255, 255, 255),
                    );
                }
            }
            PrimitiveShape::Triangle {
                x1,
                y1,
                x2,
                y2,
                x3,
                y3,
            } => {
                let color = Color::RGB(color.0, color.1, color.2);
                // Use custom vertices if provided
                let vertices = if let Some(SpriteShapeData::Triangle { vertices }) = shape_data {
                    vertices
                } else {
                    &[(*x1, *y1), (*x2, *y2), (*x3, *y3)]
                };

                let sx1 = x + (vertices[0].0 * scale) as i32;
                let sy1 = y + (vertices[0].1 * scale) as i32;
                let sx2 = x + (vertices[1].0 * scale) as i32;
                let sy2 = y + (vertices[1].1 * scale) as i32;
                let sx3 = x + (vertices[2].0 * scale) as i32;
                let sy3 = y + (vertices[2].1 * scale) as i32;

                renderer.draw_triangle(sx1, sy1, sx2, sy2, sx3, sy3, color);

                if debug {
                    let min_x = sx1.min(sx2).min(sx3);
                    let max_x = sx1.max(sx2).max(sx3);
                    let min_y = sy1.min(sy2).min(sy3);
                    let max_y = sy1.max(sy2).max(sy3);

                    renderer.draw_bounding_box(
                        (min_x + max_x) / 2,
                        (min_y + max_y) / 2,
                        (max_x - min_x) as u32,
                        (max_y - min_y) as u32,
                        Color::RGB(255, 255, 255),
                    );
                }
            }
            PrimitiveShape::Line { x2, y2 } => {
                let color = Color::RGB(color.0, color.1, color.2);
                // Use custom line endpoint if provided
                let end = if let Some(SpriteShapeData::Line { end }) = shape_data {
                    end
                } else {
                    &(*x2, *y2)
                };

                let end_x = x + (end.0 * scale) as i32;
                let end_y = y + (end.1 * scale) as i32;

                renderer.draw_line(x, y, end_x, end_y, color);

                if debug {
                    let min_x = x.min(end_x);
                    let max_x = x.max(end_x);
                    let min_y = y.min(end_y);
                    let max_y = y.max(end_y);

                    renderer.draw_bounding_box(
                        (min_x + max_x) / 2,
                        (min_y + max_y) / 2,
                        (max_x - min_x) as u32,
                        (max_y - min_y) as u32,
                        Color::RGB(255, 255, 255),
                    );
                }
            }
        }
    }

    // Debug
    pub fn debug_print_entities(&self) -> Result<(), &'static str> {
        self.entity_manager.debug_print_entities(&self.assets)
    }
}
