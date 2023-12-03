use crate::util::{pos::Pos2d, try_index::TryIndex};

use super::Solver;

#[derive(Debug)]
pub struct Day3 {
    /// 2D Vec of characters in the schematic, indexed like this: `self.schematic[y][x]`
    schematic: Vec<Vec<char>>,
}

const INPUT: &str = include_str!("../../input/day3");

#[derive(Debug)]
struct Number {
    pos: Pos2d<i16>,
    value: i16,
}

fn are_adjacent(a: Pos2d<i16>, b: Pos2d<i16>) -> bool {
    (a - b).abs().max() == 1
}

impl Day3 {
    fn near_symbol(&self, x: i16, y: i16) -> bool {
        for check_y in y - 1..=y + 1 {
            for check_x in x - 1..=x + 1 {
                let is_symbol = self
                    .schematic
                    .try_index(check_y.into())
                    .and_then(|line| line.try_index(check_x.into()))
                    .map_or(false, |&ch| ch != '.' && !ch.is_ascii_digit());
                if is_symbol {
                    return true;
                }
            }
        }
        false
    }

    fn numbers(&self) -> impl Iterator<Item = Number> + '_ {
        self.schematic.iter().enumerate().flat_map(|(y, line)| {
            (0..line.len()).filter_map(move |x| {
                let left = line.try_index(x as isize - 1);
                let right = line[x];
                let left_is_digit = left.map_or(false, |x| x.is_ascii_digit());
                let right_is_digit = right.is_ascii_digit();
                if left_is_digit {
                    None
                } else if right_is_digit {
                    let value = line[x..]
                        .iter()
                        .take_while(|c| c.is_ascii_digit())
                        .fold(0, |acc, c| acc * 10 + c.to_digit(10).unwrap());
                    Some(Number {
                        pos: Pos2d {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        },
                        value: value.try_into().unwrap(),
                    })
                } else {
                    None
                }
            })
        })
    }
}

impl Solver for Day3 {
    type Solution1 = u32;
    type Solution2 = u32;

    fn new() -> Self {
        Day3 { schematic: vec![] }
    }

    fn reset(&mut self) {
        self.schematic.clear();
    }

    fn parse_input(&mut self) {
        self.schematic = INPUT.lines().map(|line| line.chars().collect()).collect();
    }

    fn solve_part1(&self) -> u32 {
        self.numbers()
            .filter(|number| {
                self.near_symbol(number.pos.x, number.pos.y)
                    || (number.value >= 10 && self.near_symbol(number.pos.x + 1, number.pos.y))
                    || (number.value >= 100 && self.near_symbol(number.pos.x + 2, number.pos.y))
            })
            .fold(0u32, |acc, number| acc + number.value as u32)
    }

    fn solve_part2(&self) -> u32 {
        let numbers: Vec<Number> = self.numbers().collect();

        self.schematic
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                (0..line.len()).filter_map(move |x| {
                    if line[x] == '*' {
                        Some(Pos2d::<i16> {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        })
                    } else {
                        None
                    }
                })
            })
            .filter_map(|gear| {
                let adjacent_numbers: Vec<&Number> = numbers
                    .iter()
                    .filter(|number| {
                        are_adjacent(number.pos, gear)
                            || (number.value >= 10 && are_adjacent(number.pos.add(1, 0), gear))
                            || (number.value >= 100 && are_adjacent(number.pos.add(2, 0), gear))
                    })
                    .collect();
                if adjacent_numbers.len() == 2 {
                    Some(adjacent_numbers[0].value as u32 * adjacent_numbers[1].value as u32)
                } else {
                    None
                }
            })
            .sum()
    }

    fn print_solutions(&self, part1: u32, part2: u32) {
        println!("Sum of part numbers: {part1}");
        println!("Sum of gear ratios: {part2}");
    }
}
