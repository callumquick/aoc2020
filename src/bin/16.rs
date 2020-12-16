/// Solution to Advent of Code Challenge Day 16.
use aoc2020::{get_day_input, print_elapsed_time};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io;
use std::ops::RangeInclusive;
use std::str::FromStr;

type Number = u64;
type Ticket = Vec<Number>;
type Constraint = [RangeInclusive<Number>; 2];

const DAYNUM: &'static str = "16";
type ChallengeData = InputData;
type ChallengeOut = Number;

#[derive(Debug, Clone)]
struct InputData {
    constraints: HashMap<String, Constraint>,
    your_ticket: Ticket,
    tickets: Vec<Ticket>,
}

impl FromStr for InputData {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (fields, yours, nearby) = s.split("\n\n").next_tuple().unwrap();

        let constraints = fields
            .lines()
            .map(|s| {
                let (key, range_specifier) = s.split(": ").next_tuple().unwrap();
                let (range1, range2) = range_specifier.split(" or ").next_tuple().unwrap();
                let (range1_low, range1_high) = range1.split('-').next_tuple().unwrap();
                let (range2_low, range2_high) = range2.split('-').next_tuple().unwrap();
                (
                    key.to_string(),
                    [
                        range1_low.parse().unwrap()..=range1_high.parse().unwrap(),
                        range2_low.parse().unwrap()..=range2_high.parse().unwrap(),
                    ],
                )
            })
            .collect();
        let your_ticket = yours
            .strip_prefix("your ticket:\n")
            .unwrap()
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let tickets = nearby
            .strip_prefix("nearby tickets:\n")
            .unwrap()
            .lines()
            .map(|s| s.split(',').map(|s| s.parse()).collect())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        Ok(Self {
            constraints,
            your_ticket,
            tickets,
        })
    }
}

fn ticket_find_invalid(ticket: &Ticket, constraints: &HashMap<String, Constraint>) -> Vec<Number> {
    let mut invalid = Vec::new();
    for number in ticket {
        let mut valid = false;
        for constraint in constraints.values() {
            if constraint[0].contains(&number) || constraint[1].contains(&number) {
                // Fits at least one field constraint.
                valid = true;
            }
        }
        if !valid {
            invalid.push(*number);
        }
    }
    invalid
}

/// A version of ticket_find_invalid which returns early on the first invalid number to be able to
/// quickly dismiss invalid tickets.
fn ticket_is_invalid(ticket: &Ticket, constraints: &HashMap<String, Constraint>) -> bool {
    for number in ticket {
        let mut valid = false;
        for constraint in constraints.values() {
            if constraint[0].contains(&number) || constraint[1].contains(&number) {
                // Fits at least one field constraint.
                valid = true;
            }
        }
        if !valid {
            return true;
        }
    }
    false
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    let mut invalid = Vec::new();
    for ticket in &data.tickets {
        invalid.extend(ticket_find_invalid(ticket, &data.constraints));
    }
    Some(invalid.iter().sum())
}

/// Solution to part two.
fn part_two(data: &ChallengeData, startswith: &'static str) -> Option<ChallengeOut> {
    let valid_tickets: Vec<&Ticket> = data
        .tickets
        .iter()
        .filter(|ticket| !ticket_is_invalid(ticket, &data.constraints))
        .collect();

    // For each "column" in a ticket, determine which set of constraints it fits. If it fits a
    // single constraint, that column must correspond to that field. If a field gets allocated to a
    // column, remove it from consideration and from all existing analyses until each column is
    // assigned exactly one field.
    // Keep track of the definites and the possibilities.
    let mut field_defs: HashMap<usize, String> = HashMap::new();
    let mut field_possibles: HashMap<usize, HashSet<String>> = HashMap::new();

    let mut col = 0;
    while field_defs.len() < data.your_ticket.len() {
        for (field, constraint) in data.constraints.iter() {
            if field_defs.values().find(|&v| v == field).is_some() {
                // No longer need this constraint to be considered.
                continue;
            }
            let mut col_valid = true;
            for number in valid_tickets.iter().map(|v| v[col]) {
                if !constraint[0].contains(&number) && !constraint[1].contains(&number) {
                    col_valid = false;
                    break;
                }
            }
            if col_valid {
                let possibles = field_possibles.entry(col).or_insert(HashSet::new());
                possibles.insert(field.to_string());
            }
        }

        // Must have matched at least one field, so unwrap.
        let mut newly_fixed: Vec<String> = Vec::new();
        if field_possibles.get(&col).unwrap().len() == 1 {
            // Just the one, so fix this field as defined to be this column.
            for field in field_possibles.get(&col).unwrap().iter() {
                field_defs.insert(col, field.to_string());
                newly_fixed.push(field.to_string());
            }
        }

        // For any previously assessed column, remove this field from consideration. If that leaves
        // it with just one, add that newly fixed field to the list of now fixed fields, then start
        // from the beginning again to try and find other now-fixed fields.
        // This could be written as a recursion (?), but here have it as a while with dynamic loop variables.
        let mut iter_col = 0;
        while iter_col < col && !newly_fixed.is_empty() {
            let mut newly_fixed_new = Vec::new();
            for fixed in newly_fixed.iter() {
                field_possibles.get_mut(&iter_col).unwrap().remove(fixed);
                // If this has been reduced to a single choice and it is not already recorded in the
                // field definitions, it has been newly fixed!
                if field_possibles.get(&iter_col).unwrap().len() == 1
                    && field_defs.get(&iter_col).is_none()
                {
                    for field in field_possibles.get(&iter_col).unwrap().iter() {
                        field_defs.insert(iter_col, field.to_string());
                        newly_fixed_new.push(field.to_string());
                    }
                }
            }
            if !newly_fixed_new.is_empty() {
                // Start the search again from the beginning in case we need to fixup other
                // previous columns using the newly fixed fields.
                iter_col = 0
            } else {
                // Continue looking.
                iter_col += 1;
            }
            newly_fixed.extend(newly_fixed_new);
        }

        if col == data.your_ticket.len() && field_defs.len() < data.your_ticket.len() {
            return None;
        }

        col += 1;
    }

    Some(
        field_defs
            .iter()
            .filter(|(_, field)| field.starts_with(startswith))
            .map(|(col, _)| data.your_ticket[*col])
            .product(),
    )
}

fn get_data(input: String) -> Result<ChallengeData, io::Error> {
    input.parse()
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
    let ans2 = print_elapsed_time(|| part_two(&data, "departure"))
        .expect("No solution found for part two");
    println!("Answer: {}", ans2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            .to_string();

        let data = get_data(input.to_string()).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data), Some(4 + 55 + 12));
    }

    #[test]
    fn test_given_example_part_two() {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
            .to_string();

        let data = get_data(input.to_string()).expect("Couldn't convert test input");

        // Assert get the right number (second arg gives different fields).
        assert_eq!(part_two(&data, "class"), Some(12));
        assert_eq!(part_two(&data, "row"), Some(11));
        assert_eq!(part_two(&data, "seat"), Some(13));
    }
}
