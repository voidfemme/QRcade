-- Constants for our game
local SCREEN_WIDTH = 800
local SCREEN_HEIGHT = 600
local WALL_THICKNESS = 20
local GRAVITY_FORCE = 200
local TERMINAL_VELOCITY = 300
local BOUNCE_DAMPING = 0.8
local COLLISION_RESPONSE = 5  -- Added constant for collision response strength

-- Colors
local WALL_COLOR = {128, 128, 128}  -- Gray
local BALL_COLORS = {
    {0, 255, 0},    -- Green
    {255, 0, 0},    -- Red
    {0, 0, 255},    -- Blue
    {255, 255, 0},  -- Yellow
    {255, 0, 255}   -- Purple
}

-- Store our entities
local walls = {}
local balls = {}

-- Helper function to handle collision response
local function handle_collision(ball, wall, vx, vy)
    local ball_x, ball_y = get_transform(ball)
    local wall_x, wall_y = get_transform(wall)
    
    -- Determine if this is a vertical or horizontal wall
    local is_vertical = wall == walls[3] or wall == walls[4]  -- Left or right walls
    
    -- Apply bounce velocity
    if is_vertical then
        set_velocity(ball, vx * -BOUNCE_DAMPING, vy)
        -- Push ball away from wall more strongly
        local push_direction = ball_x < wall_x and -COLLISION_RESPONSE or COLLISION_RESPONSE
        set_transform(ball, ball_x + push_direction, ball_y, 0, 1, 1)
    else
        set_velocity(ball, vx, vy * -BOUNCE_DAMPING)
        -- Push ball away from wall more strongly
        local push_direction = ball_y < wall_y and -COLLISION_RESPONSE or COLLISION_RESPONSE
        set_transform(ball, ball_x, ball_y + push_direction, 0, 1, 1)
    end
end

-- Helper function to create a wall
local function create_wall(x, y, width, height)
    local wall = create_entity()
    set_transform(wall, x, y, 0, 1, 1)
    add_shape(wall, "rectangle", WALL_COLOR[1], WALL_COLOR[2], WALL_COLOR[3], 
        {width = width, height = height})
    table.insert(walls, wall)
    return wall
end

-- Helper function to create a ball
local function create_ball(x, y)
    local ball = create_entity()
    local color = BALL_COLORS[math.random(#BALL_COLORS)]
    set_transform(ball, x, y, 0, 1, 1)
    add_shape(ball, "circle", color[1], color[2], color[3], {radius = 20})
    -- Initialize with zero velocity - needed for gravity to work
    set_velocity(ball, 0, 0)
    add_downward_gravity(ball, GRAVITY_FORCE, TERMINAL_VELOCITY)
    -- Make the ball draggable using our new API
    make_entity_draggable(ball)
    table.insert(balls, ball)
    return ball
end

function on_start()
    -- Create walls around the screen
    -- Bottom wall (stored at index 1)
    create_wall(SCREEN_WIDTH/2, SCREEN_HEIGHT - WALL_THICKNESS/2, 
        SCREEN_WIDTH, WALL_THICKNESS)
    -- Top wall (stored at index 2)
    create_wall(SCREEN_WIDTH/2, WALL_THICKNESS/2, 
        SCREEN_WIDTH, WALL_THICKNESS)
    -- Left wall (stored at index 3)
    create_wall(WALL_THICKNESS/2, SCREEN_HEIGHT/2, 
        WALL_THICKNESS, SCREEN_HEIGHT)
    -- Right wall (stored at index 4)
    create_wall(SCREEN_WIDTH - WALL_THICKNESS/2, SCREEN_HEIGHT/2, 
        WALL_THICKNESS, SCREEN_HEIGHT)
    
    -- Create some balls at random positions
    for i = 1, 5 do
        local x = WALL_THICKNESS + math.random(SCREEN_WIDTH - 2*WALL_THICKNESS)
        local y = WALL_THICKNESS + math.random(SCREEN_HEIGHT/2)  -- Start in top half
        create_ball(x, y)
    end
    
    print("Click and drag the colored balls! Watch them bounce!")
end

function on_frame(delta_time)
    local mouse_x = _G.mouse_x or 0
    local mouse_y = _G.mouse_y or 0
    
    -- Handle dragging
    if is_mouse_pressed("LEFT") then
        local entity = can_drag_entity(mouse_x, mouse_y)
        if entity then
            for _, ball in ipairs(balls) do
                if ball == entity then
                    start_drag(entity, mouse_x, mouse_y)
                    set_gravity_enabled(entity, false)
                    break
                end
            end
        end
    else
        -- Re-enable gravity for any previously dragged ball
        for _, ball in ipairs(balls) do
            if is_dragging(ball) then
                set_gravity_enabled(ball, true)
                -- Give a small upward boost to prevent immediate floor collision
                local vx, vy = get_velocity(ball)
                set_velocity(ball, vx, math.min(vy, 0))
                break
            end
        end
        end_drag()
    end
    
    -- Handle collisions and bouncing
    for _, ball in ipairs(balls) do
        if not is_dragging(ball) then
            local vx, vy = get_velocity(ball)
            
            -- Check collisions with each wall
            for _, wall in ipairs(walls) do
                if is_colliding(ball, wall) then
                    handle_collision(ball, wall, vx, vy)
                    -- Get updated velocity after collision
                    vx, vy = get_velocity(ball)
                end
            end
            
            -- Additional floor check for extra safety
            local ball_x, ball_y = get_transform(ball)
            if ball_y > SCREEN_HEIGHT - WALL_THICKNESS - 20 then  -- 20 is ball radius
                set_transform(ball, ball_x, SCREEN_HEIGHT - WALL_THICKNESS - 20, 0, 1, 1)
                if vy > 0 then  -- Only bounce if moving downward
                    set_velocity(ball, vx, -math.abs(vy) * BOUNCE_DAMPING)
                end
            end
        end
    end
end
