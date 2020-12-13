/// Solution to Advent of Code Challenge Day 10.
use aoc2020::{get_day_input, print_elapsed_time};
use std::collections::HashMap;
use std::num::ParseIntError;

type Number = u64;

const DAYNUM: &'static str = "10";
type ChallengeData = Vec<Number>;
type ChallengeOut = Number;

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    let mut data = data.clone();
    data.sort();

    let mut num_1v_diffs: Number = 0;
    // There's always 1 3V difference between the biggest adapter since it is 3V more than the
    // maximum in the dataset.
    let mut num_3v_diffs: Number = 1;
    let mut last: Number = 0;

    for number in data {
        match number - last {
            1 => num_1v_diffs += 1,
            2 => (),
            3 => num_3v_diffs += 1,
            _ => return None,
        }
        last = number;
    }

    Some(num_1v_diffs * num_3v_diffs)
}

/// Use sorted data to work out the number of ways to reach an adapter from the available compatible
/// adapters.
/// Use some basic caching to try and beat performance issues.
fn num_ways(data: &ChallengeData, idx: usize, known: &mut HashMap<usize, Number>) -> Number {
    // The base case is that the first adapter has only one way to get to it (from the charging
    // port).
    if let Some(&ans) = known.get(&idx) {
        return ans;
    }
    match idx {
        0 => 1,
        _ => {
            let mut ways = 0;
            let mut idx_diff = 1;
            // Can reach this adapter if the num volts different is 3 or less. If can't reach
            // this adapter, then we've calculated all the varied ways from reachable adapters
            // to this one.
            while idx_diff <= idx && data[idx] - data[idx - idx_diff] <= 3 {
                ways += num_ways(data, idx - idx_diff, known);
                idx_diff += 1;
            }
            known.insert(idx, ways);
            ways
        }
    }
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    // Use a recursive function to calculate the number of ways to get to a given (final) adapter.
    // The "last" (device) adapter doesn't count (because there is only one way to get to it as it
    // is 3V above the highest adapter in the data set), but we need to consider way the ingress
    // voltage (0V relative charging port) can reach a number of adapters.
    let mut data = data.clone();
    // Add in the "first" voltage, the 0V represented by the charging power.
    data.push(0);
    data.sort();
    let mut cache: HashMap<usize, Number> = HashMap::new();
    Some(num_ways(&data, data.len() - 1, &mut cache))
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
        let input: String = "16
10
15
5
1
11
7
19
6
12
4"
        .to_string();
        let data = get_data(input).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data), Some(7 * 5));
        assert_eq!(part_two(&data), Some(8))
    }

    #[test]
    fn test_other_given_example() {
        let input: String = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
        .to_string();
        let data = get_data(input).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data), Some(22 * 10));
        assert_eq!(part_two(&data), Some(19208))
    }
}
