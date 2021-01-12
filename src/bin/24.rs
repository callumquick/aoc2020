/// Solution to Advent of Code Challenge Day 24.
use aoc2020::{get_day_input, print_elapsed_time};
use std::collections::HashSet;
use std::io;
use std::str::FromStr;

const DAYNUM: &'static str = "24";
type ChallengeData = Vec<Instruction>;
type ChallengeOut = usize;

/// Define the coordinate of a tile in the hexagonal grid as follows:
/// - Reference tile is (0, 0)
/// - Moving east is (1, 0), west is (-1, 0)
/// - Draw the lines of "constant vertical" diagonally across the hexagons from north-west to
///   south-east
/// - This makes moving north-west (0, 1) but north-east (1, 1)
/// - South-west is (-1, -1) and south-east (0, -1) (opposites of above)
/// This accounts for the hexagonal grid because although the tiles don't actually fit on these
/// lines on a graph, different steps to the same tile will still give the same coordinate and it is
/// unique to that tile.
///
///    / \   / \   / \
///   /   \ /   \ /   \
///  |     |     |     |
///  |-1,1 | 0,1 | 1,1 |
///  |     |     |     |
///   \   / \   / \   / \
///    \ /   \ /   \ /   \
///     |     |     |     |
///     |-1,0 | 0,0 | 1,0 |
///     |     |     |     |
///      \   / \   / \   /
///       \ /   \ /   \ /
///
type Coord = (i32, i32);

fn vec_add(vec1: Coord, vec2: Coord) -> Coord {
    (vec1.0 + vec2.0, vec1.1 + vec2.1)
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl FromStr for Direction {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "e" => Self::E,
            "se" => Self::SE,
            "sw" => Self::SW,
            "w" => Self::W,
            "nw" => Self::NW,
            "ne" => Self::NE,
            _ => panic!("Couldn't convert string into direction: {}", s),
        })
    }
}

impl Direction {
    fn unit_vec(&self) -> Coord {
        match self {
            Self::E => (1, 0),
            Self::W => (-1, 0),
            Self::NW => (0, 1),
            Self::NE => (1, 1),
            Self::SE => (0, -1),
            Self::SW => (-1, -1),
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    dirs: Vec<Direction>,
}

impl FromStr for Instruction {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars: Vec<char> = s.chars().rev().collect();
        let mut dirs = Vec::new();
        while !chars.is_empty() {
            let mut dir_str = String::new();
            dir_str.push(chars.pop().unwrap());
            // The directions South and North don't exist: and s or n is always followed by a
            // qualifier
            if dir_str == "s" || dir_str == "n" {
                dir_str.push(chars.pop().expect("Cannot have an S or N unqualified"));
            }
            dirs.push(dir_str.parse()?);
        }
        Ok(Self { dirs })
    }
}

impl Instruction {
    /// Convert the set of instructions to a final coordinate.
    fn to_coord(&self) -> Coord {
        let mut coord = (0, 0);

        for direction in &self.dirs {
            coord = vec_add(coord, direction.unit_vec());
        }

        coord
    }
}

/// Find all coords which are adjecent to this one (there will be 6 because hexagons are the
/// bestagons).
fn get_adjacent_coords(coord: Coord) -> [Coord; 6] {
    [
        vec_add(coord, Direction::E.unit_vec()),
        vec_add(coord, Direction::W.unit_vec()),
        vec_add(coord, Direction::NW.unit_vec()),
        vec_add(coord, Direction::NE.unit_vec()),
        vec_add(coord, Direction::SE.unit_vec()),
        vec_add(coord, Direction::SW.unit_vec()),
    ]
}

/// Generate the initial tileset from the given instructions.
fn get_initial_tiles(instructions: &ChallengeData) -> HashSet<Coord> {
    let mut black_tiles: HashSet<Coord> = HashSet::new();

    for instruction in instructions {
        let tile = instruction.to_coord();

        if !black_tiles.remove(&tile) {
            // Wasn't already flipped to black so insert it into the set of black tiles (if it was
            // already in the set it flips back to white and is already removed)
            black_tiles.insert(tile);
        }
    }

    black_tiles
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    Some(get_initial_tiles(data).len())
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    let mut black_tiles: HashSet<Coord> = get_initial_tiles(data);

    // Perform the 100 days of iterations.
    for _ in 0..100 {
        let mut new_tiles = HashSet::new();

        for tile in &black_tiles {
            let neighbours = get_adjacent_coords(*tile);
            let mut black_neighbours = 0;
            for neighbour in &neighbours {
                if black_tiles.contains(neighbour) {
                    black_neighbours += 1;
                }
            }

            // If a black tile has zero or more than 2 neighbours, flip to white (don't re-add it to
            // the new black tiles)
            if black_neighbours != 0 && black_neighbours <= 2 {
                new_tiles.insert(*tile);
            }

            // Use this black tile to try and find white tiles which have exactly 2 black tile
            // neighbours.
            for neighbour in &neighbours {
                if !black_tiles.contains(neighbour) {
                    // Is a white tile
                    let onward_neighbours = get_adjacent_coords(*neighbour);
                    let mut black_neighbours = 0;
                    for onward_neighbour in &onward_neighbours {
                        if black_tiles.contains(onward_neighbour) {
                            black_neighbours += 1;
                        }
                    }

                    if black_neighbours == 2 {
                        new_tiles.insert(*neighbour);
                    }
                }
            }
        }

        black_tiles = new_tiles;
    }

    Some(black_tiles.len())
}

fn get_data(input: String) -> Result<ChallengeData, io::Error> {
    input.lines().map(|s| s.parse()).collect()
}

fn main() -> Result<(), io::Error> {
    println!("Day {}:", DAYNUM);
    println!("==========");
    println!("Getting data...");
    let data = print_elapsed_time(|| get_data(get_day_input(DAYNUM)))?;
    println!("==========");
    println!("Solving part one...");
    let ans1 = print_elapsed_time(|| part_one(&data)).expect("No solution found for part one");
    println!("Answer: {}", ans1);
    println!("==========");
    println!("Solving part two...");
    let ans2 = print_elapsed_time(|| part_two(&data)).expect("No solution found for part two");
    println!("Answer: {}", ans2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"
            .to_string();

        let data = get_data(input.to_string()).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data), Some(10));
        assert_eq!(part_two(&data), Some(2208));
    }
}
