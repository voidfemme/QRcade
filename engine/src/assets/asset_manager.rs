use crate::engine::rendering::{Renderer, Sdl2Renderer};
use sdl2::pixels::Color;

// Define the different types of primitive shapes the engine supports
#[derive(Debug)]
pub enum PrimitiveShape {
    Rectangle { width: f32, height: f32 },
    Circle { radius: f32 },
    // Add more shapes like Triangle, Line, etc. later
}

// A built-in asset that combines a shape with visual properties
pub struct BuiltInAsset {
    pub shape: PrimitiveShape,
    color: Color,
    name: &'static str, // A human-readable shape with visual properties
    id: u32,            // A unique numeric identifier
}

pub struct AssetManager {
    // Our catalog of built-in assets
    assets: Vec<BuiltInAsset>,
}

impl AssetManager {
    pub fn new() -> Self {
        // Create our initial catalog of assets
        let assets = vec![
            // Add some basic shapes with different colors
            BuiltInAsset {
                shape: PrimitiveShape::Rectangle {
                    width: 32.0,
                    height: 32.0,
                },
                color: Color::RGB(255, 255, 255), // White square
                name: "square",
                id: 1,
            },
            BuiltInAsset {
                shape: PrimitiveShape::Circle { radius: 16.0 },
                color: Color::RGB(255, 255, 255), // White circle
                name: "circle",
                id: 1,
            },
        ];
        println!("Initialized AssetManager with assets:");
        for asset in &assets {
            println!("  - Asset '{}' (ID: {})", asset.name, asset.id);
        }

        Self { assets }
    }

    pub fn get_by_name(&self, name: &str) -> Option<&BuiltInAsset> {
        // Look through our vector of assets for one with a matching name
        self.assets.iter().find(|asset| asset.name == name)
    }

    pub fn render_asset(&self, asset: &BuiltInAsset, x: i32, y: i32, renderer: &mut Sdl2Renderer) {
        match asset.shape {
            PrimitiveShape::Rectangle { width, height } => {
                renderer.draw_rect(x, y, width as u32, height as u32, asset.color);
            }
            PrimitiveShape::Circle { radius } => {
                renderer.draw_circle(x, y, radius as u32, asset.color);
            }
        }
    }
}
