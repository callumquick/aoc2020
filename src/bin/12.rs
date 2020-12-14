/// Solution to Advent of Code Challenge Day 12.
use aoc2020::{get_day_input, print_elapsed_time};
use std::io;
use std::str::FromStr;

type Number = u32;

const DAYNUM: &'static str = "12";
type ChallengeData = Vec<Instruction>;
type ChallengeOut = Number;

enum Direction {
    North,
    East,
    South,
    West,
    Forward,
    Left,
    Right,
}

impl Direction {
    fn from_char(ch: char) -> Self {
        match ch {
            'N' => Self::North,
            'E' => Self::East,
            'S' => Self::South,
            'W' => Self::West,
            'F' => Self::Forward,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Unknown character for direction in instruction set"),
        }
    }

    fn direction_vector(&self) -> (i32, i32) {
        match self {
            Self::North => (0, 1),
            Self::East => (1, 0),
            Self::South => (0, -1),
            Self::West => (-1, 0),
            _ => panic!("Facing does not have a direction"),
        }
    }
}

struct Instruction {
    dir: Direction,
    num: i32,
}

impl FromStr for Instruction {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            dir: Direction::from_char(s.chars().nth(0).unwrap()),
            num: s[1..].parse().unwrap(),
        })
    }
}

struct Ship {
    x: i32,
    y: i32,
    facing: Direction,
}

struct Waypoint {
    x: i32,
    y: i32,
}

impl Ship {
    fn taxicab_distance(&self) -> Number {
        (self.x.abs() + self.y.abs()) as Number
    }

    fn update_v1(&mut self, instruction: &Instruction) {
        match instruction.dir {
            Direction::North => self.y += instruction.num,
            Direction::East => self.x += instruction.num,
            Direction::South => self.y -= instruction.num,
            Direction::West => self.x -= instruction.num,
            Direction::Forward => {
                let dir_vector = self.facing.direction_vector();
                self.x += dir_vector.0 * instruction.num;
                self.y += dir_vector.1 * instruction.num;
            }
            Direction::Right => {
                let num_turns: u32 = instruction.num as u32 / 90;
                for _ in 0..num_turns {
                    self.facing = match self.facing {
                        Direction::North => Direction::East,
                        Direction::East => Direction::South,
                        Direction::South => Direction::West,
                        Direction::West => Direction::North,
                        _ => panic!("Ship cannot face non-cardinal direction"),
                    };
                }
            }
            Direction::Left => {
                let num_turns: u32 = instruction.num as u32 / 90;
                for _ in 0..num_turns {
                    self.facing = match self.facing {
                        Direction::North => Direction::West,
                        Direction::West => Direction::South,
                        Direction::South => Direction::East,
                        Direction::East => Direction::North,
                        _ => panic!("Ship cannot face non-cardinal direction"),
                    };
                }
            }
        }
    }

    fn update_v2(&mut self, instruction: &Instruction, waypoint: &mut Waypoint) {
        match instruction.dir {
            Direction::North => waypoint.y += instruction.num,
            Direction::East => waypoint.x += instruction.num,
            Direction::South => waypoint.y -= instruction.num,
            Direction::West => waypoint.x -= instruction.num,
            Direction::Forward => {
                self.x += waypoint.x * instruction.num;
                self.y += waypoint.y * instruction.num;
            }
            Direction::Right => {
                let num_turns: u32 = instruction.num as u32 / 90;
                for _ in 0..num_turns {
                    let new_point = (waypoint.y, -waypoint.x);
                    waypoint.x = new_point.0;
                    waypoint.y = new_point.1;
                }
            }
            Direction::Left => {
                let num_turns: u32 = instruction.num as u32 / 90;
                for _ in 0..num_turns {
                    let new_point = (-waypoint.y, waypoint.x);
                    waypoint.x = new_point.0;
                    waypoint.y = new_point.1;
                }
            }
        }
    }
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    let mut ship = Ship {
        x: 0,
        y: 0,
        facing: Direction::East,
    };
    for instruction in data {
        ship.update_v1(instruction);
    }
    Some(ship.taxicab_distance())
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    let mut ship = Ship {
        x: 0,
        y: 0,
        facing: Direction::East,
    };
    let mut waypoint = Waypoint { x: 10, y: 1 };
    for instruction in data {
        ship.update_v2(instruction, &mut waypoint);
    }
    Some(ship.taxicab_distance())
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
        let input: String = "F10
N3
F7
R90
F11"
        .to_string();
        let data = get_data(input).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data), Some(17 + 8));
        assert_eq!(part_two(&data), Some(214 + 72));
    }
}
