use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
    Table,
}

pub struct Location {
    pub grid: Vec<Vec<TileType>>,
    pub width: usize,
    pub height: usize,
    pub spawn: (usize, usize),
}

fn parse_location(contents: &str) -> Location {
    let lines: Vec<&str> = contents.lines().collect();
    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut grid = vec![vec![TileType::Floor; height]; width];
    let mut spawn = (0, 0);

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => grid[x][y] = TileType::Wall,
                '=' => grid[x][y] = TileType::Table,
                '@' => {
                    grid[x][y] = TileType::Floor;
                    spawn = (x, y);
                }
                _ => grid[x][y] = TileType::Floor,
            }
        }
    }

    Location {
        grid,
        width,
        height,
        spawn,
    }
}

pub fn load_locations(dir: &Path) -> HashMap<String, Location> {
    let mut locations = HashMap::new();

    let entries = fs::read_dir(dir).unwrap_or_else(|e| panic!("Failed to read maps directory {:?}: {}", dir, e));

    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("txt") {
            let name = path
                .file_stem()
                .expect("Map file has no stem")
                .to_str()
                .expect("Map filename is not valid UTF-8")
                .to_string();
            let contents = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("Failed to read map file {:?}: {}", path, e));
            let location = parse_location(&contents);
            assert!(location.width > 0 && location.height > 0, "Map {:?} is empty", path);
            locations.insert(name, location);
        }
    }

    locations
}
