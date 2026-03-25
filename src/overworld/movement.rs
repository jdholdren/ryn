use serde::Deserialize;

// Position is a component that represents where an entity is in the world.
#[derive(Deserialize)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

// Velocity represents the entity moving in a certain direction with a certain magnitude.
pub struct Velocity {
    pub vector: (isize, isize),
}

use super::{Overworld, System};

pub struct MovementSystem;

impl System for MovementSystem {
    fn update(ow: &mut Overworld) {
        for entity in &mut ow.entities {
            if let (Some(pos), Some(vel)) = (&mut entity.pos, &entity.velocity) {
                let nx = pos.x as isize + vel.vector.0;
                let ny = pos.y as isize + vel.vector.1;
                if nx >= 0 && ny >= 0 {
                    pos.x = nx as usize;
                    pos.y = ny as usize;
                }
            }
        }
    }
}
