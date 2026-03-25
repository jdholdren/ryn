// The rendering logic lives here, giving us a way to render a frame of tiles
// in the terminal in a performant way.

use std::io::{Result, Write, stdout};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Tile {
    pub c: char,
    pub color: Color,
}

#[derive(Clone, Copy)]
pub struct Frame<const W: usize, const H: usize>([[Tile; H]; W]);

impl<const W: usize, const H: usize> Frame<W, H> {
    pub fn blank() -> Self {
        Frame([[Tile { c: ' ', color: Color::White }; H]; W])
    }

    pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
        if x < W && y < H {
            self.0[x][y] = tile;
        }
    }

    pub fn draw_text(&mut self, x: usize, y: usize, text: &str, color: Color) {
        for (i, c) in text.chars().enumerate() {
            let tx = x + i;
            if tx >= W || y >= H {
                break;
            }
            self.0[tx][y] = Tile { c, color };
        }
    }

    pub fn draw_box(&mut self, x: usize, y: usize, w: usize, h: usize, color: Color) {
        if w < 2 || h < 2 {
            return;
        }
        let (x2, y2) = (x + w - 1, y + h - 1);
        for px in x..=x2 {
            for py in y..=y2 {
                if px >= W || py >= H {
                    continue;
                }
                let c = match (px == x || px == x2, py == y || py == y2) {
                    (true, true) => '+',
                    (false, true) => '-',
                    (true, false) => '|',
                    _ => continue,
                };
                self.0[px][py] = Tile { c, color };
            }
        }
    }
}

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

        // Calculate diffs between new frame and previous, iterating row-major
        // (top to bottom, left to right) to minimize cursor jumping
        let mut current_color: Option<Color> = None;
        for y in 0..H {
            for x in 0..W {
                let tile = &next.0[x][y];
                let prev_tile = self
                    .previous
                    .as_ref()
                    .map(|p| &p.0[x][y])
                    .unwrap_or(&Tile {
                        c: ' ',
                        color: Color::White,
                    });
                if prev_tile == tile {
                    continue;
                }

                stdout.queue(MoveTo(x as u16, y as u16))?;
                if current_color != Some(tile.color) {
                    stdout.queue(SetForegroundColor(tile.color))?;
                    current_color = Some(tile.color);
                }
                stdout.queue(Print(tile.c))?;
            }
        }

        // DEBUG: Draw UI info
        stdout
            .queue(MoveTo(2, (H - 1) as u16))?
            .queue(SetForegroundColor(Color::Cyan))?
            .queue(Print(format!("CTRL-C: Quit | {:2}fps", fps)))?
            .flush()?;

        self.previous = Some(next);

        Ok(())
    }
}
