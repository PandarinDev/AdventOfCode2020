use std::error::Error;

#[derive(PartialEq)]
enum Tile {
    Open,
    Tree
}

struct Map {
    width: usize,
    tiles: Vec<Tile>
}

impl Map {

    fn get_tile_at(&self, x: usize, y: usize) -> Option<&Tile> {
        // Note: The map repeats horizontally, which we can emulate with x %/
        self.tiles.get(y * self.width + x % self.width)
    }

}

#[derive(Debug)]
struct MapParsingError;

impl Error for MapParsingError {}

impl std::fmt::Display for MapParsingError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse map.")
    }

}

const TOBOGGAN_PATTERNS: [(usize, usize); 5] = [
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2)
];

fn main() -> Result<(), Box<dyn Error>> {
    let map = parse_map()?;
    let mut tree_counters = Vec::<usize>::new();
    for pattern in &TOBOGGAN_PATTERNS {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut tree_counter: usize = 0;
        while let Some(tile) = map.get_tile_at(x, y) {
            if *tile == Tile::Tree {
                tree_counter += 1;
            }
            x += pattern.0;
            y += pattern.1;
        }
        tree_counters.push(tree_counter);
    }
    let result = tree_counters.iter().fold(1, |a, b| a * b);
    println!("{}", result);
    Ok(())
}

fn parse_map() -> Result<Map, Box<dyn Error>> {
    let file_path = std::env::current_exe()?.join("../../../resources/day3.txt");
    let input = std::fs::read_to_string(file_path)?;
    let lines: Vec<_> = input.lines().collect();
    let width = lines[0].chars().count();
    let mut tiles: Vec<Tile> = vec![];
    for line in lines {
        for character in line.chars() {
            let tile = match character {
                '.' => Tile::Open,
                '#' => Tile::Tree,
                _ => return Err(MapParsingError.into())
            };
            tiles.push(tile);
        }
    }
    Ok(Map { width, tiles })
}
