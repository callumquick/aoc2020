/// Solution to Advent of Code Challenge Day 08.
use aoc2020::{get_day_input, print_elapsed_time};
use itertools::Itertools;
use std::collections::HashSet;
use std::convert::TryInto;
use std::io;
use std::str::FromStr;

const DAYNUM: &'static str = "08";
type ChallengeData = Code;
type ChallengeOut = i32;

type Code = Vec<Instruction>;

#[derive(Debug, Copy, Clone)]
enum ExitCode {
    LoopDetected,
    Success,
    Failure,
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Nop(isize),
    Acc(i32),
    Jmp(isize),
}

impl FromStr for Instruction {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (word, num): (&str, &str) = s
            .split(' ')
            .next_tuple()
            .expect("Instruction is not of the correct form: <verb> <amount>");
        Ok(match word {
            "nop" => Instruction::Nop(
                num.parse()
                    .expect("Amount given in instruction is not a valid integer"),
            ),
            "jmp" => Instruction::Jmp(
                num.parse()
                    .expect("Amount given in instruction is not a valid integer"),
            ),
            "acc" => Instruction::Acc(
                num.parse()
                    .expect("Amount given in instruction is not a valid integer"),
            ),
            _ => panic!("Invalid instruction verb given: {}", word),
        })
    }
}

#[derive(Debug, Clone)]
struct Program {
    counter: usize,
    text: Code,
    data: i32,
}

impl From<Code> for Program {
    fn from(code: Code) -> Self {
        Program {
            counter: 0,
            text: code,
            data: 0,
        }
    }
}

impl Program {
    fn run(&mut self) -> ExitCode {
        let mut visited: HashSet<usize> = HashSet::new();
        while self.counter < self.text.len() {
            if let Some(_) = visited.get(&self.counter) {
                return ExitCode::LoopDetected;
            }
            visited.insert(self.counter);
            match self.text[self.counter] {
                Instruction::Nop(_) => {
                    self.counter += 1;
                }
                Instruction::Acc(inc) => {
                    self.data += inc;
                    self.counter += 1;
                }
                Instruction::Jmp(offset) => {
                    self.counter = (self.counter as isize + offset)
                        .try_into()
                        .expect("Jump instruction took counter out of bounds");

                    // Technically works without this, but the challenge explicitly states this is
                    // not a valid way to terminate the program (jump further than 1 instruction out
                    // of the program)
                    if self.counter > self.text.len() {
                        return ExitCode::Failure;
                    }
                }
            }
        }
        ExitCode::Success
    }
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    let mut program = Program::from(data.to_vec());
    match program.run() {
        ExitCode::LoopDetected => Some(program.data),
        _ => None,
    }
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    // For each instruction, if it is a nop or a jmp, try the program with the instruction switched
    // to see if it can exit normally.
    for linenum in 0..data.len() {
        let mut code = data.to_vec();
        code[linenum] = match code[linenum] {
            Instruction::Acc(_) => continue,
            Instruction::Jmp(offset) => Instruction::Nop(offset),
            Instruction::Nop(offset) => Instruction::Jmp(offset),
        };
        let mut program = Program::from(code);
        match program.run() {
            ExitCode::Success => {
                return Some(program.data);
            }
            _ => (),
        }
    }
    None
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
    println!(
        "Answer: {}",
        print_elapsed_time(|| part_one(&data)).expect("No solution found for part one"),
    );
    println!("==========");
    println!("Solving part two...");
    println!(
        "Answer: {}",
        print_elapsed_time(|| part_two(&data)).expect("No solution found for part two"),
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: String = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            .to_string();
        let data = get_data(input).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data), Some(5));
        assert_eq!(part_two(&data), Some(8));
    }
}
