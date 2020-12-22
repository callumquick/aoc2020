/// Solution to Advent of Code Challenge Day 21.
use aoc2020::{get_day_input, print_elapsed_time};
use std::collections::{HashMap, HashSet};
use std::io;
use std::str::FromStr;

const DAYNUM: &'static str = "21";
type ChallengeData = Vec<Food>;
type ChallengeOut = u32;

#[derive(Clone, Debug)]
struct Food {
    ingreds: HashSet<String>,
    allergens: HashSet<String>,
}

impl FromStr for Food {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" (contains ");
        let ingred_list = parts.next().unwrap();
        let allergens: Option<HashSet<String>> = parts.next().map(|s| {
            s.trim_matches(')')
                .split(", ")
                .map(|s| s.to_string())
                .collect()
        });
        Ok(Self {
            ingreds: ingred_list.split(' ').map(|s| s.to_string()).collect(),
            allergens: allergens.unwrap_or(HashSet::new()),
        })
    }
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    // - For each allergen, keep a set of which ingredients it could be
    // - For each food listed:
    //     - For each allergen listed:
    //         - If already have a set of possible ingreds, take the union with the new possible
    //           ingreds to see which ingreds are always the same
    //         - Else take the new possible ingreds as current best guess
    // Then, combine all the sets for all allergens with their possibilities. Take the set of all
    // ingredients and the difference is any ingredient which can't possibly be an allergen.

    let mut allergen_possibles: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_ingreds: HashSet<String> = HashSet::new();

    for food in data {
        let new_possible_ingreds: HashSet<_> = food.ingreds.iter().map(|s| s.to_string()).collect();
        all_ingreds.extend(new_possible_ingreds.clone());
        for allergen in &food.allergens {
            let entry = allergen_possibles
                .entry(allergen.clone())
                .or_insert_with(|| food.ingreds.iter().map(|s| s.to_string()).collect());
            *entry = &entry.clone() & &new_possible_ingreds;
        }
    }

    let mut all_allergen_possibles: HashSet<String> = HashSet::new();
    for possibles in allergen_possibles.values() {
        all_allergen_possibles = &all_allergen_possibles | possibles;
    }

    let impossible_allergens = &all_ingreds - &all_allergen_possibles;

    let mut number_impossibles = 0;
    for food in data {
        for ingred in &food.ingreds {
            if impossible_allergens.contains(ingred) {
                number_impossibles += 1;
            }
        }
    }

    Some(number_impossibles)
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<String> {
    // Use the same strategy as part one to get the possible matches for each allergen.
    let mut allergen_possibles: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_ingreds: HashSet<String> = HashSet::new();

    for food in data {
        let new_possible_ingreds: HashSet<_> = food.ingreds.iter().map(|s| s.to_string()).collect();
        all_ingreds.extend(new_possible_ingreds.clone());
        for allergen in &food.allergens {
            let entry = allergen_possibles
                .entry(allergen.clone())
                .or_insert_with(|| food.ingreds.iter().map(|s| s.to_string()).collect());
            *entry = &entry.clone() & &new_possible_ingreds;
        }
    }

    // Then go a step further: for each allergen with only one possible ingredient, define it and
    // then remove that ingredient from any other allergen possibles. Repeat until all are defined.
    let mut allergen_defs: HashMap<String, String> = HashMap::new();
    let mut defined_ingreds = HashSet::new();

    while !allergen_possibles.is_empty() {
        for (allergen, possibles) in allergen_possibles.clone().iter() {
            let possibles = &possibles.clone() - &defined_ingreds;
            if possibles.len() == 1 {
                for possible in possibles.iter() {
                    defined_ingreds.insert(possible.clone());
                    allergen_defs.insert(allergen.clone(), possible.clone());
                    allergen_possibles.remove(allergen);
                }
            }
        }
    }

    let mut allergens: Vec<&String> = allergen_defs.keys().collect();
    allergens.sort();
    let dangerous_ingreds: Vec<String> = allergens
        .iter()
        .map(|allergen| allergen_defs.get(*allergen).unwrap().to_string())
        .collect();
    Some(dangerous_ingreds.join(","))
}

fn get_data(input: String) -> Result<ChallengeData, io::Error> {
    input.trim().split("\n").map(|s| s.parse()).collect()
}

fn main() -> Result<(), io::Error> {
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
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"
            .to_string();

        let data = get_data(input.to_string()).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data), Some(5));
        assert_eq!(part_two(&data), Some(String::from("mxmxvkd,sqjhc,fvjkl")));
    }
}
