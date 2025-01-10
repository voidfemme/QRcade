mod ecs;
mod engine;
mod lua_api;

use crate::ecs::systems::rendering::render_system;
use crate::engine::rendering::Renderer;
use ecs::components::component::GameState;
use ecs::systems::movement_system::movement_system;
use engine::rendering::Sdl2Renderer;
use lua_api::collision_api::register_collision_api;
use lua_api::entity_api::register_entity_api;
use lua_api::input_api::register_input_api;
use lua_api::renderable_api::register_renderable_api;
use lua_api::state_manager::StateManager;
use lua_api::transform_api::register_transform_api;
use lua_api::{call_on_end, call_on_frame, call_on_start};
use mlua::{Lua, Result as LuaResult};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

fn load_lua_script(lua: &Lua, filepath: &str) -> Result<(), mlua::Error> {
    let script_content = fs::read(filepath)
        .map_err(|e| mlua::Error::external(format!("Failed to read file: {}", e)))?;

    lua.load(&script_content).exec()?;
    Ok(())
}

fn setup(_state_manager: Rc<StateManager>, lua: &Lua) -> Result<(), mlua::Error> {
    // Load and run the initialization Lua script
    load_lua_script(lua, "resources/lua_scripts/example_script.lua")?;

    // Call the Lua on_start function if it exists
    call_on_start(lua)?;
    Ok(())
}

fn update(state_manager: Rc<StateManager>, lua: &Lua, delta_time: f32) -> Result<(), mlua::Error> {
    // Call Lua update function
    call_on_frame(lua, delta_time)?;

    // Update physics/movement
    movement_system(state_manager);
    Ok(())
}

fn render(state_manager: Rc<StateManager>, renderer: &mut Sdl2Renderer) {
    renderer.clear();
    render_system(state_manager, renderer, 1.0);
    renderer.present();
}

fn main() -> LuaResult<()> {
    // Initialize Gamestate from component.rs
    let gamestate_rc = Rc::new(RefCell::new(GameState::new()));

    // Create a single StateManager instance
    let state_manager = Rc::new(StateManager::new(Rc::clone(&gamestate_rc)));

    // Create the SDL2Renderer
    let mut renderer = Sdl2Renderer::new("My Engine", 800, 600);

    // Create an SDL event pump for handling input
    let sdl_context = &renderer.sdl_context;
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Set up lua
    let lua = Lua::new();
    register_entity_api(&lua, Rc::clone(&state_manager))?;
    register_transform_api(&lua, Rc::clone(&state_manager))?;
    register_renderable_api(&lua, Rc::clone(&state_manager))?;
    register_input_api(&lua, Rc::clone(&state_manager))?;
    register_collision_api(&lua, Rc::clone(&state_manager))?;

    // Run setup
    setup(Rc::clone(&state_manager), &lua)?;
    state_manager.debug_print_entities().unwrap();

    // Set up time
    let mut last_time = std::time::Instant::now();

    // Main loop simulation
    let mut running = true;
    while running {
        let current_time = std::time::Instant::now();
        let delta_time = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => running = false, // Handle window close
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => running = false, // Handle Escape key
                Event::KeyDown {
                    keycode: Some(code),
                    ..
                } => {
                    state_manager
                        .handle_key(code, true)
                        .unwrap_or_else(|e| println!("Error handling keydown: {}", e));
                }
                Event::KeyUp {
                    keycode: Some(code),
                    ..
                } => {
                    state_manager
                        .handle_key(code, false)
                        .unwrap_or_else(|e| println!("Error handling keyup: {}", e));
                }
                _ => {}
            }
        }

        // Update game state
        {
            update(Rc::clone(&state_manager), &lua, delta_time)?;
        }

        // Render
        {
            render(Rc::clone(&state_manager), &mut renderer);
        }
    }

    call_on_end(&lua)?;
    Ok(())
}
