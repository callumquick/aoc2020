/// Solution to Advent of Code Challenge Day 13.
use aoc2020::{get_day_input, print_elapsed_time};
use std::num::ParseIntError;

type Number = u64;

const DAYNUM: &'static str = "13";
type ChallengeData = DepartureTarget;
type ChallengeOut = Number;

struct DepartureTarget {
    timestamp: Number,
    buses: Vec<Number>,
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    // Minimise the possible time remainder from our timestamp to the next bus departure for each
    // bus: each bus can be a maximum of its ID later than our timestamp at the airport so find the
    // multiple of it which is between our timestamp and our timestamp plus its ID, and get the
    // difference.
    let buses: Vec<&Number> = data.buses.iter().filter(|&num| *num != 0).collect();
    let mut remainders: Vec<Number> = Vec::new();
    for &id in &buses {
        let needed_multiples = (data.timestamp / id) + 1;
        remainders.push((needed_multiples * id) - data.timestamp);
    }
    let min = *remainders.iter().min().unwrap();
    let id = buses[remainders.iter().position(|&item| item == min).unwrap()];
    Some(min * id)
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    let offset_constraints: Vec<_> = data
        .buses
        .iter()
        .enumerate()
        .map(|(i, id)| (i, *id))
        .filter(|(_, id)| *id != 0)
        .collect();

    // Need to use LCM: since if we know we have a solution for two IDs, then to extend the solution
    // to the third ID while still making it hold for the first two, the place to search will be
    // some multiple of the LCM of the first two!
    //
    // However, all bus IDs are prime (presumably to ensure there is only one exact solution of part
    // 1), so the LCM of two primes is the two primes multiplied, so just refine the seek amount by
    // multiplying it by the new ID.

    let mut timestamp = 0;
    // Lowest seek amount is to check each number in turn.
    let mut seek_amount = 1;

    for (offset, id) in offset_constraints {
        while (timestamp + offset as Number) % id != 0 {
            timestamp += seek_amount;
        }
        // New amount to seek by is the LCM of the previous values and the new value (but for primes
        // this is just their multiple).
        seek_amount *= id;
    }

    Some(timestamp)
}

fn get_data(input: String) -> Result<ChallengeData, ParseIntError> {
    let lines: Vec<_> = input.lines().collect();
    Ok(DepartureTarget {
        timestamp: lines[0].parse()?,
        buses: lines[1]
            .split(',')
            .map(|num| if num == "x" { "0" } else { num })
            .map(|num| num.parse())
            .collect::<Result<_, _>>()?,
    })
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
        let input: String = "939
7,13,x,x,59,x,31,19"
            .to_string();
        let data = get_data(input).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data), Some(59 * 5));
        assert_eq!(part_two(&data), Some(1068781));
    }

    #[test]
    fn test_part_two_examples() {
        let inputs: [String; 5] = [
            "0
17,x,13,19"
                .to_string(),
            "0
67,7,59,61"
                .to_string(),
            "0
67,x,7,59,61"
                .to_string(),
            "0
67,7,x,59,61"
                .to_string(),
            "0
1789,37,47,1889"
                .to_string(),
        ];

        let answers: [Number; 5] = [3417, 754018, 779210, 1261476, 1202161486];

        for (input, answer) in inputs.iter().zip(answers.iter()) {
            let data = get_data(input.to_string()).expect("Couldn't convert test input");

            assert_eq!(part_two(&data), Some(*answer));
        }
    }
}
