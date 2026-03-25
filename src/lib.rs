pub mod location;
pub mod overworld;
pub mod render;

use std::time::Duration;

use crossterm::event::KeyEvent;

pub const WIDTH: usize = 100;
pub const HEIGHT: usize = 54;

// A screen is a layer of the game that is essentially a game world.
//
// It can receive player input, decide to move its own tick, and then
// produce a frame.
pub trait Screen {
    // Update receives elapsed time and updates its state based on player input.
    // Returns true if the game should quit.
    fn update(&mut self, event: Option<KeyEvent>, elapsed: Duration);

    // Produces the current frame for rendering.
    fn produce_frame(&self) -> render::Frame<WIDTH, HEIGHT>;
}
