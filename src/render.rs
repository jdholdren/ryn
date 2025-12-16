// The rendering logic lives here, giving us a way to render a frame of tiles
// in the terminal in a performant way.

use std::io::{Result, Write, stdout};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
};

pub type Frame<const W: usize, const H: usize> = [[char; H]; W];

pub struct Renderer<const W: usize, const H: usize> {
    previous: Option<Frame<W, H>>,
}

impl<const W: usize, const H: usize> Renderer<W, H> {
    pub fn new() -> Self {
        Self { previous: None }
    }

    pub fn render(&mut self, next: Frame<W, H>, fps: u128) -> Result<()> {
        let mut stdout = stdout();

        if self.previous.is_none() {
            // Clear screen and move cursor to top-left
            stdout.queue(Clear(ClearType::All))?.queue(MoveTo(0, 0))?;
        }

        // Calculate diffs between new frame and previous
        for (x, col) in next.iter().enumerate() {
            for (y, c) in col.iter().enumerate() {
                let mut prev_char = ' ';
                if let Some(prev) = self.previous.as_ref() {
                    prev_char = prev[x][y];
                }
                if prev_char == *c {
                    continue;
                }

                stdout
                    .queue(MoveTo(x as u16, y as u16))?
                    .queue(SetForegroundColor(Color::White))?
                    .queue(Print(*c))?;
            }
        }

        // DEBUG: Draw UI info
        stdout
            .queue(MoveTo(2, (H - 1) as u16))?
            .queue(SetForegroundColor(Color::Cyan))?
            .queue(Print(format!(
                "WASD/Arrows: Move | Q/ESC: Quit | {:2}fps",
                fps
            )))?
            .flush()?;

        self.previous = Some(next);

        Ok(())
    }
}
