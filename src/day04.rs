use aoc_runner_derive::aoc;

use std::convert::From;

const CARD_SIZE: usize = 5;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Number {
    Unmarked(u64),
    Marked(u64),
}

impl Number {
    pub fn is_marked(&self) -> bool {
        match self {
            Self::Unmarked(_) => false,
            Self::Marked(_) => true,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Card {
    numbers: [Number; CARD_SIZE * CARD_SIZE],
}

impl From<&str> for Card {
    fn from(raw: &str) -> Self {
        let mut card = Card {
            numbers: [Number::Unmarked(0); CARD_SIZE * CARD_SIZE],
        };
        let raw_numbers = raw.split_whitespace().map(|x| x.parse::<u64>().unwrap());

        for (i, x) in raw_numbers.into_iter().enumerate() {
            card.numbers[i] = Number::Unmarked(x);
        }

        card
    }
}

impl Card {
    fn is_bingo_at_range(&self, range: std::ops::Range<usize>, step: usize) -> bool {
        for i in range.step_by(step) {
            if !self.numbers[i].is_marked() {
                return false;
            }
        }
        true
    }

    fn is_bingo(&self, i: usize) -> bool {
        let col = i % CARD_SIZE;
        let row = i / CARD_SIZE;

        let row_range = (CARD_SIZE * row)..(CARD_SIZE * (row + 1));
        let col_range = col..(CARD_SIZE * CARD_SIZE);

        self.is_bingo_at_range(row_range, 1) || self.is_bingo_at_range(col_range, CARD_SIZE)
    }

    fn score(&self) -> u64 {
        self.numbers
            .iter()
            .map(|x| match x {
                Number::Unmarked(value) => *value,
                _ => 0,
            })
            .sum()
    }

    pub fn mark(&mut self, value: u64) -> Option<u64> {
        let position = self
            .numbers
            .iter()
            .position(|x| *x == Number::Unmarked(value));
        if let Some(i) = position {
            self.numbers[i] = Number::Marked(value);
            if self.is_bingo(i) {
                return Some(self.score() * value);
            }
        }

        None
    }
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Card>) {
    let mut chunks = input.split("\n\n");
    // the first chunk contains the numbers to be drawn
    let draws: Vec<u64> = chunks
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    // following chunks contain the bingo cards
    let cards: Vec<Card> = chunks.map(Card::from).collect();

    (draws, cards)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let (draws, mut cards) = parse_input(input);

    for draw in draws.into_iter() {
        for card in cards.iter_mut() {
            if let Some(score) = card.mark(draw) {
                return score;
            };
        }
    }

    0
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> u64 {
    let (draws, mut cards) = parse_input(input);

    let mut latest_winner_score: u64 = 0;
    let mut to_ignore = Vec::<usize>::new();

    for draw in draws.into_iter() {
        for (i, card) in cards.iter_mut().enumerate() {
            if to_ignore.contains(&i) {
                continue;
            }
            if let Some(score) = card.mark(draw) {
                latest_winner_score = score;
                to_ignore.push(i)
            };
        }
    }

    latest_winner_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
        "#;

    #[test]
    fn test_day4_part1() {
        assert_eq!(solve_part1(INPUT), 4512);
    }

    #[test]
    fn test_day4_part2() {
        assert_eq!(solve_part2(INPUT), 1924);
    }
}
