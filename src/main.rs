mod render;
mod tiles;

use crossterm::{
    cursor::{Hide, Show},
    event::{Event, KeyCode, poll, read},
    execute,
    style::ResetColor,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{Result, stdout};
use std::time::{Duration, Instant};

const TARGET_FPS: u64 = 24;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / TARGET_FPS);
const WIDTH: usize = 80;
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
    let mut player_pos = (5, 5);
    let mut renderer = render::Renderer::<WIDTH, HEIGHT>::new();

    let mut last_fps: u128 = 0;

    let mut running = true;
    while running {
        let frame_start = Instant::now();

        // Handle input (non-blocking)
        if poll(Duration::from_millis(100)).unwrap_or(false)
            && let Ok(event) = read()
            && let Event::Key(key) = event
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => running = false,
                KeyCode::Char('w') | KeyCode::Up => {
                    if player_pos.1 > 1 {
                        player_pos.1 -= 1;
                    }
                }
                KeyCode::Char('s') | KeyCode::Down => {
                    if player_pos.1 < HEIGHT - 2 {
                        player_pos.1 += 1;
                    }
                }
                KeyCode::Char('a') | KeyCode::Left => {
                    if player_pos.0 > 0 {
                        player_pos.0 -= 1;
                    }
                }
                KeyCode::Char('d') | KeyCode::Right => {
                    if player_pos.0 < WIDTH - 1 {
                        player_pos.0 += 1;
                    }
                }
                _ => {}
            }
        }

        // TODO: Run update loop

        // Render
        let frame = create_frame(player_pos);
        renderer.render(frame, last_fps)?;

        // Sleep for remaining time to maintain target FPS
        let frame_time = frame_start.elapsed();
        if frame_time < FRAME_DURATION {
            std::thread::sleep(FRAME_DURATION - frame_time);
        }

        // Calculate how long this iteration took
        let end_loop = frame_start.elapsed();
        last_fps = 1000_i32
            .checked_div(end_loop.as_millis() as i32)
            .unwrap_or(1000) as u128;
    }

    Ok(())
}

// For now, takes a player position and makes a frame from the game state (player position).
fn create_frame(player_pos: (usize, usize)) -> render::Frame<WIDTH, HEIGHT> {
    let mut frame = [[' '; HEIGHT]; WIDTH];

    // Draw player
    frame[player_pos.0][player_pos.1] = '@';

    frame
}
