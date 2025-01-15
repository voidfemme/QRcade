-- Screen dimensions
local screen_width = 800
local screen_height = 600

-- Entities
local ground
local falling_ball    -- This ball will have gravity
local floating_ball   -- This ball will float freely

-- Debug counter to limit prints
local debug_counter = 0

function on_start()
    -- Create ground platform
    ground = create_entity()
    set_transform(ground, screen_width/2, screen_height - 20, 0, 1.0, 1.0)
    add_shape(ground, "rectangle", 100, 100, 100, { width = screen_width, height = 40 })

    -- Create ball that falls with gravity (red)
    falling_ball = create_entity()
    set_transform(falling_ball, screen_width/3, 100, 0, 1.0, 1.0)
    add_shape(falling_ball, "circle", 255, 0, 0, { radius = 20 })
    set_velocity(falling_ball, 0, 0)  -- Initialize velocity component first
    add_downward_gravity(falling_ball, 1, 10)  -- Gentle gravity
    set_horizontal_velocity(falling_ball, 0.1)  -- Set horizontal movement

    -- Create ball that floats (blue)
    floating_ball = create_entity()
    set_transform(floating_ball, 2*screen_width/3, 100, 0, 1.0, 1.0)
    add_shape(floating_ball, "circle", 0, 0, 255, { radius = 20 })
    set_velocity(floating_ball, 0, 0)  -- Initialize velocity component first
    set_horizontal_velocity(floating_ball, -0.1)  -- Set horizontal movement

    print("Initial velocities set to 0.1 pixels per second")
    print("Initial gravity set to 1 unit")
end

function on_frame(delta_time)
    -- Print debug info occasionally
    debug_counter = debug_counter + 1
    if debug_counter % 60 == 0 then  -- Print every 60 frames
        print(string.format("Delta time: %.6f", delta_time))
        local fvx, fvy = get_velocity(falling_ball)
        print(string.format("Red ball velocity: %.2f, %.2f", fvx, fvy))
    end

    -- Handle bouncing for falling ball
    local fx, fy, _ = get_transform(falling_ball)
    local _, ground_y, _ = get_transform(ground)
    local fvx, fvy = get_velocity(falling_ball)
    
    -- Bounce off ground
    if fy + 20 > ground_y - 20 then
        set_transform(falling_ball, fx, ground_y - 40, 0, 1.0, 1.0)
        set_velocity(falling_ball, fvx * 0.8, -fvy * 0.3)
    end
    
    -- Bounce off walls for falling ball
    if fx - 20 < 0 or fx + 20 > screen_width then
        set_horizontal_velocity(falling_ball, -fvx)  -- Just reverse horizontal direction
    end

    -- Handle bouncing for floating ball
    local flx, fly, _ = get_transform(floating_ball)
    local flvx, flvy = get_velocity(floating_ball)
    
    if flx - 20 < 0 or flx + 20 > screen_width then
        set_horizontal_velocity(floating_ball, -flvx)  -- Just reverse horizontal direction
    end
end
