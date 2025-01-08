mod game_state;
mod lua_api;

use game_state::component::{GameState, Velocity};
use game_state::transform::Transform;
use mlua::{Lua, Result as LuaResult};

fn setup() {
    println!("Setting Up!");
}

fn update() {
    println!("Updating");
}

fn render() {
    println!("Rendering");
}

fn main() -> LuaResult<()> {
    // 1) Initialize Gamestate from component.rs
    let mut game_state = GameState::new();

    // 2) Create an entity and give it a Transform + Velocity
    let entity_id = game_state.create_entity();
    game_state.add_transform(entity_id, Transform::new(0.0, 0.0, 0.0));
    game_state.add_velocity(entity_id, Velocity { dx: 1.0, dy: 0.5 });

    // 3) Run your engine setup
    setup();

    // 4) Set up lua
    let lua = Lua::new();
    let rust_add_func = lua.create_function(|_, (a, b): (i32, i32)| Ok(a + b))?;
    lua.globals().set("rust_add", rust_add_func)?;

    // 5) a small Lua script that uses our rust function
    let lua_script = r#"
        print("Hello from Lua!")
        local sum = rust_add(10, 32)
        print("10 + 32 = ", sum)
    "#;
    lua.load(lua_script).exec()?;

    // 6) Main loop simulation
    let mut running = true;
    let mut frame_count = 0;
    while running {
        update();
        game_state::systems::physics::physics_system(&mut game_state);

        // Print the transform to see changes
        if let Some(t) = game_state.transforms.get(&entity_id) {
            println!(
                "Frame {}: Entity {} => Position ({:.1}, {:.1})",
                frame_count, entity_id, t.x, t.y
            );
        }
        render();
        frame_count +=1;
        if frame_count > 4 {
            // ~5 frames total
            running = false;
        }
    }

    println!("Exiting");
    Ok(())
}
