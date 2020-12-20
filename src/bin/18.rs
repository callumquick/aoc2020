/// Solution to Advent of Code Challenge Day 18.
use aoc2020::{get_day_input, print_elapsed_time};
use std::io;
use std::str::FromStr;

const DAYNUM: &'static str = "18";
type ChallengeData = Vec<Expression>;
type ChallengeOut = u64;

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    Start,
    Multiply,
    Add,
}

#[derive(Clone, Debug)]
enum Expression {
    Expression(Vec<(Operator, Expression)>),
    Number(u64),
}

impl FromStr for Expression {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut expressions: Vec<(Operator, Expression)> = Vec::new();
        let mut open_brackets: u32 = 0;
        let mut bracket_expression = String::new();
        let mut curr_oper = Operator::Start;
        for ch in s.replace(' ', "").chars() {
            if ch == ')' {
                open_brackets -= 1;
                if open_brackets == 0 {
                    // This is the closing bracket of the sub expression, so parse the sub
                    // expression and add it to the list of operations to expressions.
                    expressions.push((curr_oper.clone(), bracket_expression.parse().unwrap()));
                    bracket_expression.clear();
                    continue;
                }
            }
            if ch == '(' {
                open_brackets += 1;
                if open_brackets == 1 {
                    // This is the first bracket of a new expression we're going to recursively
                    // parse, so don't add this bracket to the bracket expression.
                    continue;
                }
            }
            if open_brackets > 0 {
                bracket_expression.push(ch);
                continue;
            }
            match ch {
                // Already dealt with bracket above
                '(' | ')' => continue,
                '*' => {
                    curr_oper = Operator::Multiply;
                }
                '+' => {
                    curr_oper = Operator::Add;
                }
                '1'..='9' => expressions.push((
                    curr_oper.clone(),
                    Expression::Number(ch.to_digit(10).unwrap() as u64),
                )),
                _ => panic!("Found invalid character in expression"),
            }
        }
        Ok(Self::Expression(expressions))
    }
}

impl Expression {
    fn calculate_v1(&self) -> u64 {
        let mut answer: u64 = 0;
        match self {
            Expression::Expression(expressions) => {
                expressions.iter().for_each(|(op, exp)| match op {
                    Operator::Start => answer = exp.calculate_v1(),
                    Operator::Multiply => answer *= exp.calculate_v1(),
                    Operator::Add => answer += exp.calculate_v1(),
                });
            }
            Expression::Number(number) => {
                answer = *number as u64;
            }
        }
        answer
    }
    fn calculate_v2(&self) -> u64 {
        match self {
            Expression::Expression(expressions) => {
                // Iterate through the expression, creating a new list of expressions as we go.
                // Compare to the previous expression: if the operator between them is addition,
                // then perform the addition and create a new "operator, expression" pair with the
                // resulting number as the expression. If amalgamating, add to the new expression
                // list, otherwise add to the list without modification. Then iterate that
                // (multiplication only) list with calculate_v1.
                let mut new_expressions: Vec<(Operator, Expression)> = Vec::new();
                for (op, exp) in expressions {
                    // If this is an Add, add it to the previous expression.
                    // NOTE: This works since the first operator, expression pair in an expression
                    // list should be a Start, not an Add.
                    if *op == Operator::Add {
                        let (old_op, old_exp) = new_expressions.pop().unwrap();
                        new_expressions.push((
                            old_op,
                            Expression::Number(old_exp.calculate_v2() + exp.calculate_v2()),
                        ));
                    } else {
                        // Keep the existing operator, but calculate the expression as a number to
                        // ensure the addition is pre-calculated in all sub-expressions.
                        new_expressions.push((op.clone(), Expression::Number(exp.calculate_v2())));
                    }
                }
                // Now only left with Multiply operator, run through the expression in standard
                // left-to-right precedence.
                Expression::Expression(new_expressions).calculate_v1()
            }
            Expression::Number(number) => *number as u64,
        }
    }
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    Some(data.iter().map(|exp| exp.calculate_v1()).sum())
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    Some(data.iter().map(|exp| exp.calculate_v2()).sum())
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
    fn test_given_examples() {
        let inputs: [String; 6] = [
            "1 + 2 * 3 + 4 * 5 + 6".to_string(),
            "1 + (2 * 3) + (4 * (5 + 6))".to_string(),
            "2 * 3 + (4 * 5)".to_string(),
            "5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string(),
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string(),
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string(),
        ];

        let answers_v1: [u64; 6] = [71, 51, 26, 437, 12240, 13632];
        let answers_v2: [u64; 6] = [231, 51, 46, 1445, 669060, 23340];

        for (input, (answer_v1, answer_v2)) in
            inputs.iter().zip(answers_v1.iter().zip(answers_v2.iter()))
        {
            let data = get_data(input.to_string()).expect("Couldn't convert test input");
            assert_eq!(part_one(&data), Some(*answer_v1));
            assert_eq!(part_two(&data), Some(*answer_v2));
        }
    }
}
