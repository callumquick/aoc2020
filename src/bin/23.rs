/// Solution to Advent of Code Challenge Day 23.
use aoc2020::{get_day_input, print_elapsed_time};
use std::io;

const DAYNUM: &'static str = "23";
type ChallengeData = Vec<u32>;
type ChallengeOut = String;

trait LinkedList {
    fn get_next(&self, label: u32) -> u32;
    fn set_next(&mut self, label: u32, value: u32);
}

impl LinkedList for Vec<u32> {
    fn get_next(&self, label: u32) -> u32 {
        self[label as usize - 1]
    }

    fn set_next(&mut self, label: u32, value: u32) {
        self[label as usize - 1] = value;
    }
}

/// Do a number of iteration on a cup deque, where the curr cup at the start is taken to be the cup
/// at index 0.
fn do_iterations(cups: &mut Vec<u32>, first_cup: u32, iterations: usize) {
    let mut curr_cup = first_cup;
    let highest_number: u32 = cups.len() as u32;

    for _ in 0..iterations {
        // Get the three cups after the current cup in the linked list
        let pick1 = cups.get_next(curr_cup);
        let pick2 = cups.get_next(pick1);
        let pick3 = cups.get_next(pick2);

        // Remove them from the circle by making the current cup point to the cup which was after
        // the third cup.
        cups[curr_cup as usize - 1] = cups.get_next(pick3);

        // Seek the destination cup among the remaining cups: we know which cups were picked up
        // (they're now in the cups variable). If the sought dest cup is amongst the picked cups,
        // try again.
        let mut dest_cup = curr_cup - 1;
        while [pick1, pick2, pick3].contains(&dest_cup) || dest_cup == 0 {
            // If it's less than any numbered cup, wrap around to start from the highest numbered cup.
            if dest_cup < 1 {
                dest_cup = highest_number;
            } else {
                dest_cup -= 1;
            }
        }

        // For each element in the picked cups, insert immediately clockwise of the destination cup.
        // Insert the whole slice [1, 2, 3] by setting dest cup to point to 1 and setting 3 to point
        // to dest cup's next
        cups[pick3 as usize - 1] = cups.get_next(dest_cup);
        cups[dest_cup as usize - 1] = pick1;

        // The new current cup is the cup after the current cup
        curr_cup = cups.get_next(curr_cup);
    }
}

fn get_cup_layout(data: &ChallengeData, size: u32) -> Vec<u32> {
    let mut cups = vec![0u32; size as usize];
    let mut labels = data.clone();

    labels.extend((labels.len() as u32 + 1)..=size);

    for labels in labels.windows(2) {
        // The Vec is built up with each cup label k in the circle at index k - 1 pointing to the
        // next cup in the circle.
        cups[labels[0] as usize - 1] = labels[1];
    }

    // To complete the circle, the last label needs to be added to point to the first.
    cups[labels[size as usize - 1] as usize - 1] = labels[0];

    cups
}

/// Solution to part one.
fn part_one(data: &ChallengeData, iterations: usize) -> Option<ChallengeOut> {
    let mut cups = get_cup_layout(data, 9);

    do_iterations(&mut cups, data[0], iterations);

    // Get all cups after cup 1 by starting from the next cup from it and reading the linked list
    // until we reach a label of 1
    let mut cups_contiguous = Vec::new();
    let mut curr = cups[0];
    while curr != 1 {
        cups_contiguous.push(curr);
        curr = cups.get_next(curr);
    }

    Some(
        cups_contiguous
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join(""),
    )
}

/// Solution to part two.
fn part_two(data: &ChallengeData, iterations: usize) -> Option<u64> {
    let mut cups = get_cup_layout(data, 1_000_000);

    do_iterations(&mut cups, data[0], iterations);

    // Need the cup after cup 1 (at index 0), and the cup after that (at index of cups[0] - 1) multiplied
    Some(cups[0] as u64 * cups[cups[0] as usize - 1] as u64)
}

fn get_data(input: String) -> Result<ChallengeData, io::Error> {
    Ok(input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as _)
        .collect())
}

fn main() -> Result<(), io::Error> {
    println!("Day {}:", DAYNUM);
    println!("==========");
    println!("Getting data...");
    let data = print_elapsed_time(|| get_data(get_day_input(DAYNUM)))?;
    println!("==========");
    println!("Solving part one...");
    let ans1 = print_elapsed_time(|| part_one(&data, 100)).expect("No solution found for part one");
    println!("Answer: {}", ans1);
    println!("==========");
    println!("Solving part two...");
    let ans2 =
        print_elapsed_time(|| part_two(&data, 10_000_000)).expect("No solution found for part two");
    println!("Answer: {}", ans2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input = "389125467".to_string();

        let data = get_data(input.to_string()).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data, 10), Some("92658374".to_string()));
        assert_eq!(part_one(&data, 100), Some("67384529".to_string()));
        assert_eq!(part_two(&data, 10_000_000), Some(149245887792));
    }
}
