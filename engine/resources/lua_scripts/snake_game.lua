-- Configuration
local TILE_SIZE = 32
local GRID_WIDTH = 20
local GRID_HEIGHT = 15
local MOVE_DELAY = 0.15 -- seconds between moves

-- Calculate window dimensions and offset for centering
local WINDOW_WIDTH = 800  -- Match your window size
local WINDOW_HEIGHT = 600 -- Match your window size

-- Calculate the game field dimensions
local GAME_WIDTH = GRID_WIDTH * TILE_SIZE
local GAME_HEIGHT = GRID_HEIGHT * TILE_SIZE

-- Calculate offsets to center the game field
local OFFSET_X = (WINDOW_WIDTH - GAME_WIDTH) / 2
local OFFSET_Y = (WINDOW_HEIGHT - GAME_HEIGHT) / 2

-- Colors
local SNAKE_COLOR = { 0, 255, 0 }       -- Green
local FOOD_COLOR = { 255, 0, 0 }        -- Red
local WALL_COLOR = { 50, 50, 50 }       -- Dark gray
local BACKGROUND_COLOR = { 20, 20, 20 } -- Dark

-- UI Elements
local ui = {
    score_display = nil,
    game_over_text = nil,
    ready_text = nil,
    instructions = nil
}

-- Helper function to convert grid position to screen position
local function grid_to_screen(grid_x, grid_y)
    return OFFSET_X + (grid_x * TILE_SIZE), OFFSET_Y + (grid_y * TILE_SIZE)
end

-- Create UI elements
local function create_ui()
    -- Score display (top center)
    ui.score_display = create_entity()
    set_transform(ui.score_display, WINDOW_WIDTH / 2, 30, 0)
    add_text(ui.score_display, TEXT.SCORE, {
        color = {255, 255, 255},
        scale = 1.5,
        h_align = ALIGN.CENTER,
        v_align = ALIGN.TOP
    })

    -- Game over text (center)
    ui.game_over_text = create_entity()
    set_transform(ui.game_over_text, WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2 - 30, 0)
    add_text(ui.game_over_text, TEXT.GAME_OVER, {
        color = {255, 0, 0},
        scale = 3.0,
        h_align = ALIGN.CENTER,
        v_align = ALIGN.MIDDLE
    })
    set_text_visibility(ui.game_over_text, false)

    -- Try again text
    ui.try_again_text = create_entity()
    set_transform(ui.try_again_text, WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2 + 30, 0)
    add_text(ui.try_again_text, TEXT.TRY_AGAIN, {
        color = {255, 255, 255},
        scale = 1.5,
        h_align = ALIGN.CENTER,
        v_align = ALIGN.MIDDLE
    })
    set_text_visibility(ui.try_again_text, false)

    -- Ready text
    ui.ready_text = create_entity()
    set_transform(ui.ready_text, WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2 - 30, 0)
    add_text(ui.ready_text, TEXT.READY, {
        color = {0, 255, 0},
        scale = 3.0,
        h_align = ALIGN.CENTER,
        v_align = ALIGN.MIDDLE
    })

    -- Instructions
    ui.instructions = create_entity()
    set_transform(ui.instructions, WINDOW_WIDTH / 2, WINDOW_HEIGHT - 30, 0)
    add_text(ui.instructions, TEXT.PRESS_SPACE, {
        color = {200, 200, 200},
        scale = 1.0,
        h_align = ALIGN.CENTER,
        v_align = ALIGN.BOTTOM
    })
end

-- Create the game field
local map = create_entity()
create_tilemap(map, GRID_WIDTH, GRID_HEIGHT, TILE_SIZE)

-- Create walls around the edge
for x = 0, GRID_WIDTH - 1 do
    for y = 0, GRID_HEIGHT - 1 do
        if x == 0 or x == GRID_WIDTH - 1 or y == 0 or y == GRID_HEIGHT - 1 then
            set_tile(map, x, y, 1, false, WALL_COLOR[1], WALL_COLOR[2], WALL_COLOR[3])
        else
            set_tile(map, x, y, 0, true, BACKGROUND_COLOR[1], BACKGROUND_COLOR[2], BACKGROUND_COLOR[3])
        end
    end
end

-- Create border walls that frame the play area
local BORDER_THICKNESS = TILE_SIZE - 2
local BORDER_COLOR = { 100, 100, 100 }

local borders = {
    top = create_entity(),
    bottom = create_entity(),
    left = create_entity(),
    right = create_entity(),
}

-- Adjust border positions to account for offset
add_shape(borders.top, "rectangle", BORDER_COLOR[1], BORDER_COLOR[2], BORDER_COLOR[3],
    { width = GAME_WIDTH + TILE_SIZE * 2, height = TILE_SIZE - 2 })
set_transform(borders.top, OFFSET_X + GAME_WIDTH / 2, OFFSET_Y - (TILE_SIZE / 2), 0, 1.0, 1.0)

add_shape(borders.bottom, "rectangle", BORDER_COLOR[1], BORDER_COLOR[2], BORDER_COLOR[3],
    { width = GAME_WIDTH + TILE_SIZE * 2, height = TILE_SIZE - 2 })
set_transform(borders.bottom, OFFSET_X + GAME_WIDTH / 2, OFFSET_Y + GAME_HEIGHT + (TILE_SIZE / 2), 0, 1.0, 1.0)

add_shape(borders.left, "rectangle", BORDER_COLOR[1], BORDER_COLOR[2], BORDER_COLOR[3],
    { width = TILE_SIZE - 2, height = GAME_HEIGHT + TILE_SIZE * 2 })
set_transform(borders.left, OFFSET_X - (TILE_SIZE / 2), OFFSET_Y + GAME_HEIGHT / 2, 0, 1.0, 1.0)

add_shape(borders.right, "rectangle", BORDER_COLOR[1], BORDER_COLOR[2], BORDER_COLOR[3],
    { width = TILE_SIZE - 2, height = GAME_HEIGHT + TILE_SIZE * 2 })
set_transform(borders.right, OFFSET_X + GAME_WIDTH + (TILE_SIZE / 2), OFFSET_Y + GAME_HEIGHT / 2, 0, 1.0, 1.0)

-- Snake data
local snake = {
    head = create_entity(),
    body = {},
    direction = { x = 1, y = 0 },
    next_direction = { x = 1, y = 0 },
    positions = { { x = 5, y = 5 } },
    length = 1,
}

-- Update function to convert grid coordinates to screen coordinates
local function update_entity_position(entity, grid_x, grid_y)
    local screen_x, screen_y = grid_to_screen(grid_x, grid_y)
    set_transform(entity, screen_x, screen_y, 0, 1.0, 1.0)
end

-- Create snake head and set initial position
add_shape(snake.head, "rectangle", SNAKE_COLOR[1], SNAKE_COLOR[2], SNAKE_COLOR[3],
    { width = TILE_SIZE - 2, height = TILE_SIZE - 2 })
update_entity_position(snake.head, snake.positions[1].x, snake.positions[1].y)

-- Create food
local food = {
    entity = create_entity(),
    position = { x = 0, y = 0 },
}
add_shape(food.entity, "rectangle", FOOD_COLOR[1], FOOD_COLOR[2], FOOD_COLOR[3],
    { width = TILE_SIZE - 2, height = TILE_SIZE - 2 })

-- Place food in random position (updated to use grid_to_screen)
local function place_food()
    while true do
        local x = math.random(1, GRID_WIDTH - 2)
        local y = math.random(1, GRID_HEIGHT - 2)

        local occupied = false
        for _, pos in ipairs(snake.positions) do
            if pos.x == x and pos.y == y then
                occupied = true
                break
            end
        end

        if not occupied then
            food.position.x = x
            food.position.y = y
            update_entity_position(food.entity, x, y)
            break
        end
    end
end

-- Add a new body segment (updated to use grid_to_screen)
local function add_body_segment()
    local segment = create_entity()
    add_shape(segment, "rectangle", SNAKE_COLOR[1], SNAKE_COLOR[2], SNAKE_COLOR[3],
        { width = TILE_SIZE - 2, height = TILE_SIZE - 2 })
    table.insert(snake.body, segment)

    local last_pos = snake.positions[#snake.positions]
    table.insert(snake.positions, { x = last_pos.x, y = last_pos.y })
    update_entity_position(segment, last_pos.x, last_pos.y)
end

-- Check for collisions with walls or self
local function check_collision(x, y)
    if not is_walkable(map, x, y) then
        return true
    end

    for i = 2, #snake.positions do
        if snake.positions[i].x == x and snake.positions[i].y == y then
            return true
        end
    end

    return false
end

-- Update score display
local function update_score()
    update_text(ui.score_display, TEXT.SCORE)
end

-- Show game over screen
local function show_game_over()
    set_text_visibility(ui.game_over_text, true)
    set_text_visibility(ui.try_again_text, true)
    set_text_visibility(ui.instructions, true)
end

-- Reset game
local function reset_game()
    -- Reset snake
    snake.positions = { { x = 5, y = 5 } }
    snake.direction = { x = 1, y = 0 }
    snake.next_direction = { x = 1, y = 0 }
    
    -- Remove old body segments
    for _, segment in ipairs(snake.body) do
        destroy_entity(segment)
    end
    snake.body = {}
    
    -- Reset position
    update_entity_position(snake.head, snake.positions[1].x, snake.positions[1].y)
    
    -- Reset game state
    game_over = false
    score = 0
    update_score()
    
    -- Place new food
    place_food()
    
    -- Hide game over screen
    set_text_visibility(ui.game_over_text, false)
    set_text_visibility(ui.try_again_text, false)
    
    -- Show ready screen
    set_text_visibility(ui.ready_text, true)
    set_text_visibility(ui.instructions, true)
    game_started = false
end

-- Initialize first food
place_food()

-- Game state
local game_over = false
local game_started = false
local move_cooldown = MOVE_DELAY
local score = 0

-- Initialize UI
create_ui()

function on_frame(delta_time)
    -- Handle game start
    if not game_started then
        if is_key_pressed("SPACE") then
            game_started = true
            set_text_visibility(ui.ready_text, false)
            set_text_visibility(ui.instructions, false)
        end
        return
    end

    -- Handle game over
    if game_over then
        if is_key_pressed("SPACE") then
            reset_game()
        end
        return
    end

    -- Handle input
    if (is_key_pressed("UP") or is_key_pressed("W")) and snake.direction.y == 0 then
        snake.next_direction = { x = 0, y = -1 }
    elseif (is_key_pressed("DOWN") or is_key_pressed("S")) and snake.direction.y == 0 then
        snake.next_direction = { x = 0, y = 1 }
    elseif (is_key_pressed("LEFT") or is_key_pressed("A")) and snake.direction.x == 0 then
        snake.next_direction = { x = -1, y = 0 }
    elseif (is_key_pressed("RIGHT") or is_key_pressed("D")) and snake.direction.x == 0 then
        snake.next_direction = { x = 1, y = 0 }
    end

    -- Update movement cooldown
    move_cooldown = move_cooldown - delta_time
    if move_cooldown <= 0 then
        move_cooldown = MOVE_DELAY

        -- Update direction
        snake.direction = snake.next_direction

        -- Calculate new head position
        local new_head_x = snake.positions[1].x + snake.direction.x
        local new_head_y = snake.positions[1].y + snake.direction.y

        -- Check for collisions
        if check_collision(new_head_x, new_head_y) then
            game_over = true
            show_game_over()
            return
        end

        -- Check for food collision
        if new_head_x == food.position.x and new_head_y == food.position.y then
            add_body_segment()
            place_food()
            score = score + 10
            update_score()
        end

        -- Move body (from tail to head)
        for i = #snake.positions, 2, -1 do
            snake.positions[i].x = snake.positions[i - 1].x
            snake.positions[i].y = snake.positions[i - 1].y
            if i > 1 then
                update_entity_position(snake.body[i - 1], snake.positions[i].x, snake.positions[i].y)
            end
        end

        -- Update head position
        snake.positions[1].x = new_head_x
        snake.positions[1].y = new_head_y
        update_entity_position(snake.head, snake.positions[1].x, snake.positions[1].y)
    end
end

-- Initialize the game
reset_game()
