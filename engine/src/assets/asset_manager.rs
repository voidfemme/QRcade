use crate::engine::rendering::{Renderer, Sdl2Renderer};
use sdl2::pixels::Color;

// Define the different types of primitive shapes the engine supports
#[derive(Debug)]
pub enum PrimitiveShape {
    Rectangle {
        width: f32,
        height: f32,
    },
    Circle {
        radius: f32,
    },
    // A triangle is defined by three points relative to its position
    Triangle {
        x1: f32,
        y1: f32, // First vertex relative to position
        x2: f32,
        y2: f32, // Second vertex relative to position
        x3: f32,
        y3: f32, // Third vertex relative to position
    },
    // A line is defined by its start and end points relative to its position
    Line {
        x2: f32,
        y2: f32, // End point relative to start position
    },
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
                id: 2,
            },
            BuiltInAsset {
                shape: PrimitiveShape::Triangle {
                    x1: 0.0,
                    y1: -16.0,
                    x2: -16.0,
                    y2: 16.0,
                    x3: 16.0,
                    y3: 16.0,
                },
                color: Color::RGB(255, 255, 255),
                name: "triangle",
                id: 3,
            },
            BuiltInAsset {
                shape: PrimitiveShape::Line { x2: 32.0, y2: 32.0 },
                color: Color::RGB(255, 255, 255),
                name: "line",
                id: 4,
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
            PrimitiveShape::Triangle {
                x1,
                y1,
                x2,
                y2,
                x3,
                y3,
            } => {
                let ax = x + x1 as i32;
                let ay = y + y1 as i32;
                let bx = x + x2 as i32;
                let by = y + y2 as i32;
                let cx = x + x3 as i32;
                let cy = y + y3 as i32;
                renderer.draw_triangle(ax, ay, bx, by, cx, cy, asset.color);
            }
            PrimitiveShape::Line { x2, y2 } => {
                let end_x = x + x2 as i32;
                let end_y = y + y2 as i32;
                renderer.draw_line(x, y, end_x, end_y, asset.color);
            }
        }
    }
}
