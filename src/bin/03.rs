/// Solution to Advent of Code Challenge Day 03.
use aoc2020::{get_day_input, print_elapsed_time};
use std::io;
use std::str::FromStr;

/// Each tile is either a tree or open space.
#[derive(PartialEq)]
enum Tile {
    Open,
    Tree,
}

impl Tile {
    fn from_ch(ch: char) -> Option<Self> {
        match ch {
            '.' => Some(Self::Open),
            '#' => Some(Self::Tree),
            _ => None,
        }
    }
}

struct TileRow {
    row: Vec<Tile>,
}

impl FromStr for TileRow {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            row: s
                .chars()
                .map(|c| {
                    Tile::from_ch(c).ok_or(Self::Err::new(
                        io::ErrorKind::Other,
                        "Invalid character hit reading input",
                    ))
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

/// Parse a list of TileRows from an input string.
fn get_tile_rows(input: String) -> Result<Vec<TileRow>, io::Error> {
    input.lines().map(|s| s.parse()).collect::<Result<_, _>>()
}

/// O(N) in the length of the data to perform the slope calculations and hit the
/// right tiles to check if tree.
fn part_one(data: &Vec<TileRow>, right: usize, down: usize) -> Option<u64> {
    if data.is_empty() {
        return None;
    }

    // All lines are guaranteed to be the same length.
    let length = data[0].row.len();

    let mut row_idx: usize = 0;
    let mut tree_count: u64 = 0;
    let mut col_idx: usize = 0;

    // While there is enough space for the next hop.
    while row_idx + down < data.len() {
        col_idx = (col_idx + right) % length;
        row_idx += down;
        if data[row_idx].row[col_idx] != Tile::Tree {
            continue;
        }
        tree_count += 1;
    }

    Some(tree_count)
}

/// O(NM) in the length of the data to the number of slopes given to perform the
/// slope calculations and hit the right tiles to check if tree.
fn part_two(data: &Vec<TileRow>, slopes: &[(usize, usize)]) -> Option<u64> {
    slopes
        .iter()
        .map(|(right, down)| part_one(data, *right, *down))
        .product::<Option<_>>()
}

fn main() -> Result<(), io::Error> {
    let input = get_day_input("03");
    let data = get_tile_rows(input)?;
    println!("Day 03:");
    println!("==========");
    println!(
        "Part one: {}",
        print_elapsed_time(|| part_one(&data, 3, 1)).expect("No solution found for part one"),
    );
    println!(
        "Part two: {}",
        print_elapsed_time(|| part_two(&data, &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2),]))
            .expect("No solution found for part two"),
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: String = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            .to_string();
        let data = get_tile_rows(input).expect("Couldn't convert test input");

        // Check each gives the right answer.
        assert_eq!(part_one(&data, 3, 1), Some(7));
        assert_eq!(
            part_two(&data, &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2),]),
            Some(2 * 7 * 3 * 4 * 2)
        );
    }
}
