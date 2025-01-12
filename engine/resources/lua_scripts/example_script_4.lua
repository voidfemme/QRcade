-- Constants
local TILE_SIZE = 32

-- Predefined maze layout: 0 = walkable, 1 = wall
local maze = {
  {1,1,1,1,1,1,1,1,1,1},
  {1,0,0,1,0,0,0,1,0,1},
  {1,0,1,1,0,1,0,1,0,1},
  {1,0,0,0,0,1,0,0,0,1},
  {1,1,1,1,0,1,1,1,0,1},
  {1,0,0,0,0,0,0,1,0,1},
  {1,0,1,1,1,1,0,1,0,1},
  {1,0,0,0,0,1,0,0,0,1},
  {1,1,1,1,1,1,1,1,0,1},
  {1,1,1,1,1,1,1,1,1,1},
}

-- Create entities
local player = create_entity()
local map_entity = create_entity()

-- Create a 10x10 tilemap for our maze
create_tilemap(map_entity, 10, 10, TILE_SIZE)
local tilemap = map_entity

-- Build the maze using the predefined layout
for y = 0, 9 do
  for x = 0, 9 do
    if maze[y+1][x+1] == 1 then
      set_tile(tilemap, x, y, 1, false, 255, 0, 0)  -- wall
    else
      set_tile(tilemap, x, y, 2, true, 0, 255, 0)   -- walkable
    end
  end
end

-- Set player's starting tile position at (1,1)
local player_tile_x = 1
local player_tile_y = 1

set_transform(player, player_tile_x * TILE_SIZE, player_tile_y * TILE_SIZE, 0, 1, 1)
add_shape(player, "square", 0, 0, 255, {size = TILE_SIZE - 4})

function move_player(dx, dy)
  local new_tile_x = player_tile_x + dx
  local new_tile_y = player_tile_y + dy
  
  -- Check bounds
  if new_tile_x < 0 or new_tile_x > 9 or new_tile_y < 0 or new_tile_y > 9 then
    return
  end
  
  if is_walkable(tilemap, new_tile_x, new_tile_y) then
    player_tile_x = new_tile_x
    player_tile_y = new_tile_y
    set_transform(player, player_tile_x * TILE_SIZE, player_tile_y * TILE_SIZE, 0, 1, 1)
    print("Moved to tile:", player_tile_x, player_tile_y)
  else
    print("Hit a wall at:", new_tile_x, new_tile_y)
  end
end

function update()
  local moved = false

  if is_key_pressed("W") then
    move_player(0, -1)
    moved = true
  end
  if is_key_pressed("S") then
    move_player(0, 1)
    moved = true
  end
  if is_key_pressed("A") then
    move_player(-1, 0)
    moved = true
  end
  if is_key_pressed("D") then
    move_player(1, 0)
    moved = true
  end

  if moved then
    print("Player position:", player_tile_x, player_tile_y)
  end
end

