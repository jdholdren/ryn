use std::time::Duration;

use crossterm::{event::KeyCode, event::KeyEvent, style::Color};

use crate::{
    HEIGHT, Screen, WIDTH,
    location::{Location, TileType},
    render::{self, Tile},
};

pub struct Overworld {
    name: String,
    location: Location,
    player_pos: (usize, usize),
}

impl Overworld {
    pub fn new(name: String, location: Location) -> Self {
        let player_pos = location.spawn;
        Overworld {
            name,
            location,
            player_pos,
        }
    }

    fn can_move_to(&self, x: usize, y: usize) -> bool {
        x < self.location.width
            && y < self.location.height
            && matches!(self.location.grid[x][y], TileType::Floor)
    }
}

impl Screen for Overworld {
    fn update(&mut self, event: Option<KeyEvent>, _: Duration) -> bool {
        let Some(key) = event else {
            return false;
        };

        let (px, py) = self.player_pos;
        let (nx, ny) = match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return true,
            KeyCode::Char('w') | KeyCode::Up if py > 0 => (px, py - 1),
            KeyCode::Char('s') | KeyCode::Down => (px, py + 1),
            KeyCode::Char('a') | KeyCode::Left if px > 0 => (px - 1, py),
            KeyCode::Char('d') | KeyCode::Right => (px + 1, py),
            _ => return false,
        };

        if self.can_move_to(nx, ny) {
            self.player_pos = (nx, ny);
        }

        false
    }

    fn produce_frame(&self) -> render::Frame<WIDTH, HEIGHT> {
        let mut frame = [[Tile {
            c: ' ',
            color: Color::White,
        }; HEIGHT]; WIDTH];

        // Render location tiles into the frame
        let render_w = self.location.width.min(WIDTH);
        let render_h = self.location.height.min(HEIGHT);
        for x in 0..render_w {
            for y in 0..render_h {
                let tile = match self.location.grid[x][y] {
                    TileType::Wall => Tile {
                        c: '#',
                        color: Color::White,
                    },
                    TileType::Floor => Tile {
                        c: '.',
                        color: Color::DarkGrey,
                    },
                    TileType::Table => Tile {
                        c: '=',
                        color: Color::DarkYellow,
                    },
                };
                frame[x][y] = tile;
            }
        }

        // Draw room name at top of frame
        for (i, c) in self.name.chars().enumerate() {
            let x = i + 2;
            if x >= WIDTH {
                continue;
            }
            frame[x][0] = Tile {
                c,
                color: Color::Cyan,
            };
        }

        // Draw player
        frame[self.player_pos.0][self.player_pos.1] = Tile {
            c: '@',
            color: Color::Yellow,
        };

        frame
    }
}
