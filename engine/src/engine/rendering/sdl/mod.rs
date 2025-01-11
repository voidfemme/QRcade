use crate::engine::rendering::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub struct Sdl2Renderer {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
}

impl Sdl2Renderer {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Sdl2Renderer {
            sdl_context,
            canvas,
        }
    }
}

impl Renderer for Sdl2Renderer {
    fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(40, 40, 40));
        self.canvas.clear();
    }

    fn present(&mut self) {
        self.canvas.present();
    }

    fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        self.canvas.set_draw_color(color);
        let rect = Rect::new(x, y, width, height);
        let _ = self.canvas.fill_rect(rect);
    }

    fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: u32, color: Color) {
        // First set the drawing color
        self.canvas.set_draw_color(color);

        // for a filled circle, we need to draw scan lines between the pairs of points
        for dy in -(radius as i32)..=radius as i32 {
            let y = center_y + dy;

            // Calculate the width of this horizontal line using the circle equation
            let delta_x = ((radius as f32).powi(2) - (dy as f32).powi(2)).sqrt() as i32;

            // Draw a horizontal line from -dx to +dx at this y coordinate
            let x1 = center_x - delta_x;
            let x2 = center_x + delta_x;

            let line = Rect::new(x1, y, (x2 - x1) as u32, 1);
            let _ = self.canvas.fill_rect(line);
        }

        // The midpoint circle algorithm works by efficiently determining which pixels
        // to draw to create a circle. It uses integer arithmetic to avoid floating point
        // calculations, making it very efficient
        let mut x: i32 = radius as i32;
        let mut y: i32 = 0;
        let mut decision = 1 - x;

        while y <= x {
            // In each iteration, we draw 8 points to complete the circle
            // This is possible due to the circle's 8-way symmetry
            for &point in &[
                (center_x + x, center_y + y),
                (center_x + y, center_y + x),
                (center_x - y, center_y + x),
                (center_x - x, center_y + y),
                (center_x - x, center_y - y),
                (center_x - y, center_y - x),
                (center_x + y, center_y - x),
                (center_x + x, center_y - y),
            ] {
                let _ = self.canvas.draw_point(point);
            }

            // Update position using the midpoint circle algorithm
            y += 1;
            if decision <= 0 {
                // The point is inside or on the circle
                decision += 2 * y + 1;
            } else {
                // The point is outside the circle
                x -= 1;
                decision += 2 * (y - x) + 1;
            }
        }
    }

    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas
            .draw_line(Point::new(x1, y1), Point::new(x2, y2))
            .unwrap();
    }

    fn draw_triangle(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        color: Color,
    ) {
        let canvas = &mut self.canvas;
        canvas.set_draw_color(color);
        canvas
            .draw_line(Point::new(x1, y1), Point::new(x2, y2))
            .unwrap();
        canvas
            .draw_line(Point::new(x2, y2), Point::new(x3, y3))
            .unwrap();
        canvas
            .draw_line(Point::new(x3, y3), Point::new(x1, y1))
            .unwrap();
    }

    fn draw_bounding_box(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        // Adjust position to align with bottom right
        let half_w = width as i32 / 2;
        let half_h = height as i32 / 2;

        // Convert box coordinates to line endpoints
        let left = x - half_w;
        let right = x + half_w;
        let top = y - half_h;
        let bottom = y + half_h;

        // Draw the four lines of the box
        self.draw_line(left, top, right, top, color); // Top
        self.draw_line(right, top, right, bottom, color); // Right
        self.draw_line(right, bottom, left, bottom, color); // Bottom
        self.draw_line(left, bottom, left, top, color); // Left
    }
}
