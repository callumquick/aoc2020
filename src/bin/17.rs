/// Solution to Advent of Code Challenge Day 17.
use aoc2020::{get_day_input, print_elapsed_time};
use std::collections::{HashMap, HashSet};
use std::io;
use std::str::FromStr;

const DAYNUM: &'static str = "17";
type ChallengeData = InitialState;
type ChallengeOut = usize;

type Position = Vec<i32>;

fn add_positions(p1: &Position, p2: &Position) -> Position {
    p1.iter().zip(p2.iter()).map(|(x1, x2)| x1 + x2).collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Cube {
    Active,
    Inactive,
}

impl Cube {
    fn from_ch(ch: char) -> Self {
        match ch {
            '.' => Self::Inactive,
            '#' => Self::Active,
            _ => panic!("Invalid character in the initial state data"),
        }
    }
}

type Row = Vec<Cube>;

struct InitialState {
    rows: Vec<Row>,
}

impl FromStr for InitialState {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rows: s
                .lines()
                .map(|s| s.chars().map(|ch| Cube::from_ch(ch)).collect())
                .collect(),
        })
    }
}

impl InitialState {
    fn get_active_positions(&self) -> HashSet<(i32, i32)> {
        let mut active_positions = HashSet::new();
        for (y, row) in self.rows.iter().enumerate() {
            for (x, cube) in row.iter().enumerate() {
                match cube {
                    Cube::Active => {
                        active_positions.insert((x as i32, y as i32));
                    }
                    _ => continue,
                }
            }
        }
        active_positions
    }
}

#[derive(Clone, Debug)]
struct State {
    cubes: HashSet<Position>,
    dimensions: usize,
}

impl State {
    fn from_initial(initial: &InitialState, dimensions: usize) -> Self {
        assert!(dimensions >= 2);
        let mut cubes = HashSet::new();
        for (x, y) in initial.get_active_positions() {
            let mut dimension_position = vec![0i32; dimensions];
            dimension_position[0] = x;
            dimension_position[1] = y;
            cubes.insert(dimension_position);
        }
        State { cubes, dimensions }
    }

    fn cycle(&mut self) {
        // Create a mapping of all positions in the space that have active neighbours to the number
        // of active neighbours they have. These positions include inactive cubes that have active
        // neighbours by adding 1 active neighbour to nearby positions for each active cube, but
        // also include active cubes that have no active neighbours.
        let position_to_active_neighbours = self.get_position_to_active_neighbours();

        for (position, neighbours) in position_to_active_neighbours {
            if self.cubes.contains(&position) && !(2..=3).contains(&neighbours) {
                self.cubes.remove(&position);
            } else if neighbours == 3 {
                self.cubes.insert(position);
            }
        }
    }

    fn get_neighbour_directions(&self) -> Vec<Position> {
        let mut neighbour_directions = vec![vec![0i32; self.dimensions]];
        // For each dimension, add the "-1" and "+1" variants in that dimensions to the already
        // calculated neighbour directions.
        for dimension in 0..self.dimensions {
            let mut new_directions = Vec::new();
            for direction in &neighbour_directions {
                let mut new_direction_up = direction.clone();
                new_direction_up[dimension] = 1;
                new_directions.push(new_direction_up);
                let mut new_direction_down = direction.clone();
                new_direction_down[dimension] = -1;
                new_directions.push(new_direction_down);
            }
            neighbour_directions.extend(new_directions);
        }
        // This produces all direction including the starting "0" vector, which doesn't point to any
        // neighbours but the self: remove this.
        neighbour_directions.remove(0);
        neighbour_directions
    }

    fn get_neighbours(&self, p: &Position) -> Vec<Position> {
        self.get_neighbour_directions()
            .iter()
            .map(|direction| add_positions(p, direction))
            .collect()
    }

    fn get_position_to_active_neighbours(&self) -> HashMap<Position, u32> {
        let mut position_to_active_neighbours = HashMap::new();
        for active_pos in &self.cubes {
            // Ensure active cubes are placed in the mapping, even if they have 0 active neighbours.
            position_to_active_neighbours
                .entry(active_pos.to_vec())
                .or_insert(0);
            for neighbour in self.get_neighbours(active_pos) {
                *position_to_active_neighbours.entry(neighbour).or_insert(0) += 1;
            }
        }
        position_to_active_neighbours
    }
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    let mut state = State::from_initial(data, 3);
    for _ in 0..6 {
        state.cycle();
    }
    Some(state.cubes.len())
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    let mut state = State::from_initial(data, 4);
    for _ in 0..6 {
        state.cycle();
    }
    Some(state.cubes.len())
}

fn get_data(input: String) -> Result<ChallengeData, io::Error> {
    input.parse()
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
        let input = ".#.
..#
###"
        .to_string();

        let data = get_data(input.to_string()).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data), Some(112));
        assert_eq!(part_two(&data), Some(848));
    }
}
