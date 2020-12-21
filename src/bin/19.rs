/// Solution to Advent of Code Challenge Day 19.
use aoc2020::{get_day_input, print_elapsed_time};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io;
use std::str::FromStr;

const DAYNUM: &'static str = "19";
type ChallengeData = InputData;
type ChallengeOut = usize;

#[derive(Clone, Debug)]
enum Match {
    // Matches another rule
    Rule(u32),
    // Matches a character directly
    Char(char),
}

impl FromStr for Match {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_matches('"');
        let ch = s.parse::<char>().ok();
        match ch {
            Some(ch) if ('a'..='z').contains(&ch) => Ok(Self::Char(ch)),
            Some(_) | None => Ok(Self::Rule(s.parse::<u32>().unwrap())),
        }
    }
}

type Rule = Vec<Vec<Match>>;
type Rules = HashMap<u32, Rule>;

#[derive(Clone, Debug)]
struct InputData {
    rules: Rules,
    messages: Vec<String>,
}

/// Expand a rule into a selection of strings that would have to be matched exactly for the
/// input to be valid.
fn expand_rule(
    rules: &Rules,
    key: u32,
    cache: &mut HashMap<u32, HashSet<String>>,
) -> HashSet<String> {
    // Attempt to hit the cache first
    if let Some(ans) = cache.get(&key) {
        return ans.clone();
    }
    let rule = rules.get(&key).unwrap();
    let mut matches = HashSet::new();
    for choice in rule {
        // Start with an empty match string to fill for this rule arm.
        let mut choice_matches: HashSet<String> = ["".to_string()].iter().cloned().collect();
        for match_item in choice {
            let mut new_choice_matches = HashSet::new();
            match match_item {
                Match::Char(ch) => {
                    for choice_match in choice_matches.drain() {
                        new_choice_matches.insert(choice_match + &ch.to_string());
                    }
                }
                Match::Rule(num) => {
                    let new_match_particles = expand_rule(rules, *num, cache);
                    for choice_match in choice_matches.drain() {
                        for new_match_particle in &new_match_particles {
                            new_choice_matches.insert(choice_match.clone() + new_match_particle);
                        }
                    }
                }
            }
            choice_matches.clear();
            choice_matches.extend(new_choice_matches);
        }
        matches.extend(choice_matches);
    }
    cache.insert(key, matches.clone());
    matches
}

fn matches_series(message: &str, match_series: &[HashSet<String>]) -> bool {
    let mut startswith = Vec::new();
    for match_item in &match_series[0] {
        if message.starts_with(match_item) {
            startswith.push(match_item);
        }
    }
    for start in startswith {
        let remaining_message = message.strip_prefix(start).unwrap();
        if match_series.len() == 1 {
            return remaining_message.is_empty();
        }
        if matches_series(remaining_message, &match_series[1..]) {
            return true;
        }
    }
    false
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    let mut cache = HashMap::new();
    // Rule 0 is
    //     - 0: 8 11
    // So break it down into matches for 8 and matches for 11: combining these two into a total of
    // sum matches is too expensive, so instead validate by hand that the message:
    //     - Starts with any match from rule 8
    //     - The remaining message after stripping that match is in the matches for rule 11
    let mut match_series = Vec::new();
    for matches in data.rules.get(&0).unwrap() {
        for match_item in matches {
            if let Match::Rule(num) = match_item {
                match_series.push(expand_rule(&data.rules, *num, &mut cache));
            }
        }
    }
    Some(
        data.messages
            .iter()
            .filter(|message| matches_series(message, &match_series))
            .count(),
    )
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    // The new rule 8 and rule 11 are as follows:
    //     8: 42 | 42 8
    // This allows any pattern which is (infinite) combinations of any string matching rule 42.
    //     11: 42 31 | 42 11 31
    // This allows any pattern which is (infinite) combinations of any string matching rule 42
    // followed by the *same* number of strings matching 31.
    // Rule 0 is still:
    //     0: 8 11
    // The sum total of these rules means to verify a given message:
    //     - Strip any prefixes from rule 42, keeping count.
    //     - Strip any prefixes from rule 31, keeping count.
    //     - If the string is not empty, it does not match.
    //     - Ensure that the number of rule 42 matches was greater than the number of rule 31
    //       matches, which is what you would expect if had 1 (or more) matching 42 to satisfy rule
    //       8, then as many 42 and 31 matches following.
    let mut cache = HashMap::new();
    let rule_42_matches = expand_rule(&data.rules, 42, &mut cache);
    let rule_31_matches = expand_rule(&data.rules, 31, &mut cache);

    Some(
        data.messages
            .iter()
            .filter(|message| {
                let mut num_42: u32 = 0;
                let mut num_31: u32 = 0;
                let mut remaining_message: String = message.clone().to_string();

                let mut any_42_matches = true;
                while any_42_matches {
                    any_42_matches = false;
                    for match_item in &rule_42_matches {
                        if remaining_message.starts_with(match_item) {
                            any_42_matches = true;
                            num_42 += 1;
                            remaining_message = remaining_message
                                .strip_prefix(match_item)
                                .unwrap()
                                .to_string();
                        }
                    }
                }

                let mut any_31_matches = true;
                while any_31_matches {
                    any_31_matches = false;
                    for match_item in &rule_31_matches {
                        if remaining_message.starts_with(match_item) {
                            any_31_matches = true;
                            num_31 += 1;
                            remaining_message = remaining_message
                                .strip_prefix(match_item)
                                .unwrap()
                                .to_string();
                        }
                    }
                }

                if !remaining_message.is_empty() {
                    return false;
                }
                return num_42 > num_31 && num_31 > 0;
            })
            .count(),
    )
}

fn get_data(input: String) -> Result<ChallengeData, io::Error> {
    let (rule_strs, messages) = input.split("\n\n").next_tuple().unwrap();
    let rules: Rules = rule_strs
        .lines()
        .map(|s| {
            let (key, rule_str) = s.split(':').map(|s| s.trim()).next_tuple().unwrap();
            let key: u32 = key.parse().unwrap();
            let rule: Rule = rule_str
                .split(" | ")
                .map(|s| s.trim().split(' ').map(|s| s.parse().unwrap()).collect())
                .collect();
            (key, rule)
        })
        .collect();
    Ok(InputData {
        rules,
        messages: messages.lines().map(|s| s.to_string()).collect(),
    })
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
        let input = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb"
            .to_string();

        let data = get_data(input.to_string()).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data), Some(2));
    }

    #[test]
    fn test_given_example_part_two() {
        let input = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
            .to_string();

        let data = get_data(input.to_string()).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data), Some(3));
        assert_eq!(part_two(&data), Some(12));
    }
}
