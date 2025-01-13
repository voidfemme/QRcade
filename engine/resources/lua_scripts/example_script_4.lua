-- Configuration
local TILE_SIZE = 32
local PLAYER_COLOR = {0, 255, 0}    -- Green
local WALL_COLOR = {50, 50, 50}     -- Dark gray
local FLOOR_COLOR = {200, 200, 200}  -- Light gray

-- Create the map entity
local map = create_entity()
create_tilemap(map, 10, 10, TILE_SIZE)

-- Simple maze layout (1 = wall, 0 = floor)
local maze = {
    {1, 1, 1, 1, 1, 1, 1, 1, 1, 1},
    {1, 0, 0, 0, 0, 1, 0, 0, 0, 1},
    {1, 0, 1, 1, 0, 0, 0, 1, 0, 1},
    {1, 0, 0, 0, 0, 1, 1, 1, 0, 1},
    {1, 0, 1, 1, 0, 0, 0, 0, 0, 1},
    {1, 0, 0, 1, 1, 1, 1, 1, 0, 1},
    {1, 1, 0, 0, 0, 0, 0, 0, 0, 1},
    {1, 0, 0, 1, 1, 1, 1, 1, 0, 1},
    {1, 0, 0, 0, 0, 0, 0, 0, 0, 1},
    {1, 1, 1, 1, 1, 1, 1, 1, 1, 1}
}

-- Place the tiles
for y = 1, #maze do
    for x = 1, #maze[y] do
        local tile_type = maze[y][x]
        if tile_type == 1 then
            -- Wall tile
            set_tile(map, x-1, y-1, 1, false, WALL_COLOR[1], WALL_COLOR[2], WALL_COLOR[3])
        else
            -- Floor tile
            set_tile(map, x-1, y-1, 0, true, FLOOR_COLOR[1], FLOOR_COLOR[2], FLOOR_COLOR[3])
        end
    end
end

-- Create player entity
local player = create_entity()
local player_pos = {
    grid_x = 1,  -- Starting position (in grid coordinates)
    grid_y = 1
}

-- Update player position function
local function update_player_position()
    set_transform(player, 
        player_pos.grid_x * TILE_SIZE,  -- Engine will handle centering
        player_pos.grid_y * TILE_SIZE, 
        0, 1.0, 1.0)
end

-- Create player sprite
add_shape(player, "square", PLAYER_COLOR[1], PLAYER_COLOR[2], PLAYER_COLOR[3], {width = 20, height = 20})
update_player_position()

-- Helper function to check if a grid position is walkable
local function can_move_to(x, y)
    return is_walkable(map, x, y)
end

-- Movement cooldown to prevent too rapid movement
local move_cooldown = 0
local MOVE_DELAY = 0.15  -- seconds between moves

function on_frame(delta_time)
    -- Update cooldown
    if move_cooldown > 0 then
        move_cooldown = move_cooldown - delta_time
        return
    end

    -- Handle movement
    local dx, dy = 0, 0
    
    if is_key_pressed("UP") or is_key_pressed("W") then
        dy = -1
    elseif is_key_pressed("DOWN") or is_key_pressed("S") then
        dy = 1
    elseif is_key_pressed("LEFT") or is_key_pressed("A") then
        dx = -1
    elseif is_key_pressed("RIGHT") or is_key_pressed("D") then
        dx = 1
    end
    
    -- Try to move if a direction was pressed
    if dx ~= 0 or dy ~= 0 then
        local new_x = player_pos.grid_x + dx
        local new_y = player_pos.grid_y + dy
        
        if can_move_to(new_x, new_y) then
            player_pos.grid_x = new_x
            player_pos.grid_y = new_y
            update_player_position()
            move_cooldown = MOVE_DELAY  -- Set cooldown
        end
    end
end

print("Use WASD or arrow keys to move the green square")
