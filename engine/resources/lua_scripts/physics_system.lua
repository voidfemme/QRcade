-- Physics Playground Demo
-- This demo showcases various physics interactions including:
-- - Basic movement and velocity
-- - Gravity effects
-- - Collision responses
-- - Different types of objects with unique behaviors

-- Screen dimensions for positioning
local SCREEN_WIDTH = 800
local SCREEN_HEIGHT = 600

-- Create our objects
local objects = {
    -- Player-controlled ball that can be moved with WASD
    controlled_ball = create_entity(),
    
    -- Ball affected by downward gravity
    falling_ball = create_entity(),
    
    -- Ball that orbits around a center point
    orbiting_ball = create_entity(),
    
    -- Central attractor that pulls other objects
    attractor = create_entity(),
    
    -- Ground platform to demonstrate collisions
    ground = create_entity(),
}

-- Colors for our objects
local COLORS = {
    controlled = {0, 255, 0},   -- Green
    falling = {255, 0, 0},      -- Red
    orbiting = {0, 0, 255},     -- Blue
    attractor = {255, 255, 0},  -- Yellow
    ground = {100, 100, 100},   -- Gray
}

-- Initialize all our objects
function initialize_objects()
    -- Set up the player-controlled ball
    add_shape(objects.controlled_ball, "circle", COLORS.controlled[1], 
        COLORS.controlled[2], COLORS.controlled[3], {radius = 20})
    set_transform(objects.controlled_ball, 200, 200, 0, 1.0, 1.0)
    set_velocity(objects.controlled_ball, 0, 0)  -- Start stationary
    
    -- Set up the falling ball with gravity
    add_shape(objects.falling_ball, "circle", COLORS.falling[1], 
        COLORS.falling[2], COLORS.falling[3], {radius = 20})
    set_transform(objects.falling_ball, 400, 100, 0, 1.0, 1.0)
    set_velocity(objects.falling_ball, 50, 0)  -- Initial horizontal velocity
    add_downward_gravity(objects.falling_ball, 500, 1000)  -- Strong gravity, reasonable terminal velocity
    
    -- Set up the orbiting ball
    add_shape(objects.orbiting_ball, "circle", COLORS.orbiting[1], 
        COLORS.orbiting[2], COLORS.orbiting[3], {radius = 15})
    set_transform(objects.orbiting_ball, SCREEN_WIDTH/2 + 100, SCREEN_HEIGHT/2, 0, 1.0, 1.0)
    set_velocity(objects.orbiting_ball, 0, 200)  -- Initial vertical velocity
    
    -- Set up the central attractor
    add_shape(objects.attractor, "circle", COLORS.attractor[1], 
        COLORS.attractor[2], COLORS.attractor[3], {radius = 30})
    set_transform(objects.attractor, SCREEN_WIDTH/2, SCREEN_HEIGHT/2, 0, 1.0, 1.0)
    add_attractive_gravity(objects.attractor, 50000, 1000)  -- Strong attraction with reasonable terminal velocity
    
    -- Set up the ground platform
    add_shape(objects.ground, "rectangle", COLORS.ground[1], 
        COLORS.ground[2], COLORS.ground[3], 
        {width = SCREEN_WIDTH, height = 40})
    set_transform(objects.ground, SCREEN_WIDTH/2, SCREEN_HEIGHT - 20, 0, 1.0, 1.0)
end

-- Movement speed for the controlled ball
local CONTROL_SPEED = 200

-- Called when the game starts
function on_start()
    initialize_objects()
    print("Physics Playground Controls:")
    print("- WASD: Move the green ball")
    print("- Spacebar: Reset the red ball position")
    print("- Watch how the blue ball orbits the yellow attractor!")
end

-- Handle bouncing off screen edges
function handle_screen_bounds(entity)
    local x, y = get_transform(entity)
    local vx, vy = get_velocity(entity)
    local bounce_damping = 0.8  -- Reduce velocity slightly on bounce
    
    -- Bounce off screen edges
    if x < 20 or x > SCREEN_WIDTH - 20 then
        set_velocity(entity, -vx * bounce_damping, vy)
        -- Keep in bounds
        if x < 20 then
            set_transform(entity, 20, y, 0, 1.0, 1.0)
        else
            set_transform(entity, SCREEN_WIDTH - 20, y, 0, 1.0, 1.0)
        end
    end
    
    -- Bounce off top
    if y < 20 then
        set_velocity(entity, vx, -vy * bounce_damping)
        set_transform(entity, x, 20, 0, 1.0, 1.0)
    end
    
    -- Bounce off ground
    if y > SCREEN_HEIGHT - 60 then  -- Account for ground height
        set_velocity(entity, vx * 0.95, -vy * bounce_damping)  -- Extra horizontal damping on ground
        set_transform(entity, x, SCREEN_HEIGHT - 60, 0, 1.0, 1.0)
    end
end

-- Update function called every frame
function on_frame(delta_time)
    -- Handle controlled ball movement
    local vx, vy = 0, 0
    if is_key_pressed("W") then vy = -CONTROL_SPEED end
    if is_key_pressed("S") then vy = CONTROL_SPEED end
    if is_key_pressed("A") then vx = -CONTROL_SPEED end
    if is_key_pressed("D") then vx = CONTROL_SPEED end
    set_velocity(objects.controlled_ball, vx, vy)
    
    -- Reset falling ball when spacebar is pressed
    if is_key_pressed("SPACE") then
        set_transform(objects.falling_ball, 400, 100, 0, 1.0, 1.0)
        set_velocity(objects.falling_ball, 50, 0)
    end
    
    -- Handle bouncing off screen edges for all dynamic objects
    handle_screen_bounds(objects.controlled_ball)
    handle_screen_bounds(objects.falling_ball)
    handle_screen_bounds(objects.orbiting_ball)
end
