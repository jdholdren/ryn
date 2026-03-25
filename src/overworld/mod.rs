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
    player: bool,
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

        Overworld {
            entities,
            location,
        }
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
        let mut frame = render::Frame::<WIDTH, HEIGHT>::blank();

        // Render location tiles
        for x in 0..self.location.width.min(WIDTH) {
            for y in 0..self.location.height.min(HEIGHT) {
                let tile = match self.location.grid[x][y] {
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
                frame.set(x, y, tile);
            }
        }

        // Render entities that have a position and renderable
        for entity in &self.entities {
            let (Some(pos), Some(r)) = (&entity.pos, &entity.renderable) else {
                continue;
            };

            frame.set(pos.x, pos.y, Tile { c: r.c, color: r.color });
        }

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
