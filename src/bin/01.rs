/// Solution to Advent of Code Challenge Day 01.
use aoc2020::{get_day_input, get_num_set, print_elapsed_time};
use std::collections::HashSet;

const TARGET: u32 = 2020;

fn main() {
    let input = get_day_input("01");
    let num_set = get_num_set(input);
    println!("Day 01:");
    println!("==========");
    println!(
        "Part one: {}",
        print_elapsed_time(|| part_one(&num_set, TARGET)).expect("No solution found for part one"),
    );
    println!(
        "Part two: {}",
        print_elapsed_time(|| part_two(&num_set, TARGET)).expect("No solution found for part two"),
    );
}

/// Find the product of the two numbers which sum to the target value.
fn part_one(input: &HashSet<u32>, target: u32) -> Option<u32> {
    for num1 in input {
        for num2 in input {
            if num1 + num2 == target {
                return Some(num1 * num2);
            }
        }
    }
    None
}

/// Find the product of the three numbers which sum to the target value.
fn part_two(input: &HashSet<u32>, target: u32) -> Option<u32> {
    for num1 in input {
        for num2 in input {
            for num3 in input {
                if num1 + num2 + num3 == target {
                    return Some(num1 * num2 * num3);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: HashSet<u32> = [1721, 979, 366, 299, 675, 1456].iter().cloned().collect();

        // If we give 0 here, there is no solution.
        assert_eq!(part_one(&input, 0), None);
        assert_eq!(part_two(&input, 0), None);

        // Check they can reach the target.
        assert_eq!(part_one(&input, TARGET), Some(1721 * 299));
        assert_eq!(part_two(&input, TARGET), Some(979 * 366 * 675));
    }
}
