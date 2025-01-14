mod assets;
mod ecs;
mod engine;
mod lua;

use ecs::{movement_system, render_system, GameState};
use engine::rendering::{Renderer, Sdl2Renderer};
use lua::{
    call_on_end, call_on_frame, call_on_start, register_collision_api, register_entity_api,
    register_gravity_api, register_input_api, register_renderable_api, register_tilemap_api,
    register_transform_api, register_velocity_api, StateManager,
};

use mlua::{Lua, Result as LuaResult};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::path::Path;
use std::rc::Rc;

struct EngineConfig {
    debug_mode: bool,
    window_width: u32,
    window_height: u32,
    window_title: &'static str,
    script_path: String,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            debug_mode: false,
            window_width: 800,
            window_height: 600,
            window_title: "My Engine",
            script_path: "resources/lua_scripts/example_script.lua".to_string(),
        }
    }
}

fn load_lua_script(lua: &Lua, filepath: &str) -> Result<(), mlua::Error> {
    // Verify the script file exists
    if !Path::new(filepath).exists() {
        return Err(mlua::Error::external(format!(
            "Script file not found: {}",
            filepath
        )));
    }
    let script_content = fs::read(filepath)
        .map_err(|e| mlua::Error::external(format!("Failed to read file: {}", e)))?;

    lua.load(&script_content).exec()?;
    Ok(())
}

fn setup(
    _state_manager: Rc<StateManager>,
    lua: &Lua,
    script_path: &str,
) -> Result<(), mlua::Error> {
    // Load and run the initialization Lua script
    load_lua_script(lua, script_path)?;

    // Call the Lua on_start function if it exists
    call_on_start(lua)?;
    Ok(())
}

fn update(state_manager: Rc<StateManager>, lua: &Lua, delta_time: f32) -> Result<(), mlua::Error> {
    // Call Lua update function
    call_on_frame(lua, delta_time)?;

    // Update physics/movement
    movement_system(state_manager, delta_time);
    Ok(())
}

fn render(state_manager: Rc<StateManager>, renderer: &mut Sdl2Renderer, debug: bool) {
    renderer.clear();
    render_system(state_manager, renderer, 1.0, debug);
    renderer.present();
}

fn main() -> LuaResult<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    // Create engine configuration
    let mut config = EngineConfig {
        debug_mode: std::env::var("DEBUG").is_ok(),
        ..Default::default()
    };

    // If a script path is provided as an argument, use it
    if args.len() > 1 {
        config.script_path = args[1].clone();
    }

    println!("Loading script: {}", config.script_path);

    // Initialize Gamestate with debug mode
    let gamestate_rc = Rc::new(RefCell::new(GameState::new()));
    let state_manager = Rc::new(StateManager::new(Rc::clone(&gamestate_rc)));

    // Create the renderer with configurations
    let mut renderer = Sdl2Renderer::new(
        config.window_title,
        config.window_width,
        config.window_height,
    );

    // Create an SDL event pump for handling input
    let sdl_context = &renderer.sdl_context;
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Set up lua environment
    let lua = Lua::new();
    register_entity_api(&lua, Rc::clone(&state_manager))?;
    register_transform_api(&lua, Rc::clone(&state_manager))?;
    register_renderable_api(&lua, Rc::clone(&state_manager))?;
    register_input_api(&lua, Rc::clone(&state_manager))?;
    register_collision_api(&lua, Rc::clone(&state_manager))?;
    register_tilemap_api(&lua, Rc::clone(&state_manager))?;
    register_velocity_api(&lua, Rc::clone(&state_manager))?;
    register_gravity_api(&lua, Rc::clone(&state_manager))?;

    // Run setup
    match setup(Rc::clone(&state_manager), &lua, &config.script_path) {
        Ok(_) => {
            if config.debug_mode {
                state_manager.debug_print_entities().unwrap();
            }
        }
        Err(e) => {
            eprintln!("Failed to load script '{}': {}", config.script_path, e);
            return Err(e);
        }
    }

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
            render(Rc::clone(&state_manager), &mut renderer, config.debug_mode);
        }
    }

    call_on_end(&lua)?;
    Ok(())
}
