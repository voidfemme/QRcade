mod game_state;
mod lua_api;

use game_state::component::{GameState, Velocity};
use game_state::systems::movement_system::movement_system;
use game_state::systems::rendering;
use game_state::transform::Transform;
// use mlua::{Lua, Result as LuaResult};
use rendering::{render_system, Sdl2Renderer};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn setup(state: &mut GameState) {
    println!("Setting Up!");

    // Create an entity and give it a Transform + Velocity
    let entity_id = state.create_entity();
    state.add_transform(entity_id, Transform::new(100.0, 100.0, 0.0, 0.0, 1.0));
    state.add_velocity(entity_id, Velocity::new(2.0, 1.0));

    println!("Setup complete!");
}

fn update(state: &mut GameState) {
    // Update the game state (move entities)
    movement_system(state);
}

fn render(state: &mut GameState, renderer: &mut Sdl2Renderer) {
    renderer.clear();
    render_system(state, &mut renderer.canvas, 1.0);
    renderer.present();
}

fn main() {
    // Initialize Gamestate from component.rs
    let mut game_state = GameState::new();

    setup(&mut game_state);

    // Create the SDL2Renderer
    let mut renderer = rendering::Sdl2Renderer::new("My Engine", 800, 600);

    // Create an SDL event pump for handling input
    let sdl_context = &renderer.sdl_context;
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Run your engine setup
    // setup();

    // // Set up lua
    // let lua = Lua::new();
    // let rust_add_func = lua.create_function(|_, (a, b): (i32, i32)| Ok(a + b))?;
    // lua.globals().set("rust_add", rust_add_func)?;
    //
    // // a small Lua script that uses our rust function
    // let lua_script = r#"
    //     print("Hello from Lua!")
    //     local sum = rust_add(10, 32)
    //     print("10 + 32 = ", sum)
    // "#;
    // lua.load(lua_script).exec()?;

    // Main loop simulation
    let mut running = true;
    while running {
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
        update(&mut game_state);

        // Render
        render(&mut game_state, &mut renderer);
    }

    println!("Exiting");
}
