use std::time::Duration;

use crossterm::{event::KeyEvent, style::Color};

use crate::{
    HEIGHT, Screen, WIDTH,
    render::{self, Tile},
};

pub struct Title {
    latent_f: f64,
    pos: usize,
}

impl Title {
    pub fn new() -> Self {
        Title {
            latent_f: 0.0,
            pos: 0,
        }
    }
}

impl Screen for Title {
    fn update(&mut self, _: Option<KeyEvent>, elapsed: Duration) -> (bool, bool) {
        const SPIN_RATE: f64 = 500_f64; // 500ms to move between stages
        self.latent_f += elapsed.as_millis() as f64;

        // If the game has passed the next point before spinning, animate to the next stage and
        // remove the latent buildup of time:
        if self.latent_f < SPIN_RATE {
            return (false, false);
        }

        self.pos += 1;
        if self.pos > 3 {
            // Can't have more than position 3
            self.pos = 0;
        }
        self.latent_f -= SPIN_RATE;

        return (true, false);
    }

    fn produce_frame(&self) -> render::Frame<WIDTH, HEIGHT> {
        let mut frame = [[Tile {
            c: ' ',
            color: Color::White,
        }; HEIGHT]; WIDTH];
        let c = match self.pos {
            0 => '/',
            1 => '-',
            2 => '\\',
            3 => '|',
            _ => unreachable!(),
        };

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let color = match (x + y) % 4 {
                    0 => Color::Green,
                    _ => Color::White,
                };
                frame[x][y] = Tile { c, color: color };
            }
        }

        // Draw the title
        const TITLE: &[u8] = "RYN - A POOR ATTEMPT AT A GAME".as_bytes();
        const TITLE_START: (usize, usize) = (25, 15); // Where the top left of the corner is of the title

        for (i, c) in TITLE.iter().enumerate() {
            // The title is a single string, need to calculate
            // which character in 2d refers to the 1d value:
            frame[i + TITLE_START.0][TITLE_START.1] = Tile {
                c: *c as char,
                color: Color::White,
            };
        }

        frame
    }
}
