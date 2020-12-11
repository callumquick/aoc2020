/// Solution to Advent of Code Challenge Day 08.
use aoc2020::{get_day_input, print_elapsed_time};
use std::collections::{HashSet, VecDeque};
use std::num::ParseIntError;
use std::ops::Add;

type Number = u64;

const DAYNUM: &'static str = "09";
type ChallengeData = Vec<Number>;
type ChallengeOut = Number;

/// Solution to part one.
fn part_one(data: &ChallengeData, preamble: usize) -> Option<ChallengeOut> {
    let mut rolling_queue: VecDeque<Number> = data.iter().take(preamble).map(|n| *n).collect();
    let mut rolling_set: HashSet<Number> = rolling_queue.iter().map(|n| *n).collect();
    for number in &data[preamble..] {
        let number = *number;
        let mut ans: Option<(Number, Number)> = None;

        for x in &rolling_queue {
            let x = *x;
            if number > x {
                let y: Number = number - x;
                if rolling_set.contains(&y) {
                    ans = Some((x, y));
                    break;
                }
            }
        }

        if ans.is_none() {
            // This number is the first which does not respect the condition that it must contain
            // a pair in the last preamble which sum to it set by XMAS.
            return Some(number);
        }

        // Set the queue and tracking set to the new preamble given that this number is valid.
        rolling_queue.push_back(number);
        let old = rolling_queue.pop_front().unwrap();
        rolling_set.insert(number);
        rolling_set.remove(&old);
    }
    None
}

/// Solution to part two.
fn part_two(data: &ChallengeData, target: Number) -> Option<ChallengeOut> {
    // The contiguous set must be at least 2 long, so prepopulate with 1 value.
    let mut rolling_queue: VecDeque<Number> = data.iter().take(1).map(|n| *n).collect();
    for number in &data[1..] {
        let number = *number;
        rolling_queue.push_back(number);
        let mut curr_sum: Number = rolling_queue.iter().sum();
        while curr_sum > target {
            // We're too high: pop earlier numbers until we go low enough to continue.
            rolling_queue.pop_front();
            curr_sum = rolling_queue.iter().sum();
        }
        // May now reach the target itself: but if we need more numbers, just carry on.
        if rolling_queue.len() < 2 {
            continue;
        }
        if curr_sum == target {
            return rolling_queue
                .iter()
                .min()
                .and_then(|min| Some(min.add(*rolling_queue.iter().max()?)));
        }
    }
    None
}

fn get_data(input: String) -> Result<ChallengeData, ParseIntError> {
    input.lines().map(|s| s.parse()).collect()
}

fn main() -> Result<(), ParseIntError> {
    println!("Day {}:", DAYNUM);
    println!("==========");
    println!("Getting data...");
    let data = print_elapsed_time(|| get_data(get_day_input(DAYNUM)))?;
    println!("==========");
    println!("Solving part one...");
    let ans1 = print_elapsed_time(|| part_one(&data, 25)).expect("No solution found for part one");
    println!("Answer: {}", ans1);
    println!("==========");
    let ans2 =
        print_elapsed_time(|| part_two(&data, ans1)).expect("No solution found for part two");
    println!("Solving part two...");
    println!("Answer: {}", ans2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: String = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
        .to_string();
        let data = get_data(input).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data, 5), Some(127));
        assert_eq!(part_two(&data, 127), Some(62));
    }
}
