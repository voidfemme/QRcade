-- Configuration
local TILE_SIZE = 32
local GRID_WIDTH = 20
local GRID_HEIGHT = 15
local MOVE_DELAY = 0.15 -- seconds between moves

-- Colors
local SNAKE_COLOR = { 0, 255, 0 }       -- Green
local FOOD_COLOR = { 255, 0, 0 }        -- Red
local WALL_COLOR = { 50, 50, 50 }       -- Dark gray
local BACKGROUND_COLOR = { 20, 20, 20 } -- Dark

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
local BORDER_THICKNESS = TILE_SIZE - 2 -- Same size as snake and food segments
local BORDER_COLOR = { 100, 100, 100 } -- A slightly lighter gray than the wall tiles

-- Calculate the dimensions of the play area in pixels
local total_width = GRID_WIDTH * TILE_SIZE
local total_height = GRID_HEIGHT * TILE_SIZE

-- Create the four border walls as separate entities
local borders = {
  top = create_entity(),
  bottom = create_entity(),
  left = create_entity(),
  right = create_entity(),
}

-- We'll position the borders to align perfectly with the grid.
-- Since our game objects are TILE_SIZE - 2 pixels, we'll center them on the tile boundaries.

-- Top border: spans the full width plus two extra tiles for corners
add_shape(
  borders.top,
  "rectangle",
  BORDER_COLOR[1],
  BORDER_COLOR[2],
  BORDER_COLOR[3],
  { width = total_width + TILE_SIZE * 2, height = TILE_SIZE - 2 }
)
set_transform(borders.top, total_width / 2, -(TILE_SIZE / 2), 0, 1.0, 1.0)

-- Bottom border: mirrors the top border
add_shape(
  borders.bottom,
  "rectangle",
  BORDER_COLOR[1],
  BORDER_COLOR[2],
  BORDER_COLOR[3],
  { width = total_width + TILE_SIZE * 2, height = TILE_SIZE - 2 }
)
set_transform(borders.bottom, total_width / 2, total_height + (TILE_SIZE / 2), 0, 1.0, 1.0)

-- Left border: spans the full height plus two extra tiles for corners
add_shape(
  borders.left,
  "rectangle",
  BORDER_COLOR[1],
  BORDER_COLOR[2],
  BORDER_COLOR[3],
  { width = TILE_SIZE - 2, height = total_height + TILE_SIZE * 2 }
)
set_transform(borders.left, -(TILE_SIZE / 2), total_height / 2, 0, 1.0, 1.0)

-- Right border: mirrors the left border
add_shape(
  borders.right,
  "rectangle",
  BORDER_COLOR[1],
  BORDER_COLOR[2],
  BORDER_COLOR[3],
  { width = TILE_SIZE - 2, height = total_height + TILE_SIZE * 2 }
)
set_transform(borders.right, total_width + (TILE_SIZE / 2), total_height / 2, 0, 1.0, 1.0)

-- Snake data
local snake = {
  head = create_entity(),
  body = {},                       -- Table of entities for body segments
  direction = { x = 1, y = 0 },    -- Start moving right
  next_direction = { x = 1, y = 0 },
  positions = { { x = 5, y = 5 } }, -- Starting position (includes head)
  length = 1,
}

-- Create snake head
add_shape(
  snake.head,
  "rectangle",
  SNAKE_COLOR[1],
  SNAKE_COLOR[2],
  SNAKE_COLOR[3],
  { width = TILE_SIZE - 2, height = TILE_SIZE - 2 }
) -- Slightly smaller than tile
set_transform(snake.head, snake.positions[1].x * TILE_SIZE, snake.positions[1].y * TILE_SIZE, 0, 1.0, 1.0)

-- Create food
local food = {
  entity = create_entity(),
  position = { x = 0, y = 0 },
}
add_shape(
  food.entity,
  "rectangle",
  FOOD_COLOR[1],
  FOOD_COLOR[2],
  FOOD_COLOR[3],
  { width = TILE_SIZE - 2, height = TILE_SIZE - 2 }
)

-- Place food in random position
local function place_food()
  while true do
    local x = math.random(1, GRID_WIDTH - 2)
    local y = math.random(1, GRID_HEIGHT - 2)

    -- Check if position is occupied by snake
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
      set_transform(food.entity, x * TILE_SIZE, y * TILE_SIZE, 0, 1.0, 1.0)
      break
    end
  end
end

-- Add a new body segment
local function add_body_segment()
  local segment = create_entity()
  add_shape(
    segment,
    "rectangle",
    SNAKE_COLOR[1],
    SNAKE_COLOR[2],
    SNAKE_COLOR[3],
    { width = TILE_SIZE - 2, height = TILE_SIZE - 2 }
  )
  table.insert(snake.body, segment)

  -- Add position (will be updated in next move)
  local last_pos = snake.positions[#snake.positions]
  table.insert(snake.positions, { x = last_pos.x, y = last_pos.y })
end

-- Check for collisions with walls or self
local function check_collision(x, y)
  -- Check wall collision
  if not is_walkable(map, x, y) then
    return true
  end

  -- Check self collision (skip head position)
  for i = 2, #snake.positions do
    if snake.positions[i].x == x and snake.positions[i].y == y then
      return true
    end
  end

  return false
end

-- Initialize first food
place_food()

-- Game state
local game_over = false
local move_cooldown = MOVE_DELAY
local score = 0

function on_frame(delta_time)
  if game_over then
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
      print("Game Over! Score: " .. score)
      return
    end

    -- Check for food collision
    if new_head_x == food.position.x and new_head_y == food.position.y then
      add_body_segment()
      place_food()
      score = score + 10
      print("Score: " .. score)
    end

    -- Move body (from tail to head)
    for i = #snake.positions, 2, -1 do
      snake.positions[i].x = snake.positions[i - 1].x
      snake.positions[i].y = snake.positions[i - 1].y
      -- Update body segment position
      if i > 1 then
        set_transform(
          snake.body[i - 1],
          snake.positions[i].x * TILE_SIZE,
          snake.positions[i].y * TILE_SIZE,
          0,
          1.0,
          1.0
        )
      end
    end

    -- Update head position
    snake.positions[1].x = new_head_x
    snake.positions[1].y = new_head_y
    set_transform(snake.head, snake.positions[1].x * TILE_SIZE, snake.positions[1].y * TILE_SIZE, 0, 1.0, 1.0)
  end
end

print("Use WASD or arrow keys to control the snake!")
print("Score: 0")
