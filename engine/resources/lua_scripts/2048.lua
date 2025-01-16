-- 2048 Game Implementation

-- Game configuration constants
local GRID_SIZE = 4
local CELL_SIZE = 100
local GRID_SPACING = 10
local GRID_OFFSET_X = 200  -- Center the grid on screen
local GRID_OFFSET_Y = 100

-- Color schemes for different numbers (R, G, B)
local NUMBER_COLORS = {
    [2] = {238, 228, 218},      -- Light beige for 2
    [4] = {237, 224, 200},      -- Darker beige for 4
    [8] = {242, 177, 121},      -- Orange for 8
    [16] = {245, 149, 99},      -- Darker orange for 16
    [32] = {246, 124, 95},      -- Red-orange for 32
    [64] = {246, 94, 59},       -- Red for 64
    [128] = {237, 207, 114},    -- Yellow for 128
    [256] = {237, 204, 97},     -- Brighter yellow for 256
    [512] = {237, 200, 80},     -- Even brighter yellow for 512
    [1024] = {237, 197, 63},    -- Golden yellow for 1024
    [2048] = {237, 194, 46},    -- Deep golden for 2048
}

-- Game state variables
local grid = {}              -- Stores the numerical values
local tile_entities = {}     -- Stores the visual entities
local score = 0             -- Current game score
local game_over = false     -- Game over flag

-- Initialize empty game grid
local function init_grid()
    for i = 1, GRID_SIZE do
        grid[i] = {}
        for j = 1, GRID_SIZE do
            grid[i][j] = 0  -- 0 represents empty cell
        end
    end
end

-- Create a visual representation of a number tile
local function create_tile_entity(value, row, col)
    local tile = create_entity()
    
    -- Calculate position in screen space
    local x = GRID_OFFSET_X + (col - 1) * (CELL_SIZE + GRID_SPACING)
    local y = GRID_OFFSET_Y + (row - 1) * (CELL_SIZE + GRID_SPACING)
    
    -- Get color for this number (or use default gray)
    local color = NUMBER_COLORS[value] or {200, 200, 200}
    
    -- Create visual representation as a rectangle
    add_shape(tile, "rectangle", color[1], color[2], color[3], 
             {width = CELL_SIZE, height = CELL_SIZE})
    set_transform(tile, x, y, 0, 1.0, 1.0)
    
    return tile
end

-- Add a new random tile (2 or 4) to an empty spot
local function add_random_tile()
    -- Find all empty spots in the grid
    local empty_spots = {}
    for i = 1, GRID_SIZE do
        for j = 1, GRID_SIZE do
            if grid[i][j] == 0 then
                table.insert(empty_spots, {row = i, col = j})
            end
        end
    end
    
    -- If no empty spots, return false
    if #empty_spots == 0 then
        return false
    end
    
    -- Choose random empty spot and value (90% chance of 2, 10% chance of 4)
    local spot = empty_spots[math.random(#empty_spots)]
    local value = math.random() < 0.9 and 2 or 4
    
    -- Update grid and create visual tile
    grid[spot.row][spot.col] = value
    local tile = create_tile_entity(value, spot.row, spot.col)
    tile_entities[spot.row .. "," .. spot.col] = tile
    
    return true
end

-- Move and merge tiles in a direction
local function move_tiles(direction)
    local moved = false
    local merged = {}  -- Track which tiles have already merged this turn
    
    -- Helper function to move a single tile
    local function try_move_tile(from_row, from_col, to_row, to_col)
        if grid[from_row][from_col] == 0 then return false end
        if grid[to_row][to_col] == 0 then
            -- Move to empty space
            grid[to_row][to_col] = grid[from_row][from_col]
            grid[from_row][from_col] = 0
            return true
        elseif grid[to_row][to_col] == grid[from_row][from_col] and 
               not merged[to_row .. "," .. to_col] then
            -- Merge same numbers
            grid[to_row][to_col] = grid[to_row][to_col] * 2
            grid[from_row][from_col] = 0
            score = score + grid[to_row][to_col]
            merged[to_row .. "," .. to_col] = true
            return true
        end
        return false
    end
    
    -- Process movement based on direction
    if direction == "UP" then
        for col = 1, GRID_SIZE do
            for row = 2, GRID_SIZE do
                for r = row, 2, -1 do
                    if try_move_tile(r, col, r-1, col) then
                        moved = true
                    end
                end
            end
        end
    elseif direction == "DOWN" then
        for col = 1, GRID_SIZE do
            for row = GRID_SIZE-1, 1, -1 do
                for r = row, GRID_SIZE-1 do
                    if try_move_tile(r, col, r+1, col) then
                        moved = true
                    end
                end
            end
        end
    elseif direction == "LEFT" then
        for row = 1, GRID_SIZE do
            for col = 2, GRID_SIZE do
                for c = col, 2, -1 do
                    if try_move_tile(row, c, row, c-1) then
                        moved = true
                    end
                end
            end
        end
    elseif direction == "RIGHT" then
        for row = 1, GRID_SIZE do
            for col = GRID_SIZE-1, 1, -1 do
                for c = col, GRID_SIZE-1 do
                    if try_move_tile(row, c, row, c+1) then
                        moved = true
                    end
                end
            end
        end
    end
    
    -- Update visual representation after movement
    for i = 1, GRID_SIZE do
        for j = 1, GRID_SIZE do
            local key = i .. "," .. j
            -- Remove old visual entity
            if tile_entities[key] then
                destroy_entity(tile_entities[key])
                tile_entities[key] = nil
            end
            -- Create new visual entity if cell is not empty
            if grid[i][j] > 0 then
                tile_entities[key] = create_tile_entity(grid[i][j], i, j)
            end
        end
    end
    
    return moved
end

-- Check if any moves are possible
local function check_game_over()
    -- First check for empty cells
    for i = 1, GRID_SIZE do
        for j = 1, GRID_SIZE do
            if grid[i][j] == 0 then
                return false
            end
        end
    end
    
    -- Then check for possible merges
    for i = 1, GRID_SIZE do
        for j = 1, GRID_SIZE do
            local value = grid[i][j]
            -- Check right neighbor
            if j < GRID_SIZE and grid[i][j+1] == value then
                return false
            end
            -- Check bottom neighbor
            if i < GRID_SIZE and grid[i+1][j] == value then
                return false
            end
        end
    end
    
    return true  -- No moves possible
end

-- Initialize game
function on_start()
    -- Create empty grid
    init_grid()
    
    -- Add initial two tiles
    add_random_tile()
    add_random_tile()
    
    print("Use arrow keys to play. Try to reach 2048!")
end

-- Main game loop
function on_frame(delta_time)
    if game_over then return end
    
    local moved = false
    
    -- Handle input
    if is_key_pressed("UP") then
        moved = move_tiles("UP")
    elseif is_key_pressed("DOWN") then
        moved = move_tiles("DOWN")
    elseif is_key_pressed("LEFT") then
        moved = move_tiles("LEFT")
    elseif is_key_pressed("RIGHT") then
        moved = move_tiles("RIGHT")
    end
    
    -- If tiles moved, add a new random tile
    if moved then
        add_random_tile()
        print("Score: " .. score)  -- Display current score
        
        -- Check for game over
        if check_game_over() then
            game_over = true
            print("Game Over! Final Score: " .. score)
        end
    end
end
