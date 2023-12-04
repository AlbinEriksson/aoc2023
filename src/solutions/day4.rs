use crate::util::{intset::IntSet, set::Set};

use super::Solver;

struct Card {
    winning_numbers: Vec<u8>,
    drawn_numbers: Vec<u8>,
}

pub struct Day4 {
    cards: Vec<Card>,
}

const INPUT: &str = include_str!("../../input/day4");

impl Card {
    fn count_matching_numbers(&self) -> usize {
        let mut winning_set = IntSet::new(0, 100);
        self.winning_numbers.iter().for_each(|&num| winning_set.add(num.into()));
        self.drawn_numbers
            .iter()
            .filter(|&&num| winning_set.contains(num.into()))
            .count()
    }
}

impl Day4 {
    fn parse_numbers(numbers: &str) -> Vec<u8> {
        numbers
            .trim()
            .split(' ')
            .filter(|num| !num.is_empty())
            .map(|num| num.parse().unwrap())
            .collect()
    }
}

impl Solver for Day4 {
    type Solution1 = u32;
    type Solution2 = u32;

    fn new() -> Self {
        Day4 { cards: vec![] }
    }

    fn reset(&mut self) {
        self.cards.clear();
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            let (_, numbers) = line.split_once(':').unwrap();
            let (winning_numbers, drawn_numbers) = numbers.split_once('|').unwrap();
            let winning_numbers = Day4::parse_numbers(winning_numbers);
            let drawn_numbers = Day4::parse_numbers(drawn_numbers);
            self.cards.push(Card {
                winning_numbers,
                drawn_numbers,
            })
        }
    }

    fn solve_part1(&self) -> u32 {
        self.cards
            .iter()
            .map(|card| card.count_matching_numbers())
            .filter(|&matches| matches > 0)
            .map(|matches| 1 << (matches - 1))
            .sum()
    }

    fn solve_part2(&self) -> u32 {
        let mut card_amounts = vec![1u32; self.cards.len()];
        self.cards.iter().enumerate().for_each(|(index, card)| {
            let matches = card.count_matching_numbers();
            for i in 1..=matches {
                card_amounts[index + i] += card_amounts[index];
            }
        });
        card_amounts.iter().sum()
    }

    fn print_solutions(&self, part1: u32, part2: u32) {
        println!("Sum of scratchcard scores: {part1}");
        println!("Number of scratchcard copies: {part2}");
    }
}
