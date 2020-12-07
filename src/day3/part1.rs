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

const TOBOGGAN_PATTERN: (usize, usize) = (3, 1);

fn main() -> Result<(), Box<dyn Error>> {
    let map = parse_map()?;
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut tree_counter: usize = 0;
    while let Some(tile) = map.get_tile_at(x, y) {
        if *tile == Tile::Tree {
            tree_counter += 1;
        }
        x += TOBOGGAN_PATTERN.0;
        y += TOBOGGAN_PATTERN.1;
    }
    println!("{}", tree_counter);
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
