mod render;
mod title;

use crossterm::{
    cursor::{Hide, Show},
    event::{Event, KeyCode, KeyEvent, poll, read},
    execute,
    style::{Color, ResetColor},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{Result, stdout};
use std::time::{Duration, Instant};

use crate::render::Tile;
use crate::title::Title;

const TARGET_FPS: u64 = 24;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / TARGET_FPS);
const WIDTH: usize = 100;
const HEIGHT: usize = 54;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    execute!(stdout(), Hide)?;

    let result = run_game();

    // Cleanup
    execute!(stdout(), Show, ResetColor)?;
    disable_raw_mode()?;

    result
}

fn run_game() -> Result<()> {
    let mut renderer = render::Renderer::<WIDTH, HEIGHT>::new();

    let mut screen: Box<dyn Screen> = Box::new(Title::new());

    let mut last_fps: u128 = 0;
    let mut elapsed: Duration = Duration::ZERO;
    while 1 == 1 {
        let frame_start = Instant::now();

        // Handle input (non-blocking)
        let mut key_press: Option<KeyEvent> = None;
        if poll(Duration::from_millis(100)).unwrap_or(false)
            && let Ok(event) = read()
            && let Event::Key(key) = event
        {
            key_press = Some(key);
        }

        // Detecting ctrl-c to exit
        if let Some(event) = key_press
            && event.code == KeyCode::Char('c')
            && event
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL)
        {
            break;
        }

        // Run update loop
        let (should_render, should_quit) = screen.update(key_press, elapsed);
        if should_quit {
            break;
        }

        // Render
        if should_render {
            renderer.render(screen.produce_frame(), last_fps)?;
        }

        // Sleep for remaining time to maintain target FPS
        let frame_time = frame_start.elapsed();
        if frame_time < FRAME_DURATION {
            std::thread::sleep(FRAME_DURATION - frame_time);
        }

        // Calculate how long this iteration took
        elapsed = frame_start.elapsed();
        last_fps = 1000_i32
            .checked_div(elapsed.as_millis() as i32)
            .unwrap_or(1000) as u128;
    }

    Ok(())
}

// A screen is a layer of the game that is essentially a game world.
//
// It can receive player input, decide to move its own tick, and then
// produce a frame.
trait Screen {
    // Update receives elapsed time and updates its state based on player input.
    fn update(&mut self, event: Option<KeyEvent>, elapsed: Duration) -> (bool, bool);
    // Asks the screen to produce a frame, could produce no frame if nothing of import happened.
    fn produce_frame(&self) -> render::Frame<WIDTH, HEIGHT>;
}

// Overworld is tracking player locations along with just things on the map.
struct Overworld {
    player_pos: (usize, usize),
}

impl Screen for Overworld {
    fn update(&mut self, event: Option<KeyEvent>, _: Duration) -> (bool, bool) {
        // If no player input, don't render
        if event.is_none() {
            return (false, false);
        }

        let key = event.unwrap();
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return (false, true),
            KeyCode::Char('w') | KeyCode::Up => {
                if self.player_pos.1 > 1 {
                    self.player_pos.1 -= 1;
                }
            }
            KeyCode::Char('s') | KeyCode::Down => {
                if self.player_pos.1 < HEIGHT - 2 {
                    self.player_pos.1 += 1;
                }
            }
            KeyCode::Char('a') | KeyCode::Left => {
                if self.player_pos.0 > 0 {
                    self.player_pos.0 -= 1;
                }
            }
            KeyCode::Char('d') | KeyCode::Right => {
                if self.player_pos.0 < WIDTH - 1 {
                    self.player_pos.0 += 1;
                }
            }
            _ => {}
        }

        return (true, false);
    }

    fn produce_frame(&self) -> render::Frame<WIDTH, HEIGHT> {
        let mut frame = [[Tile {
            c: ' ',
            color: Color::White,
        }; HEIGHT]; WIDTH];
        const BORDER: render::Tile = render::Tile {
            c: '#',
            color: Color::White,
        };

        // Draw the border
        for i in 0..WIDTH {
            frame[i][0] = BORDER;
            frame[i][HEIGHT - 1] = BORDER;
        }
        for i in 0..HEIGHT {
            frame[0][i] = BORDER;
            frame[WIDTH - 1][i] = BORDER;
        }

        // Draw player
        frame[self.player_pos.0][self.player_pos.1] = Tile {
            c: '@',
            color: Color::White,
        };

        frame
    }
}
