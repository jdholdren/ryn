mod movement;
mod renderable;

use std::fs;
use std::path::Path;
use std::time::Duration;

use crossterm::{event::KeyCode, event::KeyEvent, style::Color};
use serde::Deserialize;

use crate::{
    HEIGHT, Screen, WIDTH,
    location::{Location, TileType},
    overworld::movement::{MovementSystem, Position, Velocity},
    overworld::renderable::Renderable,
    overworld::renderable::RenderableData,
    render::{self, Tile},
};

pub struct Overworld {
    entities: Vec<Entity>,
    location: Location,
}

#[derive(Deserialize)]
struct EntityData {
    id: usize,
    #[serde(default)]
    position: Option<Position>,
    renderable: Option<RenderableData>,
}

#[derive(Deserialize)]
struct EntitiesFile {
    entities: Vec<EntityData>,
}

impl Overworld {
    pub fn new(location: Location, entities_path: &Path) -> Self {
        let contents = fs::read_to_string(entities_path)
            .unwrap_or_else(|e| panic!("Failed to read entities file {:?}: {}", entities_path, e));
        let file: EntitiesFile = serde_json::from_str(&contents)
            .unwrap_or_else(|e| panic!("Failed to parse entities file {:?}: {}", entities_path, e));

        let entities = file
            .entities
            .into_iter()
            .map(|data| Entity {
                id: data.id,
                pos: data.position,
                velocity: None,
                renderable: data.renderable.map(|r| r.into_renderable()),
            })
            .collect();

        Overworld { entities, location }
    }
}

impl Screen for Overworld {
    fn update(&mut self, event: Option<KeyEvent>, _: Duration) {
        // Phase 1: Map input to player velocity
        if let Some(key) = event {
            let vel = match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return,
                KeyCode::Char('w') | KeyCode::Up => (0, -1),
                KeyCode::Char('s') | KeyCode::Down => (0, 1),
                KeyCode::Char('a') | KeyCode::Left => (-1, 0),
                KeyCode::Char('d') | KeyCode::Right => (1, 0),
                _ => (0, 0),
            };
            self.entities[0].velocity = Some(Velocity { vector: vel });
        }

        // Phase 2: Run systems
        MovementSystem::update(self);

        // Phase 3: Clear one-shot velocities
        self.entities[0].velocity = None;
    }

    fn produce_frame(&self) -> render::Frame<WIDTH, HEIGHT> {
        const VIEWPORT_HEIGHT: isize = 20;
        const VIEWPORT_WIDTH: isize = 70;
        let max_width = self.location.width as isize;
        let max_height = self.location.height as isize;

        let mut frame = render::Frame::<WIDTH, HEIGHT>::blank();

        // Rendering the viewport involves rendering location tiles
        // within the viewport. These are the tiles from the
        // top left: (player_pos.x - (VIEWPORT_WIDTH / 2), player_pos.y - (VIEWPORT_HEIGHT / 2))
        // bottom_right: (player_pos.x + (VIEWPORT_WIDTH / 2), player_pos.y + (VIEWPORT_HEIGHT / 2))
        //
        // Both are basically the calc of the center point (the player), plus/minus half the viewport dimensions
        let player_pos = self.entities.get(0).unwrap().pos.clone().unwrap();
        let top_left = (
            player_pos.x - (VIEWPORT_WIDTH / 2),
            player_pos.y - (VIEWPORT_HEIGHT / 2),
        );
        let bottom_right = (
            player_pos.x + (VIEWPORT_WIDTH / 2),
            player_pos.y + (VIEWPORT_HEIGHT / 2),
        );

        // Render location tiles
        for x in top_left.0..bottom_right.0 {
            if x < 0 || x > max_width - 1 {
                continue;
            }

            for y in top_left.1..bottom_right.1 {
                if y < 0 || y > max_height - 1 {
                    continue;
                }

                let tile = match self.location.grid[x as usize][y as usize] {
                    TileType::Wall => Tile {
                        c: '#',
                        color: Color::DarkGrey,
                    },
                    TileType::Floor => Tile {
                        c: '.',
                        color: Color::DarkGrey,
                    },
                    TileType::Table => Tile {
                        c: '=',
                        color: Color::Yellow,
                    },
                };

                // Offset the position by the top left
                frame.set((x - top_left.0) as usize, (y - top_left.1) as usize, tile);
            }
        }

        // TODO: Transform this from the top left position
        // Render entities that have a position and renderable
        // for entity in &self.entities {
        //     let (Some(pos), Some(r)) = (&entity.pos, &entity.renderable) else {
        //         continue;
        //     };

        //     frame.set(
        //         pos.x as usize,
        //         pos.y as usize,
        //         Tile {
        //             c: r.c,
        //             color: r.color,
        //         },
        //     );
        // }

        // Render the player last, in the middle
        frame.set(
            VIEWPORT_WIDTH as usize / 2,
            VIEWPORT_HEIGHT as usize / 2,
            Tile {
                c: '@',
                color: Color::Yellow,
            },
        );

        // The border for the viewport
        frame.draw_box(
            0,
            0,
            VIEWPORT_WIDTH as usize,
            VIEWPORT_HEIGHT as usize,
            Color::White,
        );

        frame
    }
}

struct Entity {
    id: usize,

    // Movement
    pos: Option<Position>,
    velocity: Option<Velocity>,

    // Rendering
    renderable: Option<Renderable>,
}

pub trait System {
    fn update(ow: &mut Overworld);
}
