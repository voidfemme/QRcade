-- Constants for our game
local SCREEN_WIDTH = 800
local SCREEN_HEIGHT = 600
local WALL_THICKNESS = 20
local GRAVITY_FORCE = 4
local TERMINAL_VELOCITY = 300
local BOUNCE_DAMPING = 0.8

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
    table.insert(balls, ball)
    return ball
end

function on_start()
    -- Create walls around the screen
    -- Bottom
    create_wall(SCREEN_WIDTH/2, SCREEN_HEIGHT - WALL_THICKNESS/2, 
        SCREEN_WIDTH, WALL_THICKNESS)
    -- Top
    create_wall(SCREEN_WIDTH/2, WALL_THICKNESS/2, 
        SCREEN_WIDTH, WALL_THICKNESS)
    -- Left
    create_wall(WALL_THICKNESS/2, SCREEN_HEIGHT/2, 
        WALL_THICKNESS, SCREEN_HEIGHT)
    -- Right
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
        -- Check if we're already dragging something
        local dragging = false
        for _, ball in ipairs(balls) do
            if is_dragging(ball) then
                dragging = true
                update_drag(mouse_x, mouse_y)
                break
            end
        end
        
        -- If not dragging, try to start a drag
        if not dragging then
            local entity = can_drag_entity(mouse_x, mouse_y)
            if entity then
                start_drag(entity, mouse_x, mouse_y)
                -- Disable gravity while dragging
                set_gravity_enabled(entity, false)
            end
        end
    else
        -- Find which ball was being dragged (if any)
        for _, ball in ipairs(balls) do
            if is_dragging(ball) then
                -- Re-enable gravity when we stop dragging
                set_gravity_enabled(ball, true)
                break
            end
        end
        -- Release any dragged objects
        end_drag()
    end
    
    -- Handle collisions and bouncing
    for _, ball in ipairs(balls) do
        -- Skip if being dragged
        if not is_dragging(ball) then
            -- Check collisions with walls
            for _, wall in ipairs(walls) do
                if is_colliding(ball, wall) then
                    -- Get current velocity
                    local vx, vy = get_velocity(ball)
                    
                    -- Simple bounce - reverse velocity and apply damping
                    set_velocity(ball, vx * -BOUNCE_DAMPING, vy * -BOUNCE_DAMPING)
                    
                    -- Move ball slightly away from wall to prevent sticking
                    local x, y = get_transform(ball)
                    if wall == walls[1] or wall == walls[2] then  -- Top/bottom walls
                        set_transform(ball, x, y + (vy > 0 and -1 or 1), 0, 1, 1)
                    else  -- Side walls
                        set_transform(ball, x + (vx > 0 and -1 or 1), y, 0, 1, 1)
                    end
                end
            end
        end
    end
end
