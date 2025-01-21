------------------------------------------------
-- Window / Board Configuration
------------------------------------------------
local WINDOW_WIDTH = 800
local WINDOW_HEIGHT = 600

local GRID_WIDTH = 25
local GRID_HEIGHT = 25
local CELL_SIZE = 16

-- Compute the size of the board in pixels
local BOARD_PIXEL_WIDTH = GRID_WIDTH * CELL_SIZE
local BOARD_PIXEL_HEIGHT = GRID_HEIGHT * CELL_SIZE

-- Offset so the board is centered in the window
local BOARD_OFFSET_X = math.floor((WINDOW_WIDTH - BOARD_PIXEL_WIDTH) / 2)
local BOARD_OFFSET_Y = math.floor((WINDOW_HEIGHT - BOARD_PIXEL_HEIGHT) / 2)

------------------------------------------------
-- Global Configuration
------------------------------------------------
local SNAKE_SPEED = 0.15
------------------------------------------------
-- Global State
------------------------------------------------
local snake_segments = {}
local directions = {
	UP = { x = 0, y = -1 },
	DOWN = { x = 0, y = 1 },
	LEFT = { x = -1, y = 0 },
	RIGHT = { x = 1, y = 0 },
}
local current_dir = directions.RIGHT
local move_timer = 0
local score = 0
local is_game_over = false

local snake_head
local food_entity
local score_display
local game_over_text

------------------------------------------------
-- Helper Functions
------------------------------------------------

-- Create a snake segment in grid coords.
-- We'll still place it at the *center* of the cell.
local function create_snake_segment(gx, gy)
	local seg = create_entity()
	local px = (gx + 0.5) * CELL_SIZE
	local py = (gy + 0.5) * CELL_SIZE
	set_transform(seg, px, py, 0)

	-- Make shape smaller than the cell so we can see gaps,
	-- but it won't matter for the new body collision check.
	local shape_size = CELL_SIZE - 2
	add_shape(seg, "rectangle", 0, 255, 0, { w = shape_size, h = shape_size })

	return seg
end

local function move_segment(entity_id, gx, gy)
	local px = (gx + 0.5) * CELL_SIZE
	local py = (gy + 0.5) * CELL_SIZE
	set_transform(entity_id, px, py, 0)
end

local function get_grid_position(entity_id)
	local x, y, _ = get_transform(entity_id)
	-- Convert from center-based position to integer grid coords
	local gx = math.floor(x / CELL_SIZE)
	local gy = math.floor(y / CELL_SIZE)
	return gx, gy
end

local function place_food_random()
	local gx = math.random(0, GRID_WIDTH - 1)
	local gy = math.random(0, GRID_HEIGHT - 1)
	local px = (gx + 0.5) * CELL_SIZE
	local py = (gy + 0.5) * CELL_SIZE
	set_transform(food_entity, px, py, 0)
end

-- GRID-BASED check for body collision
local function is_colliding_with_self_grid()
	-- Grab head coords
	local hx, hy = get_grid_position(snake_head)

	-- If any body segment (2..end) occupies the same cell, that's a collision
	for i = 2, #snake_segments do
		local gx, gy = get_grid_position(snake_segments[i])
		if gx == hx and gy == hy then
			return true
		end
	end
	return false
end

local function grow_snake()
	local tail = snake_segments[#snake_segments]
	local tail_gx, tail_gy = get_grid_position(tail)
	local new_segment = create_snake_segment(tail_gx, tail_gy)
	table.insert(snake_segments, new_segment)
end

local function update_score_text()
	update_text(score_display, TEXT.SCORE)
	-- If your engine supports dynamic text:
	-- set_text_value(score_display, "Score: "..tostring(score))
end

local function end_game()
	is_game_over = true
	set_text_visibility(game_over_text, true)
	print("GAME OVER triggered!")
end

------------------------------------------------
-- Script Callbacks
------------------------------------------------

function on_start()
	math.randomseed(os.time())

	------------------------------------------------
	-- 1) Build initial snake
	------------------------------------------------
	local start_x = math.floor(GRID_WIDTH / 2)
	local start_y = math.floor(GRID_HEIGHT / 2)

	for i = 0, 2 do
		local seg = create_snake_segment(start_x - i, start_y)
		table.insert(snake_segments, seg)
	end
	snake_head = snake_segments[1]

	-- Debug info
	for i, seg in ipairs(snake_segments) do
		local gx, gy = get_grid_position(seg)
		print(string.format("Segment #%d: grid=(%d,%d)", i, gx, gy))
	end

	------------------------------------------------
	-- 2) Create food
	------------------------------------------------
	food_entity = create_entity()
	local shape_size = CELL_SIZE - 2
	add_shape(food_entity, "rectangle", 255, 0, 0, { w = shape_size, h = shape_size })
	place_food_random()

	------------------------------------------------
	-- 3) Score display
	------------------------------------------------
	score_display = create_entity()
	set_transform(score_display, 10, 10, 0)
	add_text(score_display, TEXT.SCORE, {
		color = { 255, 255, 255 },
		scale = 1.0,
		h_align = ALIGN.LEFT,
		v_align = ALIGN.TOP,
	})
	update_score_text()

	------------------------------------------------
	-- 4) Game over text (hidden initially)
	------------------------------------------------
	game_over_text = create_entity()
	local center_x = (GRID_WIDTH * CELL_SIZE) / 2
	local center_y = (GRID_HEIGHT * CELL_SIZE) / 2
	set_transform(game_over_text, center_x, center_y, 0)
	add_text(game_over_text, TEXT.GAME_OVER, {
		color = { 255, 0, 0 },
		scale = 2.0,
		h_align = ALIGN.CENTER,
		v_align = ALIGN.MIDDLE,
	})
	set_text_visibility(game_over_text, false)
end

function on_frame(delta_time)
	if is_game_over then
		return
	end

	------------------------------------------------
	-- 1) Handle input
	------------------------------------------------
	if is_key_pressed("LEFT") and current_dir ~= directions.RIGHT then
		current_dir = directions.LEFT
	elseif is_key_pressed("RIGHT") and current_dir ~= directions.LEFT then
		current_dir = directions.RIGHT
	elseif is_key_pressed("UP") and current_dir ~= directions.DOWN then
		current_dir = directions.UP
	elseif is_key_pressed("DOWN") and current_dir ~= directions.UP then
		current_dir = directions.DOWN
	end

	------------------------------------------------
	-- 2) Move the snake on a timer
	------------------------------------------------
	move_timer = move_timer + delta_time
	if move_timer >= SNAKE_SPEED then
		move_timer = move_timer - SNAKE_SPEED

		-- Old head coords
		local head_gx, head_gy = get_grid_position(snake_head)
		-- New head coords
		local new_head_x = head_gx + current_dir.x
		local new_head_y = head_gy + current_dir.y

		-- Walls
		if new_head_x < 0 or new_head_x >= GRID_WIDTH or new_head_y < 0 or new_head_y >= GRID_HEIGHT then
			print("Hit wall at (" .. new_head_x .. "," .. new_head_y .. ")")
			end_game()
			return
		end

		-- Move body from tail->head
		for i = #snake_segments, 2, -1 do
			local prev_gx, prev_gy = get_grid_position(snake_segments[i - 1])
			move_segment(snake_segments[i], prev_gx, prev_gy)
		end
		move_segment(snake_head, new_head_x, new_head_y)

		-- Check self-collision via grid
		if is_colliding_with_self_grid() then
			print("Self collision by grid logic!")
			end_game()
			return
		end

		-- Check food collision using engine's bounding-box method
		if is_colliding(snake_head, food_entity) then
			print("Food eaten! Score: " .. score .. " -> " .. (score + 1))
			score = score + 1
			grow_snake()
			place_food_random()
			update_score_text()
		end
	end
end

function on_end()
	print("Game is ending.")
end
