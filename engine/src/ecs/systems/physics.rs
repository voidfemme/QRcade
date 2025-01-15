use crate::ecs::components::gravity::GravityType;
use crate::GameState;
use std::collections::HashMap;

pub struct PhysicsSystem {
    // Cache for storing intermediate force calculations
    force_accumulator: HashMap<u32, (f32, f32)>,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            force_accumulator: HashMap::new(),
        }
    }

    pub fn update(&mut self, state: &mut GameState, delta_time: f32) {
        // Phase 1: Force Accumulation
        self.accumulate_forces(state, delta_time);

        // Phase 2: Velocity Integration
        self.integrate_velocities(state, delta_time);

        // Phase 3: Position Integration
        self.integrate_positions(state, delta_time);
    }

    fn accumulate_forces(&mut self, state: &GameState, delta_time: f32) {
        self.force_accumulator.clear();

        // Accumulate gravity forces
        for (&entity_id, gravity) in &state.gravities {
            if !gravity.enabled {
                continue;
            }

            if let Some(transform) = state.transforms.get(&entity_id) {
                let force = match gravity.gravity_type {
                    GravityType::Downward => {
                        // Simple downward force
                        (0.0, gravity.force)
                    }
                    GravityType::Attractive | GravityType::Repulsive => {
                        let mut total_fx = 0.0;
                        let mut total_fy = 0.0;

                        // Calculate gravitational influence from all other entities
                        for (&other_id, other_transform) in &state.transforms {
                            if other_id != entity_id {
                                let dx = other_transform.x - transform.x;
                                let dy = other_transform.y - transform.y;
                                let distance_squared = dx * dx + dy * dy;

                                if distance_squared > 0.0001 {
                                    let distance = distance_squared.sqrt();
                                    let force_magnitude = gravity.force / distance_squared;

                                    // Direction vector
                                    let dir_x = dx / distance;
                                    let dir_y = dy / distance;

                                    // Attractive pulls toward, Repulsive pushes away
                                    let multiplier = match gravity.gravity_type {
                                        GravityType::Attractive => 1.0,
                                        GravityType::Repulsive => -1.0,
                                        _ => unreachable!(),
                                    };

                                    total_fx += dir_x * force_magnitude * multiplier;
                                    total_fy += dir_y * force_magnitude * multiplier;
                                }
                            }
                        }
                        (total_fx, total_fy)
                    }
                };

                // Store the accumulated force
                self.force_accumulator.insert(entity_id, force);
            }
        }
    }

    fn integrate_velocities(&mut self, state: &mut GameState, delta_time: f32) {
        // Update velocities based on accumulated forces
        for (&entity_id, &(fx, fy)) in &self.force_accumulator {
            if let Some(velocity) = state.velocities.get_mut(&entity_id) {
                // Apply forces to change velocity
                velocity.dx += fx * delta_time;
                velocity.dy += fy * delta_time;

                // Apply terminal velocity if gravity is present
                if let Some(gravity) = state.gravities.get(&entity_id) {
                    let speed = (velocity.dx * velocity.dx + velocity.dy * velocity.dy).sqrt();
                    if speed > gravity.terminal_velocity {
                        let scale = gravity.terminal_velocity / speed;
                        velocity.dx *= scale;
                        velocity.dy *= scale;
                    }
                }
            }
        }

        // Apply angular velocities
        for (_, velocity) in state.velocities.iter_mut() {
            velocity.angular = velocity.angular.clamp(-10.0, 10.0); // Prevent excessive spinning
        }
    }

    fn integrate_positions(&self, state: &mut GameState, delta_time: f32) {
        // Update positions based on final velocities
        for (&entity_id, velocity) in &state.velocities {
            if let Some(transform) = state.transforms.get_mut(&entity_id) {
                // Linear motion
                transform.translate(velocity.dx * delta_time, velocity.dy * delta_time);

                // Angular motion
                transform.rotate(velocity.angular * delta_time);
            }
        }
    }
}

