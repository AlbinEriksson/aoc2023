use std::cmp::max;

use super::Solver;

#[derive(Default)]
struct Cubes {
    red: u8,
    green: u8,
    blue: u8,
}

impl Cubes {
    pub fn power(&self) -> u32 {
        (self.red as u32) * (self.green as u32) * (self.blue as u32)
    }
}

struct Game {
    id: u32,
    subsets: Vec<Cubes>,
}

pub struct Day2 {
    games: Vec<Game>,
}

const INPUT: &str = include_str!("../../input/day2");

impl Solver for Day2 {
    type Solution1 = u32;
    type Solution2 = u32;

    fn new() -> Self {
        Day2 { games: vec![] }
    }

    fn reset(&mut self) {
        self.games.clear();
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            let (id, subsets) = line.split_once(": ").unwrap();
            let (_, id) = id.split_once(' ').unwrap();
            let id: u32 = id.parse().unwrap();

            let subsets = subsets
                .split("; ")
                .map(|subset| {
                    let mut cubes = Cubes::default();
                    for cube in subset.split(", ") {
                        let (amount, color) = cube.split_once(' ').unwrap();
                        let amount: u8 = amount.parse().unwrap();
                        match color {
                            "red" => cubes.red = amount,
                            "green" => cubes.green = amount,
                            "blue" => cubes.blue = amount,
                            _ => panic!(),
                        }
                    }
                    cubes
                })
                .collect();

            self.games.push(Game { id, subsets });
        }
    }

    fn solve_part1(&self) -> u32 {
        self.games
            .iter()
            .filter_map(|game| {
                let impossible = game
                    .subsets
                    .iter()
                    .any(|subset| subset.red > 12 || subset.green > 13 || subset.blue > 14);
                if impossible {
                    None
                } else {
                    Some(game.id)
                }
            })
            .sum()
    }

    fn solve_part2(&self) -> u32 {
        self.games
            .iter()
            .map(|game| {
                game.subsets.iter().fold(Cubes::default(), |acc, subset| Cubes {
                    red: max(acc.red, subset.red),
                    green: max(acc.green, subset.green),
                    blue: max(acc.blue, subset.blue),
                })
            })
            .map(|min_cubes| min_cubes.power())
            .sum()
    }

    fn print_solutions(&self, part1: u32, part2: u32) {
        println!("Sum of IDs of possible games: {part1}");
        println!("Sum of minimum power of all games: {part2}");
    }
}
