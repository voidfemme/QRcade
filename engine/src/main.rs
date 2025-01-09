mod game_state;
mod lua_api;

use game_state::component::{GameState, Velocity};
use game_state::systems::movement_system::movement_system;
use game_state::systems::rendering;
use game_state::transform::Transform;
use lua_api::entity_api::register_entity_api;
use lua_api::renderable_api::register_renderable_api;
use lua_api::transform_api::register_transform_api;
use lua_api::{call_on_end, call_on_frame, call_on_start};
use mlua::{Lua, Result as LuaResult};
use rendering::{render_system, Sdl2Renderer};
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

fn setup(state: &mut GameState, lua: &Lua) -> Result<(), mlua::Error> {
    // Create a test entity with basic components
    let entity_id = state.create_entity();
    state.add_transform(entity_id, Transform::new(400.0, 300.0, 0.0, 0.0, 1.0));
    state.add_velocity(entity_id, Velocity::new(50.0, 30.0));

    // Load and run the initialization Lua script
    load_lua_script(lua, "resources/lua_scripts/example_script.lua")?;

    // Call the Lua on_start function if it exists
    call_on_start(lua)?;
    Ok(())
}

fn update(state: &mut GameState, lua: &Lua, delta_time: f32) -> Result<(), mlua::Error> {
    // Call Lua update function
    call_on_frame(lua, delta_time)?;

    // Update physics/movement
    movement_system(state);
    Ok(())
}

fn render(state: &mut GameState, renderer: &mut Sdl2Renderer) {
    renderer.clear();
    render_system(state, &mut renderer.canvas, 1.0);
    renderer.present();
}

fn main() -> LuaResult<()> {
    // Initialize Gamestate from component.rs
    let mut gamestate = GameState::new();

    // Create the SDL2Renderer
    let mut renderer = rendering::Sdl2Renderer::new("My Engine", 800, 600);

    // Create an SDL event pump for handling input
    let sdl_context = &renderer.sdl_context;
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Set up lua
    let lua = Lua::new();
    let gamestate_rc = Rc::new(RefCell::new(gamestate.clone()));
    register_entity_api(&lua, Rc::clone(&gamestate_rc))?;
    register_transform_api(&lua, Rc::clone(&gamestate_rc))?;
    register_renderable_api(&lua, Rc::clone(&gamestate_rc))?;

    // Run setup
    setup(&mut gamestate, &lua)?;

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

                _ => {}
            }
        }

        // Update game state
        update(&mut gamestate, &lua, delta_time)?;

        // Render
        render(&mut gamestate, &mut renderer);
    }

    call_on_end(&lua)?;
    Ok(())
}
