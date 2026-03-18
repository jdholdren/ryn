use crossterm::{
    cursor::{Hide, Show},
    event::{Event, KeyCode, KeyEvent, poll, read},
    execute,
    style::ResetColor,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{Result, stdout};
use std::path::Path;
use std::time::{Duration, Instant};

use ryn::location;
use ryn::overworld::Overworld;
use ryn::render;
use ryn::{HEIGHT, Screen, WIDTH};

const TARGET_FPS: u64 = 24;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / TARGET_FPS);

fn main() -> Result<()> {
    // Load all locations before entering raw mode (fast-fail on bad data)
    let mut locations = location::load_locations(Path::new("maps"));
    let spawn = locations
        .remove("school_room")
        .expect("maps/school_room.txt is required");

    // Setup terminal
    enable_raw_mode()?;
    // Hide the cursor
    execute!(stdout(), Hide)?;

    let result = run_game(spawn);

    // Cleanup
    execute!(stdout(), Show, ResetColor)?;
    disable_raw_mode()?;

    result
}

fn run_game(spawn: location::Location) -> Result<()> {
    let mut renderer = render::Renderer::<WIDTH, HEIGHT>::new();

    let mut screen: Box<dyn Screen> = Box::new(Overworld::new("School Room".to_string(), spawn));

    let mut last_fps: u128 = 0;
    let mut elapsed: Duration = Duration::ZERO;
    loop {
        let frame_start = Instant::now();

        // Handle input (non-blocking)
        let mut key_press: Option<KeyEvent> = None;
        if poll(Duration::from_millis(1)).unwrap_or(false)
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
        if screen.update(key_press, elapsed) {
            break;
        }

        // Render
        renderer.render(screen.produce_frame(), last_fps)?;

        // Sleep for remaining time to maintain target FPS
        let frame_time = frame_start.elapsed();
        if frame_time < FRAME_DURATION {
            std::thread::sleep(FRAME_DURATION - frame_time);
        }

        // Calculate how long this iteration took
        elapsed = frame_start.elapsed();
        last_fps = 1_000_000u128
            .checked_div(elapsed.as_micros())
            .unwrap_or(1000);
    }

    Ok(())
}
