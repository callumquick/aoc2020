/// Solution to Advent of Code Challenge Day 22.
use aoc2020::{get_day_input, print_elapsed_time};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::io;
use std::str::FromStr;

const DAYNUM: &'static str = "22";
type ChallengeData = [Deck; 2];
type ChallengeOut = u32;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Deck(VecDeque<u16>);

impl FromStr for Deck {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, deck_str) = s.split(":\n").next_tuple().unwrap();
        Ok(Self(deck_str.lines().map(|s| s.parse().unwrap()).collect()))
    }
}

fn calculate_score(winning_hand: &Deck) -> u32 {
    let mut winner_score: u32 = 0;
    for (i, card) in winning_hand.0.iter().rev().enumerate() {
        winner_score += (i as u32 + 1) * (*card as u32);
    }
    winner_score
}

/// Play the round of Recursive Combat.
///
/// Returns whether the round means the end of the game for player 1 due to a recursion-stop or
/// because the decks have run out of cards.
fn play_round(deck1: &mut Deck, deck2: &mut Deck, rounds_seen: &mut HashSet<(Deck, Deck)>) -> bool {
    if deck1.0.is_empty() || deck2.0.is_empty() {
        return true;
    }

    if rounds_seen.contains(&(deck1.clone(), deck2.clone())) {
        // This round has been seen: win is for player 1.
        return true;
    }
    // This is a new matchup: record it
    rounds_seen.insert((deck1.clone(), deck2.clone()));

    let card1 = deck1.0.pop_front().unwrap();
    let card2 = deck2.0.pop_front().unwrap();

    let player1_wins = if deck1.0.len() >= card1 as usize && deck2.0.len() >= card2 as usize {
        let mut subdeck1: Deck = Deck(deck1.0.iter().take(card1 as usize).cloned().collect());
        let mut subdeck2: Deck = Deck(deck2.0.iter().take(card2 as usize).cloned().collect());
        play_game(&mut subdeck1, &mut subdeck2)
    } else {
        card1 > card2
    };

    match player1_wins {
        true => {
            deck1.0.push_back(card1);
            deck1.0.push_back(card2);
        }
        false => {
            deck2.0.push_back(card2);
            deck2.0.push_back(card1);
        }
    }

    false
}

/// Play the game of Recursive Combat with the two starting decks.
///
/// Returns if player1 wins by the criteria that player1 has cards left.
fn play_game(deck1: &mut Deck, deck2: &mut Deck) -> bool {
    // Keep a cache of rounds which have already been played: if the round is seen, the game will
    // end for player1.
    let mut rounds_seen: HashSet<(Deck, Deck)> = HashSet::new();
    let mut end_game = false;

    while !end_game {
        end_game = play_round(deck1, deck2, &mut rounds_seen);
    }

    !deck1.0.is_empty()
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    let mut deck1 = data[0].clone();
    let mut deck2 = data[1].clone();

    while !deck1.0.is_empty() && !deck2.0.is_empty() {
        let card1 = deck1.0.pop_front().unwrap();
        let card2 = deck2.0.pop_front().unwrap();
        match card1 > card2 {
            true => {
                deck1.0.push_back(card1);
                deck1.0.push_back(card2);
            }
            false => {
                deck2.0.push_back(card2);
                deck2.0.push_back(card1);
            }
        }
    }

    let winning_hand = match deck1.0.is_empty() {
        true => &deck2,
        false => &deck1,
    };

    Some(calculate_score(winning_hand))
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    let mut deck1 = data[0].clone();
    let mut deck2 = data[1].clone();

    let winning_hand = match play_game(&mut deck1, &mut deck2) {
        true => &deck1,
        false => &deck2,
    };

    Some(calculate_score(winning_hand))
}

fn get_data(input: String) -> Result<ChallengeData, io::Error> {
    let mut decks = [Deck(VecDeque::new()), Deck(VecDeque::new())];
    let deck_vec: Vec<Deck> = input
        .trim()
        .split("\n\n")
        .map(|s| s.parse::<Deck>().unwrap())
        .collect();
    decks[0] = deck_vec[0].clone();
    decks[1] = deck_vec[1].clone();
    Ok(decks)
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
        let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
        .to_string();

        let data = get_data(input.to_string()).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data), Some(306));
        assert_eq!(part_two(&data), Some(291));
    }
}
