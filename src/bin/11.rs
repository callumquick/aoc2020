/// Solution to Advent of Code Challenge Day 11.
use aoc2020::{get_day_input, print_elapsed_time};
use std::io;
use std::str::FromStr;

type Number = u32;

const DAYNUM: &'static str = "11";
type ChallengeData = Vec<Row>;
type ChallengeOut = Number;

#[derive(PartialEq, Eq, Clone)]
enum Tile {
    Floor,
    Seat(bool),
}

impl Tile {
    fn from_ch(ch: char) -> Option<Self> {
        match ch {
            '.' => Some(Self::Floor),
            'L' => Some(Self::Seat(false)),
            '#' => Some(Self::Seat(true)),
            _ => None,
        }
    }

    fn occupied(&self) -> bool {
        match self {
            Self::Seat(true) => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Row {
    tiles: Vec<Tile>,
    length: usize,
}

impl FromStr for Row {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<_> = s
            .chars()
            .map(|ch| Tile::from_ch(ch))
            .collect::<Option<_>>()
            .unwrap();
        Ok(Row {
            length: tiles.len(),
            tiles,
        })
    }
}

/// Gets the number of occupied seats immediately around a given seat.
fn get_occupied_adjacent(plan: &ChallengeData, row: usize, col: usize) -> Number {
    let mut adjacent: Vec<&Tile> = Vec::new();
    let left = col > 0;
    let right = col < plan[0].length - 1;
    let up = row > 0;
    let down = row < plan.len() - 1;

    // Left
    if left {
        adjacent.push(&plan[row].tiles[col - 1]);
        // Up left
        if up {
            adjacent.push(&plan[row - 1].tiles[col - 1]);
        }
        // Down left
        if down {
            adjacent.push(&plan[row + 1].tiles[col - 1]);
        }
    }
    // Right
    if right {
        adjacent.push(&plan[row].tiles[col + 1]);
        // Up right
        if up {
            adjacent.push(&plan[row - 1].tiles[col + 1]);
        }
        // Down right
        if down {
            adjacent.push(&plan[row + 1].tiles[col + 1]);
        }
    }
    // Up
    if up {
        adjacent.push(&plan[row - 1].tiles[col]);
    }
    // Down
    if down {
        adjacent.push(&plan[row + 1].tiles[col]);
    }

    adjacent.iter().map(|tile| tile.occupied() as Number).sum()
}

/// Gets the number of occupied seats in the sightlines of a given seat.
fn get_occupied_sightline(plan: &ChallengeData, row: usize, col: usize) -> Number {
    let mut sightlined: Vec<&Tile> = Vec::new();

    let seek_directions: [(isize, isize); 8] = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for direction in &seek_directions {
        let mut seek = (row as isize + direction.0, col as isize + direction.1);
        while seek.0 >= 0
            && (seek.0 as usize) < plan.len()
            && seek.1 >= 0
            && (seek.1 as usize) < plan[0].length
        {
            match plan[seek.0 as usize].tiles[seek.1 as usize] {
                Tile::Seat(_) => {
                    sightlined.push(&plan[seek.0 as usize].tiles[seek.1 as usize]);
                    break;
                }
                _ => (),
            }
            seek = (seek.0 + direction.0, seek.1 + direction.1);
        }
    }

    sightlined
        .iter()
        .map(|tile| tile.occupied() as Number)
        .sum()
}

/// Apply the seat change rules to produce a new floorplan according to part one rules
fn iterate_seat_changes_v1(from: &ChallengeData) -> ChallengeData {
    let mut to = from.to_vec();

    for (row_idx, row) in from.iter().enumerate() {
        for (col_idx, tile) in row.tiles.iter().enumerate() {
            match tile {
                Tile::Floor => continue,
                Tile::Seat(false) => {
                    // If seat is empty and no adjacent seats are occupied, it is filled.
                    if get_occupied_adjacent(from, row_idx, col_idx) == 0 {
                        to[row_idx].tiles[col_idx] = Tile::Seat(true);
                    }
                }
                Tile::Seat(true) => {
                    // If seat is occupied and four or more adjacent are too, it is vacated.
                    if get_occupied_adjacent(from, row_idx, col_idx) >= 4 {
                        to[row_idx].tiles[col_idx] = Tile::Seat(false);
                    }
                }
            }
        }
    }

    to
}

/// Apply the seat change rules to produce a new floorplan according to part two rules
fn iterate_seat_changes_v2(from: &ChallengeData) -> ChallengeData {
    let mut to = from.to_vec();

    for (row_idx, row) in from.iter().enumerate() {
        for (col_idx, tile) in row.tiles.iter().enumerate() {
            match tile {
                Tile::Floor => continue,
                Tile::Seat(false) => {
                    // If seat is empty and no seats in sightline are occupied, it is filled.
                    if get_occupied_sightline(from, row_idx, col_idx) == 0 {
                        to[row_idx].tiles[col_idx] = Tile::Seat(true);
                    }
                }
                Tile::Seat(true) => {
                    // If seat is occupied and five or more in sightline are too, it is vacated.
                    if get_occupied_sightline(from, row_idx, col_idx) >= 5 {
                        to[row_idx].tiles[col_idx] = Tile::Seat(false);
                    }
                }
            }
        }
    }

    to
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    let mut from = data.to_vec();
    let mut to = iterate_seat_changes_v1(&from);

    while to != from {
        from = to;
        to = iterate_seat_changes_v1(&from);
    }

    Some(
        to.iter()
            .map(|row| {
                row.tiles
                    .iter()
                    .map(|tile| tile.occupied() as Number)
                    .sum::<Number>()
            })
            .sum::<Number>(),
    )
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    let mut from = data.to_vec();
    let mut to = iterate_seat_changes_v2(&from);

    while to != from {
        from = to;
        to = iterate_seat_changes_v2(&from);
    }

    Some(
        to.iter()
            .map(|row| {
                row.tiles
                    .iter()
                    .map(|tile| tile.occupied() as Number)
                    .sum::<Number>()
            })
            .sum::<Number>(),
    )
}

fn get_data(input: String) -> io::Result<ChallengeData> {
    input.lines().map(|s| s.parse()).collect()
}

fn main() -> io::Result<()> {
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
        let input: String = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            .to_string();
        let data = get_data(input).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data), Some(37));
        assert_eq!(part_two(&data), Some(26));
    }
}
