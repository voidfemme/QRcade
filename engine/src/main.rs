mod assets;
mod ecs;
mod engine;
mod lua;

use ecs::{render_system, DragDropSystem, GameState, InputSystem, MovementSystem, PhysicsSystem};
use engine::managers::state_manager::StateManager;
use engine::rendering::{Renderer, Sdl2Renderer};
use lua::{
    call_on_end, call_on_frame, call_on_start, register_collision_api, register_drag_drop_api,
    register_entity_api, register_gravity_api, register_input_api, register_renderable_api,
    register_text_api, register_tilemap_api, register_transform_api, register_velocity_api,
};

use mlua::{Lua, Result as LuaResult};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use tracing::{debug, error, info, trace};

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
            window_title: "QRcade",
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

fn update(
    state_manager: Rc<StateManager>,
    movement_system: &MovementSystem,
    physics_system: &mut PhysicsSystem,
    drag_drop_system: &mut DragDropSystem,
    input_system: Rc<RefCell<InputSystem>>,
    lua: &Lua,
    delta_time: f32,
) -> Result<(), mlua::Error> {
    // Get mouse position and pass it to Lua if needed
    let mouse_position = {
        let input = input_system.borrow();
        input.get_mouse_position()
    };
    let (mouse_x, mouse_y) = mouse_position;

    trace!(mouse_x, mouse_y, "Mouse position updated");

    debug!("Update loop - Mouse position: ({}, {})", mouse_x, mouse_y);

    // expose the mouse position to Lua scripts
    lua.globals().set("mouse_x", mouse_x as f32)?;
    lua.globals().set("mouse_y", mouse_y as f32)?;

    // Call Lua update function first to get any new movement commands
    call_on_frame(lua, delta_time)?;

    // update drag and drop system - THIS IS CRUCIAL
    if let Ok(mut state) = state_manager.state.try_borrow_mut() {
        debug!("Updating drag drop system");
        drag_drop_system.update(&input_system.borrow(), &mut state);
    } else {
        error!("Failed to borrow state for drag drop system update");
    }

    // Process movement commands
    movement_system.update(delta_time);

    // Update physics simulation
    if let Ok(mut state) = state_manager.state.try_borrow_mut() {
        physics_system.update(&mut state, delta_time);
    }

    Ok(())
}

fn render(state_manager: Rc<StateManager>, renderer: &mut Sdl2Renderer, debug: bool) {
    tracing::debug!("Starting render frame");
    renderer.clear();
    render_system(state_manager, renderer, 1.0, debug);
    renderer.present();
    tracing::debug!("Completed render frame");
}

fn main() -> LuaResult<()> {
    // Initialize tracing subscriber with a specific filter level
    tracing_subscriber::fmt()
        .with_env_filter("info,qrcade=debug")
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

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

    info!("Loading script: {}", config.script_path);

    // Initialize Gamestate with debug mode
    let input_system = Rc::new(RefCell::new(InputSystem::new()));
    let gamestate_rc = Rc::new(RefCell::new(GameState::new()));

    // create StateManager with reference to InputSystem
    let state_manager = Rc::new(StateManager::new(
        Rc::clone(&gamestate_rc),
        Rc::clone(&input_system),
    ));

    let movement_system = MovementSystem::new(Rc::clone(&state_manager));
    let mut physics_system = PhysicsSystem::new();
    let mut drag_drop_system = DragDropSystem::new();

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
    register_drag_drop_api(&lua, Rc::clone(&state_manager))?;
    register_text_api(&lua, Rc::clone(&state_manager))?;

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

    // Set up time tracking
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
                    input_system.borrow_mut().set_key_pressed(code);
                }
                Event::KeyUp {
                    keycode: Some(code),
                    ..
                } => {
                    input_system.borrow_mut().set_key_released(code);
                }
                Event::MouseMotion { x, y, .. } => {
                    debug!("Mouse moved to: ({}, {})", x, y);
                    // this will update both position and any active drag operation
                    input_system.borrow_mut().update_mouse_position(x, y);

                    // If we're dragging an entity, update its position through StateManager
                    state_manager
                        .update_dragged_entity(x as f32, y as f32)
                        .unwrap_or_else(|e| error!("Error updating dragged entity: {}", e));
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    debug!("Mouse button down: {:?}", mouse_btn);
                    input_system
                        .borrow_mut()
                        .set_mouse_button_pressed(mouse_btn);

                    // If it's the left mouse button, we might start dragging
                    if mouse_btn == MouseButton::Left {
                        let (x, y) = input_system.borrow().get_mouse_position();

                        // Try to find an entity under the mouse when we first click
                        if let Ok(state) = state_manager.state.try_borrow() {
                            if let Some(entity_id) =
                                drag_drop_system.find_entity_under_mouse(&state, x as f32, y as f32)
                            {
                                // If we found an entity and we have its transform, start dragging
                                if let Some(transform) = state.transforms.get(&entity_id) {
                                    drag_drop_system.start_drag(
                                        entity_id,
                                        x as f32,
                                        y as f32,
                                        transform.x,
                                        transform.y,
                                    );
                                }
                            }
                        }
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    debug!("Mouse button up: {:?}", mouse_btn);
                    input_system
                        .borrow_mut()
                        .set_mouse_button_released(mouse_btn);

                    // If it was the left button, end any drag operation
                    if mouse_btn == MouseButton::Left {
                        drag_drop_system.end_drag();
                    }
                }
                _ => {}
            }
        }

        // Update game state
        {
            update(
                Rc::clone(&state_manager),
                &movement_system,
                &mut physics_system,
                &mut drag_drop_system,
                input_system.clone(),
                &lua,
                delta_time,
            )?;
        }

        // Render
        {
            render(Rc::clone(&state_manager), &mut renderer, config.debug_mode);
        }
    }

    call_on_end(&lua)?;
    Ok(())
}
