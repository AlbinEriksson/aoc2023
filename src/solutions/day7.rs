use std::{cmp::Ordering, fmt::Debug};

use crate::util::array::AsArray;

use super::Solver;

const INPUT: &str = include_str!("../../input/day7");

#[derive(PartialEq, Clone)]
struct Card(char);

type Value = u32;

#[derive(PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

pub struct Day7 {
    hands: Vec<Hand>,
}

#[derive(PartialEq, Debug, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Card {
    fn value(&self) -> Value {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '2'..='9' => self.0.to_digit(10).unwrap(),
            _ => panic!(),
        }
    }

    fn value_with_joker(&self) -> Value {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            '2'..='9' => self.0.to_digit(10).unwrap(),
            _ => panic!(),
        }
    }
}

fn n_of_a_kind(n: u32, card_counts: &[u32; 15]) -> Option<u32> {
    card_counts
        .iter()
        .enumerate()
        .rev()
        .find_map(|(value, &count)| (count == n).then_some(value as u32))
}

fn n_and_m_of_a_kind(n: u32, m: u32, card_counts: &[u32; 15]) -> Option<(u32, u32)> {
    let groups = card_counts.iter().enumerate().rev().fold((0, 0), |acc, (value, &count)| {
        if acc.0 == 0 && count == n {
            (value, acc.1)
        } else if acc.1 == 0 && count == m {
            (acc.0, value)
        } else {
            acc
        }
    });
    (groups.0 > 0 && groups.1 > 0).then_some((groups.0 as u32, groups.1 as u32))
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut card_counts = [0u32; 15];
        for card in self.cards.iter() {
            card_counts[card.value() as usize] += 1;
        }
        n_of_a_kind(5, &card_counts)
            .and(Some(HandType::FiveOfAKind))
            .or_else(|| n_of_a_kind(4, &card_counts).and(Some(HandType::FourOfAKind)))
            .or_else(|| n_and_m_of_a_kind(3, 2, &card_counts).and(Some(HandType::FullHouse)))
            .or_else(|| n_of_a_kind(3, &card_counts).and(Some(HandType::ThreeOfAKind)))
            .or_else(|| n_and_m_of_a_kind(2, 2, &card_counts).and(Some(HandType::TwoPair)))
            .or_else(|| n_of_a_kind(2, &card_counts).and(Some(HandType::OnePair)))
            .or_else(|| n_of_a_kind(1, &card_counts).and(Some(HandType::HighCard)))
            .unwrap()
    }

    fn hand_type_with_jokers(&self) -> HandType {
        // The best strategy with jokers is always to add them to the highest amount of one card value that you already have.
        // If you have the same amount of two card values, pick the highest value.
        // Example: 22JJ3, where 22223 is better than 22333 because you have more deuces than threes.
        // Example: AJJJK, where AAAAK is better than AKKKK because you have 1 ace and 1 king, but ace is higher value.
        // Example: J1122, where 21122 is better than 11122 because both hands are full houses, but the first option starts
        // with a higher card.
        let mut card_counts = [0u32; 15];
        let mut jokers = 0;
        for card in self.cards.iter() {
            if card.0 == 'J' {
                jokers += 1;
                continue;
            }
            card_counts[card.value() as usize] += 1;
        }
        *card_counts.iter_mut().max().unwrap() += jokers;
        n_of_a_kind(5, &card_counts)
            .and(Some(HandType::FiveOfAKind))
            .or_else(|| n_of_a_kind(4, &card_counts).and(Some(HandType::FourOfAKind)))
            .or_else(|| n_and_m_of_a_kind(3, 2, &card_counts).and(Some(HandType::FullHouse)))
            .or_else(|| n_of_a_kind(3, &card_counts).and(Some(HandType::ThreeOfAKind)))
            .or_else(|| n_and_m_of_a_kind(2, 2, &card_counts).and(Some(HandType::TwoPair)))
            .or_else(|| n_of_a_kind(2, &card_counts).and(Some(HandType::OnePair)))
            .or_else(|| n_of_a_kind(1, &card_counts).and(Some(HandType::HighCard)))
            .unwrap()
    }
}

#[derive(PartialEq)]
struct TypedHand {
    cards: [Card; 5],
    hand_type: HandType,
    bid: u32,
}

impl TypedHand {
    fn cmp_each_card_with<F>(&self, other: &Self, cmp: F) -> Ordering
    where
        F: Fn(&Card, &Card) -> Ordering,
    {
        self.cards
            .iter()
            .zip(other.cards.iter())
            .find_map(|(a, b)| {
                let ordering = cmp(a, b);
                match ordering {
                    Ordering::Less | Ordering::Greater => Some(ordering),
                    Ordering::Equal => None,
                }
            })
            .unwrap_or(Ordering::Equal)
    }

    fn cmp_each_card(&self, other: &Self) -> Ordering {
        self.cmp_each_card_with(other, |a, b| a.value().cmp(&b.value()))
    }

    fn cmp_each_card_with_joker(&self, other: &Self) -> Ordering {
        self.cmp_each_card_with(other, |a, b| a.value_with_joker().cmp(&b.value_with_joker()))
    }

    fn cmp_to(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type).then_with(|| self.cmp_each_card(other))
    }

    fn cmp_to_with_joker(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then_with(|| self.cmp_each_card_with_joker(other))
    }
}

impl Debug for TypedHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in self.cards.iter() {
            write!(f, "{}", card.0)?;
        }
        write!(f, " {:>4}", self.bid)?;
        write!(f, " {:?}", self.hand_type)
    }
}

fn calc_winnings(hands: &[TypedHand]) -> u32 {
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index as u32 + 1) * hand.bid)
        .sum()
}

impl Solver for Day7 {
    type Solution1 = u32;
    type Solution2 = u32;

    fn new() -> Self {
        Day7 { hands: vec![] }
    }

    fn reset(&mut self) {
        self.hands.clear();
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            let (cards, bid) = line.split_once(' ').unwrap();
            let cards = cards.as_array().map(Card);
            let bid = bid.parse().unwrap();
            self.hands.push(Hand { cards, bid });
        }
    }

    fn solve_part1(&self) -> u32 {
        let mut hands: Vec<TypedHand> = self
            .hands
            .iter()
            .map(|hand| TypedHand {
                cards: hand.cards.clone(),
                hand_type: hand.hand_type(),
                bid: hand.bid,
            })
            .collect();
        hands.sort_by(|a, b| a.cmp_to(b));
        calc_winnings(&hands)
    }

    fn solve_part2(&self) -> u32 {
        let mut hands: Vec<TypedHand> = self
            .hands
            .iter()
            .map(|hand| TypedHand {
                cards: hand.cards.clone(),
                hand_type: hand.hand_type_with_jokers(),
                bid: hand.bid,
            })
            .collect();
        hands.sort_by(|a, b| a.cmp_to_with_joker(b));
        calc_winnings(&hands)
    }

    fn print_solutions(&self, part1: u32, part2: u32) {
        println!("Total winnings: {part1}");
        println!("Total winnings with joker rule: {part2}");
    }
}
