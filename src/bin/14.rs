/// Solution to Advent of Code Challenge Day 14.
use aoc2020::{get_day_input, print_elapsed_time};
use itertools::Itertools;
use std::collections::HashMap;
use std::io;
use std::str::FromStr;

type Number = u64;

const DAYNUM: &'static str = "14";
type ChallengeData = Vec<Instruction>;
type ChallengeOut = Number;

#[derive(Clone, Copy, Debug)]
enum Mask {
    Or(Number),
    And(Number),
    Float(Number),
}

impl Mask {
    fn apply_v1(self, num: Number) -> Number {
        match self {
            Self::Or(mask) => num | mask,
            Self::And(mask) => num & mask,
            // Float is unused in v1
            Self::Float(_) => num,
        }
    }

    fn apply_or_v2(self, num: Number) -> Number {
        match self {
            Self::Or(mask) => num | mask,
            // Only Or should be applied during the first phase
            _ => panic!("Only or applied in phase 1"),
        }
    }

    fn apply_float_v2(self, num: Number) -> [Number; 2] {
        match self {
            Self::Float(mask) => [num | mask, num & !mask],
            // Only float should not be applied during the second phase
            _ => panic!("Only float applied in phase 2"),
        }
    }
}

#[derive(Clone, Debug)]
struct Masks(Vec<Mask>);

impl FromStr for Masks {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .rev()
                .enumerate()
                // Turn an X into a 2 so it can be matched on in below code
                .map(|(i, ch)| if ch == 'X' { (i, '2') } else { (i, ch) })
                .map(|(i, ch)| {
                    // Get whether the mask is going to be "set to 1" or "set to 0".
                    // Use radix 3 to allow 'X' to be represented as 2.
                    let digit: u32 = ch.to_digit(3).unwrap();
                    // Depending on whether it is a 1 or a 0, convert it to an Or with the
                    // equivalent mask (corresponding to this position in a binary number), an And
                    // with the inverse mask or mark that this position should be treated as Float.
                    match digit {
                        2 => Mask::Float(1u64 << i),
                        1 => Mask::Or(1u64 << i),
                        0 => Mask::And(!(1u64 << i)),
                        _ => panic!("Can only use 0 or 1 in the mask definition"),
                    }
                })
                .collect(),
        ))
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Maskset(Masks),
    Memset(Number, Number),
}

impl FromStr for Instruction {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: (&str, &str) = s.split("=").map(|s| s.trim()).next_tuple().unwrap();

        if split.0 == "mask" {
            return Ok(Self::Maskset(split.1.parse()?));
        }

        let address = split
            .0
            .trim_start_matches("mem[")
            .trim_end_matches("]")
            .parse()
            .unwrap();
        let number = split.1.parse().unwrap();

        Ok(Self::Memset(address, number))
    }
}

struct ProgramState {
    masks: Masks,
    data: HashMap<Number, Number>,
}

impl ProgramState {
    fn new() -> Self {
        Self {
            masks: Masks(Vec::new()),
            data: HashMap::new(),
        }
    }

    fn run_instructions_v1(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.run_instruction_v1(instruction);
        }
    }

    fn run_instruction_v1(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Maskset(masks) => self.masks = masks.clone(),
            Instruction::Memset(addr, number) => self.memset_v1(*addr, *number),
        }
    }

    fn memset_v1(&mut self, addr: Number, number: Number) {
        let mut new = number;
        for mask in &self.masks.0 {
            new = mask.apply_v1(new);
        }
        self.data.insert(addr, new);
    }

    fn run_instructions_v2(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.run_instruction_v2(instruction);
        }
    }

    fn run_instruction_v2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Maskset(masks) => self.masks = masks.clone(),
            Instruction::Memset(addr, number) => self.memset_v2(*addr, *number),
        }
    }

    fn memset_v2(&mut self, addr: Number, number: Number) {
        let mut new = addr;

        let setmasks = self.masks.0.iter().filter(|mask| match mask {
            Mask::Or(_) => true,
            _ => false,
        });
        let floatmasks = self.masks.0.iter().filter(|mask| match mask {
            Mask::Float(_) => true,
            _ => false,
        });

        // Apply all the standard "set to 1" style masks to the number
        for mask in setmasks {
            new = mask.apply_or_v2(new);
        }

        // Then generate all the possible floating variants of it
        let mut floated_numbers = Vec::from([new]);
        for mask in floatmasks {
            let mut new_floated_numbers = Vec::new();
            for number in floated_numbers {
                new_floated_numbers.extend_from_slice(&mask.apply_float_v2(number));
            }
            floated_numbers = new_floated_numbers;
        }

        for addr in floated_numbers {
            self.data.insert(addr, number);
        }
    }
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    let mut state = ProgramState::new();
    state.run_instructions_v1(data);
    Some(state.data.values().sum())
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    let mut state = ProgramState::new();
    state.run_instructions_v2(data);
    Some(state.data.values().sum())
}

fn get_data(input: String) -> Result<ChallengeData, io::Error> {
    input.lines().map(|s| s.parse()).collect()
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
        let input: String = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            .to_string();
        let data = get_data(input).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data), Some(101 + 64));
    }

    #[test]
    fn test_given_example_part_two() {
        let input: String = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            .to_string();
        let data = get_data(input).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_two(&data), Some(208));
    }
}
