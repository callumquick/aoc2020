/// Solution to Advent of Code Challenge Day 04.
use aoc2020::{get_day_input, print_elapsed_time};
use std::collections::HashMap;
use std::io;
use std::iter::FromIterator;
use std::str::FromStr;

const DAYNUM: &'static str = "04";
type ChallengeData = Vec<PassportData>;
type ChallengeOut = u32;

/// Structure representing passport data, which may or may not be fully filled.
#[derive(Debug)]
struct PassportData {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl FromStr for PassportData {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: HashMap<&str, String> = HashMap::from_iter(s.split_whitespace().map(|s| {
            let vec: Vec<_> = s.split(':').collect();
            let (k, v) = match &vec[..] {
                &[first, second, ..] => (first, second.to_string()),
                _ => unreachable!(),
            };
            (k, v)
        }));
        Ok(PassportData {
            byr: map.get("byr").map(|s| s.clone()),
            iyr: map.get("iyr").map(|s| s.clone()),
            eyr: map.get("eyr").map(|s| s.clone()),
            hgt: map.get("hgt").map(|s| s.clone()),
            hcl: map.get("hcl").map(|s| s.clone()),
            ecl: map.get("ecl").map(|s| s.clone()),
            pid: map.get("pid").map(|s| s.clone()),
            cid: map.get("cid").map(|s| s.clone()),
        })
    }
}

impl PassportData {
    fn is_valid1(&self) -> bool {
        match self {
            Self {
                byr: Some(_),
                iyr: Some(_),
                eyr: Some(_),
                hgt: Some(_),
                hcl: Some(_),
                ecl: Some(_),
                pid: Some(_),
                cid: _,
            } => true,
            _ => false,
        }
    }

    fn byr_valid(&self) -> bool {
        match &self.byr {
            Some(byr) => byr
                .parse::<u32>()
                .map(|num| num >= 1920 && num <= 2002)
                .unwrap_or(false),
            None => false,
        }
    }

    fn iyr_valid(&self) -> bool {
        match &self.iyr {
            Some(iyr) => iyr
                .parse::<u32>()
                .map(|num| num >= 2010 && num <= 2020)
                .unwrap_or(false),
            None => false,
        }
    }

    fn eyr_valid(&self) -> bool {
        match &self.eyr {
            Some(eyr) => eyr
                .parse::<u32>()
                .map(|num| num >= 2020 && num <= 2030)
                .unwrap_or(false),
            None => false,
        }
    }

    fn hgt_valid(&self) -> bool {
        match &self.hgt {
            Some(hgt) => {
                if hgt.ends_with("cm") {
                    hgt.strip_suffix("cm")
                        .unwrap()
                        .parse::<u32>()
                        .map(|cm| cm >= 150 && cm <= 193)
                        .unwrap_or(false)
                } else if hgt.ends_with("in") {
                    hgt.strip_suffix("in")
                        .unwrap()
                        .parse::<u32>()
                        .map(|inch| inch >= 59 && inch <= 76)
                        .unwrap_or(false)
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn hcl_valid(&self) -> bool {
        match &self.hcl {
            Some(hcl) => {
                if let Some(hex) = hcl.strip_prefix('#') {
                    hex.chars()
                        .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase())
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn ecl_valid(&self) -> bool {
        match &self.ecl {
            Some(ecl) => match ecl.as_str() {
                "amb" => true,
                "blu" => true,
                "brn" => true,
                "gry" => true,
                "grn" => true,
                "hzl" => true,
                "oth" => true,
                _ => false,
            },
            None => false,
        }
    }

    fn pid_valid(&self) -> bool {
        match &self.pid {
            Some(pid) => pid.len() == 9 && pid.chars().all(|c| c.is_digit(10)),
            None => false,
        }
    }

    fn is_valid2(&self) -> bool {
        self.byr_valid()
            && self.iyr_valid()
            && self.eyr_valid()
            && self.hgt_valid()
            && self.hcl_valid()
            && self.ecl_valid()
            && self.pid_valid()
    }
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    Some(data.iter().map(|p| p.is_valid1() as u32).sum())
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    Some(data.iter().map(|p| p.is_valid2() as u32).sum())
}

fn get_data(input: String) -> Result<ChallengeData, io::Error> {
    input.split("\n\n").map(|s| s.parse()).collect()
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
    fn test_given_example_part_one() {
        let input: String = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
            .to_string();
        let data = get_data(input).expect("Couldn't convert test input");

        // Check each gives the right answer.
        assert_eq!(part_one(&data), Some(2));
    }

    #[test]
    fn test_given_example_part_two() {
        let invalid_input: String = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
            .to_string();
        let invalid_data = get_data(invalid_input).expect("Couldn't convert test input");

        // Check none of the passports are given as valid.
        assert_eq!(part_two(&invalid_data), Some(0));

        let input: String = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022"
            .to_string();
        let data = get_data(input).expect("Couldn't convert test input");

        // Check all of the passports are given as valid.
        assert_eq!(part_two(&data), Some(3));
    }
}
