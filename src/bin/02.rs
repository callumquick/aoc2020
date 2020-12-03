/// Solution to Advent of Code Challenge Day 02.
use aoc2020::{get_day_input, print_elapsed_time};
use std::num::ParseIntError;
use std::str::FromStr;

/// Structure specifying a policy and the password to validate against it.
#[derive(Debug)]
struct PasswordPolicy {
    range: (usize, usize),
    ch: char,
    password: String,
}

impl FromStr for PasswordPolicy {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split(' ').collect();

        let range_nums: Vec<usize> = fields[0]
            .split('-')
            .map(|s| {
                s.parse()
                    .expect("Range values are cannot be parsed to numbers")
            })
            .collect();
        let ch: char = fields[1]
            .chars()
            .nth(0)
            .expect("The character specified is invalid");
        let password: &str = fields[2];

        Ok(PasswordPolicy {
            ch,
            range: (range_nums[0], range_nums[1]),
            password: password.to_string(),
        })
    }
}

impl PasswordPolicy {
    /// Check if the password is valid given the sled shop policy.
    fn is_valid_sled_policy(&self) -> bool {
        let num_chars = self.password.matches(self.ch).count();
        let range = self.range.0..=self.range.1;
        range.contains(&num_chars)
    }

    /// Check if the password is valid given the toboggan store policy.
    fn is_valid_toboggan_policy(&self) -> bool {
        let match_idxs: Vec<usize> = self
            .password
            .match_indices(self.ch)
            .map(|(idx, _)| idx)
            .collect();
        match_idxs.contains(&(self.range.0 - 1)) ^ match_idxs.contains(&(self.range.1 - 1))
    }
}

/// Parse the challenge input into the list of decoded data structures.
fn get_password_policies_list(input: String) -> Vec<PasswordPolicy> {
    input
        .lines()
        .map(|s| s.parse().expect("Invalid password policy in input"))
        .collect()
}

// O(N) search through the list to validate the policies against sled criteria.
fn part_one(data: &[PasswordPolicy]) -> Option<u32> {
    Some(data.iter().fold(0, |acc, x| {
        if x.is_valid_sled_policy() {
            return acc + 1;
        }
        acc
    }))
}

// O(N) search through the list to validate the policies against tobbogan criteria.
fn part_two(data: &[PasswordPolicy]) -> Option<u32> {
    Some(data.iter().fold(0, |acc, x| {
        if x.is_valid_toboggan_policy() {
            return acc + 1;
        }
        acc
    }))
}

fn main() {
    let input = get_day_input("02");
    let data = get_password_policies_list(input);
    println!("Day 02:");
    println!("==========");
    println!(
        "Part one: {}",
        print_elapsed_time(|| part_one(&data)).expect("No solution found for part one"),
    );
    println!(
        "Part two: {}",
        print_elapsed_time(|| part_two(&data)).expect("No solution found for part two"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: String = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"
        .to_string();
        let data = get_password_policies_list(input);

        // Check each gives the right answer.
        assert_eq!(part_one(&data), Some(2));
        assert_eq!(part_two(&data), Some(1));
    }

    #[test]
    fn test_more_complex_example() {
        let input: String = "1-3 a: abcde
1-3 b: cdefg
1-90 g: dd
2-2 f: foo
1-2 f: foo
5-6 a: bbbbab
10-20 z: lowosapososdoaspdospoadpwejekjfbejfbdsdsdhadjh
"
        .to_string();
        let data = get_password_policies_list(input);

        // Check each gives the right answer.
        assert_eq!(part_one(&data), Some(2));
        assert_eq!(part_two(&data), Some(3));
    }
}
