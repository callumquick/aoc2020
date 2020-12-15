/// Solution to Advent of Code Challenge Day 15.
use aoc2020::{get_day_input, print_elapsed_time};
use std::num::ParseIntError;

type Number = usize;

const DAYNUM: &'static str = "15";
type ChallengeData = Vec<Number>;
type ChallengeOut = Number;

const TARGET1: usize = 2020;
const TARGET2: usize = 30000000;

fn solve_for(data: &ChallengeData, target: usize) -> Option<ChallengeOut> {
    // Try to correct performance issues by using massive allocated array to store history in.
    let mut last_seen: Vec<Number> = vec![0; target];
    let mut last_num: Number;
    let mut curr_turn: usize = 0;

    // Insert the starting numbers (but avoid using the last one yet as setting its history would
    // break the needed chain).
    for number in &data[..data.len() - 1] {
        curr_turn += 1;
        last_seen[*number] = curr_turn;
    }

    // Set the current "look back" number to the last starting number, then start the algorithm.
    curr_turn += 1;
    last_num = *data.last().unwrap();

    while curr_turn < target {
        let number: Number = match last_seen[last_num] {
            // Never seen before as there is no turn 0.
            0 => 0,
            _ => curr_turn - last_seen[last_num],
        };

        // Insert the previous num (which is what we're looking at) into the last seen history.
        last_seen[last_num] = curr_turn;

        curr_turn += 1;
        last_num = number;
    }

    Some(last_num)
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    solve_for(data, TARGET1)
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    solve_for(data, TARGET2)
}

fn get_data(input: String) -> Result<ChallengeData, ParseIntError> {
    input.trim().split(',').map(|s| s.parse()).collect()
}

fn main() -> Result<(), ParseIntError> {
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
    fn test_given_examples() {
        let inputs: [String; 7] = [
            "0,3,6".to_string(),
            "1,3,2".to_string(),
            "2,1,3".to_string(),
            "1,2,3".to_string(),
            "2,3,1".to_string(),
            "3,2,1".to_string(),
            "3,1,2".to_string(),
        ];

        let answers_1: [Number; 7] = [436, 1, 10, 27, 78, 438, 1836];
        let answers_2: [Number; 7] = [175594, 2578, 3544142, 261214, 6895259, 18, 362];

        for (input, (answer1, _)) in inputs.iter().zip(answers_1.iter().zip(answers_2.iter())) {
            let data = get_data(input.to_string()).expect("Couldn't convert test input");

            // Assert get the right number.
            assert_eq!(part_one(&data), Some(*answer1));
            // Part two is disabled for general testing (takes too long).
            //assert_eq!(part_two(&data), Some(*answer2));
        }
    }
}
