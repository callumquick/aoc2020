/// Solution to Advent of Code Challenge Day 06.
use aoc2020::{get_day_input, print_elapsed_time};
use std::collections::HashSet;
use std::io;
use std::iter::FromIterator;

const DAYNUM: &'static str = "06";
type ChallengeData = Vec<Vec<HashSet<char>>>;
type ChallengeOut = usize;

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    // Get the number of answers given, where each answer is only required to appear once per group.
    Some(
        data.iter()
            .map(|v| HashSet::<char>::from_iter(v.iter().flatten().map(|c| *c)).len())
            .sum(),
    )
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    // Get the number of answers given, where each answer is required to be given by all members of
    // a group.
    Some(
        data.iter()
            .map(|v| {
                let mut set = HashSet::new();
                set = &set | &v[0];
                for other in &v[1..] {
                    set = &set & other;
                }
                set.len()
            })
            .sum(),
    )
}

fn get_data(input: String) -> Result<ChallengeData, io::Error> {
    input
        .split("\n\n")
        .map(|s| {
            Ok(s.lines()
                .map(|s| HashSet::from_iter(s.chars().filter(|c| ('a'..='z').contains(&c))))
                .collect())
        })
        .collect()
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
        let input: String = "abc

a
b
c

ab
ac

a
a
a
a

b"
        .to_string();
        let data = get_data(input).expect("Couldn't convert test input");

        // Assert get the right number of answers.
        assert_eq!(part_one(&data), Some(11));
        assert_eq!(part_two(&data), Some(6));
    }
}
