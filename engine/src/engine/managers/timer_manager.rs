use super::Manager;
use crate::ecs::components::gamestate::GameState;
use crate::ecs::components::timer::{Timer, TimerId};
use mlua::Function;
use std::cell::RefCell;
use std::rc::Rc;
use tracing::{debug, error};

#[derive(Debug)]
pub struct TimerManager {
    state: Rc<RefCell<GameState>>,
}

impl Manager for TimerManager {
    fn new(state: Rc<RefCell<GameState>>) -> Self {
        debug!("Creating new TimerManager");
        Self { state }
    }

    fn get_state(&self) -> &Rc<RefCell<GameState>> {
        &self.state
    }
}

impl TimerManager {
    pub fn update_timer(&self, delta_time: f32) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                let mut finished = vec![];

                for (id, timer) in state.timers.iter_mut() {
                    timer.elapsed += delta_time;
                    if timer.elapsed >= timer.interval {
                        timer.elapsed -= timer.interval;

                        // Call the Lua function and handle potential errors
                        if let Err(e) = timer.callback.call::<()>(()) {
                            error!("Error calling timer callback: {}", e);
                            return Err("Timer callback failed");
                        }

                        if !timer.repeat {
                            finished.push(*id);
                        }
                    }
                }

                // Remove finished timers
                for id in finished {
                    state.remove_timer(id);
                }

                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn set_interval(
        &self,
        lua_callback: Function,
        interval: f32,
        repeat: bool,
    ) -> Result<TimerId, &'static str> {
        if interval <= 0.0 {
            return Err("Interval must be greater than 0");
        }

        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                let timer = Timer {
                    interval,
                    elapsed: 0.0,
                    repeat,
                    callback: lua_callback,
                };

                Ok(state.add_timer(timer))
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn clear_timer(&self, id: TimerId) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if !state.remove_timer(id) {
                    return Err("Timer not found");
                }
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }
}

