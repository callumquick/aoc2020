/// Solution to Advent of Code Challenge Day 05.
use aoc2020::{get_day_input, print_elapsed_time};
use std::collections::HashSet;
use std::io;
use std::str::FromStr;

const DAYNUM: &'static str = "05";
type ChallengeData = Vec<BoardingPass>;
type ChallengeOut = usize;

#[derive(Debug)]
struct BoardingPass {
    row: usize,
    col: usize,
    seat_id: usize,
}

impl FromStr for BoardingPass {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row_ch: Vec<_> = s.chars().take(7).collect();
        let col_ch: Vec<_> = s.chars().skip(7).take(3).collect();
        let row: usize = row_ch
            .iter()
            .map(|ch| match ch {
                'F' => 0,
                'B' => 1,
                _ => panic!("Invalid boarding pass data"),
            })
            .fold(0, |acc, x| 2 * acc + x);
        let col: usize = col_ch
            .iter()
            .map(|ch| match ch {
                'L' => 0,
                'R' => 1,
                _ => panic!("Invalid boarding pass data"),
            })
            .fold(0, |acc, x| 2 * acc + x);
        let seat_id: usize = (row * 8) + col;
        Ok(Self { row, col, seat_id })
    }
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    // Find the max seat ID.
    data.iter().map(|p| p.seat_id).max()
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    let ids: HashSet<_> = data.iter().map(|p| p.seat_id).collect();
    // Find our seat: for each taken seat, check if the seat two seats over is taken, but the seat
    // one over is not. This would be our seat.
    ids.iter()
        .find(|id| !ids.contains(&(*id + 1)) && ids.contains(&(*id + 2)))
        .map(|id| *id + 1)
}

fn get_data(input: String) -> Result<ChallengeData, io::Error> {
    input.lines().map(|s| s.parse()).collect()
}

fn main() -> Result<(), io::Error> {
    let input = get_day_input(DAYNUM);
    let data = get_data(input)?;
    println!("Day {}:", DAYNUM);
    println!("==========");
    println!(
        "Part one: {}",
        print_elapsed_time(|| part_one(&data)).expect("No solution found for part one"),
    );
    println!(
        "Part two: {}",
        print_elapsed_time(|| part_two(&data)).expect("No solution found for part two"),
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: String = "FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"
            .to_string();
        let data = get_data(input).expect("Couldn't convert test input");

        // Check all of the boarding pass data calculated correctly.
        assert_eq!(data[0].row, 44);
        assert_eq!(data[0].col, 5);
        assert_eq!(data[0].seat_id, 357);

        assert_eq!(data[1].row, 70);
        assert_eq!(data[1].col, 7);
        assert_eq!(data[1].seat_id, 567);

        assert_eq!(data[2].row, 14);
        assert_eq!(data[2].col, 7);
        assert_eq!(data[2].seat_id, 119);

        assert_eq!(data[3].row, 102);
        assert_eq!(data[3].col, 4);
        assert_eq!(data[3].seat_id, 820);

        assert_eq!(part_one(&data), Some(820));
    }
}
