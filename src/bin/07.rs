/// Solution to Advent of Code Challenge Day 07.
use aoc2020::{get_day_input, print_elapsed_time};
use std::collections::HashMap;
use std::io;
use std::ops::Mul;

const DAYNUM: &'static str = "07";
type ChallengeData = HashMap<String, HashMap<String, usize>>;
type ChallengeOut = usize;

/// Recursively determine if a bag type can (eventually) contain at least one of another bag type.
fn contains_bag_type(data: &ChallengeData, bag_type: &String, contains: &'static str) -> bool {
    data.get(bag_type)
        .map(|types| {
            types
                .iter()
                .map(|(key, _)| key)
                .any(|key| key == contains || contains_bag_type(data, key, contains))
        })
        .unwrap_or(false)
}

/// Recursively find the number of bags contained within a given bag type.
fn get_bag_num(data: &ChallengeData, bag_type: &String) -> Option<ChallengeOut> {
    data.get(bag_type)
        .map(|types| {
            types
                .iter()
                .map(|(key, val)| get_bag_num(data, key).map(|n| n.mul(val)))
                .collect::<Option<Vec<_>>>()
                .map(|s| s.iter().sum::<usize>() + 1)
        })
        .flatten()
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    Some(
        data.iter()
            .map(|(key, _)| key)
            .filter(|key| contains_bag_type(data, key, "shiny gold"))
            .count(),
    )
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    // Need number of bags contained: get_bag_num recursion includes the shiny gold bag itself
    get_bag_num(data, &"shiny gold".to_string()).map(|n| n - 1)
}

fn get_data(input: String) -> Result<ChallengeData, io::Error> {
    Ok(input
        .lines()
        .map(|s| {
            // <descr> bags contain (no other bags | {<num> <descr> bag[s]}).
            let bag_map: Vec<_> = s
                .split(" bags contain ")
                .map(|s| s.trim_matches('.'))
                .collect();
            (
                // <descr>
                bag_map[0].to_string(),
                // (no other bags | {<num> <descr> bag[s]})
                bag_map[1]
                    .split(", ")
                    .filter(|s| *s != "no other bags")
                    // {<num> <descr> bag[s]}
                    .map(|s| {
                        let v = s
                            .strip_suffix(" bags")
                            .or(s.strip_suffix(" bag"))
                            .expect("Bag description must end in 'bag' or 'bags")
                            // {<num> <descr>}
                            .splitn(2, ' ')
                            .collect::<Vec<_>>();
                        (
                            // <descr>
                            v[1].to_string(),
                            // <num>
                            v[0].parse::<usize>().expect("Bag number must be integer"),
                        )
                    })
                    .collect(),
            )
        })
        .collect())
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
        let input: String = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            .to_string();
        let data = get_data(input).expect("Couldn't convert test input");

        // Check the data conversion works.
        assert_eq!(
            data.get("light red").map(|d| d.get("bright white")),
            Some(Some(&1))
        );
        assert_eq!(
            data.get("light red").map(|d| d.get("muted yellow")),
            Some(Some(&2))
        );
        assert_eq!(
            data.get("bright white").map(|d| d.get("shiny gold")),
            Some(Some(&1))
        );
        assert_eq!(data.get("faded blue"), Some(&HashMap::new()));

        // Assert get the right number.
        assert_eq!(part_one(&data), Some(4));
        assert_eq!(part_two(&data), Some(32));
    }

    #[test]
    fn test_other_example() {
        let input: String = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
            .to_string();
        let data = get_data(input).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_two(&data), Some(126));
    }
}
