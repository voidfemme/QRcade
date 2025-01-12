-- Define global variables
local player
local player_x, player_y
local player_velocity_x, player_velocity_y
local speed = 200          -- Horizontal movement speed (pixels per second)
local jump_strength = 400  -- Initial jump velocity
local gravity = 900        -- Gravity acceleration (pixels per second squared)
local is_on_ground = false

local platform            -- Single platform entity
local screen_width = 800
local screen_height = 600

-- Function to create a platform
function create_platform(x, y, width, height)
    local plat = create_entity()
    set_transform(plat, x, y, 0, 1.0, 1.0)
    add_shape(plat, "square", 100, 100, 100)  -- Grey platform
    -- Resize the platform to the specified width and height
    set_transform(plat, x, y, 0, width / 100, height / 100)
    return plat
end

function on_start()
    -- Initialize player as a green triangle
    player = create_entity()
    player_x = 100
    player_y = 500
    player_velocity_x = 0
    player_velocity_y = 0
    set_transform(player, player_x, player_y, 0, 1.0, 1.0)
    add_shape(player, "triangle", 0, 255, 0)  -- Green triangle

    -- Create one big ground platform
    platform = create_platform(400, 580, 800, 40)
end

function on_frame(delta_time)
    -- Handle horizontal input
    if is_key_pressed("LEFT") then
        player_velocity_x = -speed
    elseif is_key_pressed("RIGHT") then
        player_velocity_x = speed
    else
        player_velocity_x = 0
    end

    -- Handle jumping
    if is_key_pressed("SPACE") and is_on_ground then
        player_velocity_y = -jump_strength
        is_on_ground = false
    end

    -- Apply gravity
    player_velocity_y = player_velocity_y + gravity * delta_time

    -- Update player position
    player_x = player_x + player_velocity_x * delta_time
    player_y = player_y + player_velocity_y * delta_time
    set_transform(player, player_x, player_y, 0, 1.0, 1.0)

    -- Collision detection with the single platform
    is_on_ground = false
    if is_colliding(player, platform) then
        -- Simple collision resolution: place player on top of the platform
        local platform_x, platform_y = get_transform(platform)
        player_y = platform_y - 20  -- Adjust based on player's height
        player_velocity_y = 0
        is_on_ground = true
        set_transform(player, player_x, player_y, 0, 1.0, 1.0)
    end

    -- Prevent player from moving out of screen bounds horizontally
    if player_x < 0 then
        player_x = 0
        set_transform(player, player_x, player_y, 0, 1.0, 1.0)
    elseif player_x > screen_width then
        player_x = screen_width
        set_transform(player, player_x, player_y, 0, 1.0, 1.0)
    end

    -- Reset player if they fall below the screen
    if player_y > screen_height then
        player_x = 100
        player_y = 500
        player_velocity_y = 0
        set_transform(player, player_x, player_y, 0, 1.0, 1.0)
    end
end

function on_end()
    print("Platformer game ended")
end

