-- Global variables for triangle physics and movement
local triangle
local triangle_x, triangle_y
local triangle_velocity_y = 0
local gravity = 900
local jump_strength = 400
local move_speed = 200

-- Screen dimensions
local screen_width = 800
local screen_height = 600

-- Entities for the platform and triangle
local ground

function on_start()
    -- Create a ground platform as a full-width rectangle at the bottom
    ground = create_entity()
    -- Position the ground at the bottom-left of the screen
    set_transform(ground, 0, screen_height - 40, 0, 1.0, 1.0)
    -- Create a rectangle shape as wide as the screen with a height of 40
    add_shape(ground, "rectangle", 100, 100, 100, { width = screen_width, height = 40 })

    -- Create a triangle above the ground
    triangle = create_entity()
    triangle_x = screen_width / 2
    triangle_y = screen_height / 2
    triangle_velocity_y = 0
    set_transform(triangle, triangle_x, triangle_y, 0, 1.0, 1.0)
    add_shape(triangle, "triangle", 0, 255, 0, {
        x1 = 0,   y1 = -16,
        x2 = -16, y2 = 16,
        x3 = 16,  y3 = 16
    })
end

function on_frame(delta_time)
    -- Horizontal movement using A and D
    local horizontal_velocity = 0
    if is_key_pressed("A") then
        horizontal_velocity = horizontal_velocity - move_speed
    end
    if is_key_pressed("D") then
        horizontal_velocity = horizontal_velocity + move_speed
    end
    triangle_x = triangle_x + horizontal_velocity * delta_time

    -- Retrieve ground's y-coordinate for collision and jump checks
    local _, ground_y, _ = get_transform(ground)
    -- Assume half the triangle's height is 16 (based on its vertices)
    local triangle_half_height = 16

    -- Jump logic: spacebar initiates jump if triangle is on or very near the ground
    if is_key_pressed("SPACE") and (triangle_y + triangle_half_height >= ground_y) then
        triangle_velocity_y = -jump_strength
    end

    -- Apply gravity to the triangle's vertical velocity
    triangle_velocity_y = triangle_velocity_y + gravity * delta_time

    -- Update the triangle's vertical position based on its velocity
    triangle_y = triangle_y + triangle_velocity_y * delta_time

    -- Update the triangle's transform with new position
    set_transform(triangle, triangle_x, triangle_y, 0, 1.0, 1.0)

    -- Simple collision check with the ground:
    -- If the bottom of the triangle reaches or goes below the top of the ground
    if triangle_y + triangle_half_height >= ground_y then
        -- Place the triangle on top of the ground and reset its vertical velocity
        triangle_y = ground_y - triangle_half_height
        triangle_velocity_y = 0
        set_transform(triangle, triangle_x, triangle_y, 0, 1.0, 1.0)
    end
end

function on_end()
    print("Script ended.")
end

