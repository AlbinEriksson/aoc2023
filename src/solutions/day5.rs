use std::fmt::Debug;

use super::Solver;

type Seed = i64;
type Length = i64;

#[derive(Clone, Copy)]
struct SeedRange {
    min: Seed,
    length: Length,
}

impl Debug for SeedRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.min, self.min + self.length)
    }
}

#[derive(Debug)]
struct Range {
    dst_start: Seed,
    src_start: Seed,
    length: Length,
}

struct Map {
    ranges: Vec<Range>,
}

pub struct Day5 {
    seeds: Vec<Seed>,
    maps: Vec<Map>,
}

const INPUT: &str = include_str!("../../input/day5");

impl Solver for Day5 {
    type Solution1 = Seed;
    type Solution2 = Seed;

    fn new() -> Self {
        Day5 {
            seeds: vec![],
            maps: vec![],
        }
    }

    fn reset(&mut self) {
        self.seeds.clear();
        self.maps.clear();
    }

    fn parse_input(&mut self) {
        let (seeds, lines) = INPUT.split_once('\n').unwrap();

        let (_, seeds) = seeds.split_once(':').unwrap();
        self.seeds = seeds.trim().split(' ').map(|seed| seed.parse().unwrap()).collect();
        self.maps = lines
            .trim()
            .split("\n\n")
            .map(|map| {
                let (_, ranges) = map.split_once('\n').unwrap();
                let ranges: Vec<Range> = ranges
                    .split('\n')
                    .map(|range| {
                        let (dst_start, rest) = range.split_once(' ').unwrap();
                        let (src_start, length) = rest.split_once(' ').unwrap();
                        let dst_start: Seed = dst_start.parse().unwrap();
                        let src_start: Seed = src_start.parse().unwrap();
                        let length: Length = length.parse().unwrap();
                        Range {
                            dst_start,
                            src_start,
                            length,
                        }
                    })
                    .collect();
                Map { ranges }
            })
            .collect();
    }

    fn solve_part1(&self) -> Seed {
        let mut seeds = self.seeds.clone();
        for map in self.maps.iter() {
            for seed in seeds.iter_mut() {
                let new_value = map
                    .ranges
                    .iter()
                    .find_map(|range| {
                        if *seed < range.src_start || *seed >= range.src_start + range.length {
                            None
                        } else {
                            Some(*seed - range.src_start + range.dst_start)
                        }
                    })
                    .unwrap_or(*seed);
                *seed = new_value;
            }
        }

        *seeds.iter().min().unwrap()
    }

    fn solve_part2(&self) -> Seed {
        let mut seed_ranges: Vec<SeedRange> = self
            .seeds
            .chunks(2)
            .map(|range| SeedRange {
                min: range[0],
                length: range[1],
            })
            .collect();
        for map in self.maps.iter() {
            seed_ranges = seed_ranges
                .iter()
                .flat_map(|seeds| {
                    let (unmapped, mapped) =
                        map.ranges
                            .iter()
                            .fold((vec![*seeds], vec![]), |(unmapped, mut mapped), range| {
                                let mut new_unmapped = vec![];
                                let range_max = range.src_start + range.length;
                                for seeds in unmapped.iter() {
                                    let seeds_max = seeds.min + seeds.length;
                                    let min = std::cmp::max(seeds.min, range.src_start);
                                    let max = std::cmp::min(seeds_max, range_max);
                                    let length = max - min;
                                    if length < 0 {
                                        new_unmapped.push(*seeds);
                                        continue;
                                    }
                                    mapped.push(SeedRange {
                                        min: min - range.src_start + range.dst_start,
                                        length: max - min,
                                    });
                                    if min > seeds.min {
                                        new_unmapped.push(SeedRange {
                                            min: seeds.min,
                                            length: min - seeds.min,
                                        })
                                    }
                                    if max < seeds_max {
                                        new_unmapped.push(SeedRange {
                                            min: max,
                                            length: seeds_max - max,
                                        });
                                    }
                                }
                                (new_unmapped, mapped)
                            });
                    unmapped
                        .iter()
                        .copied()
                        .chain(mapped.iter().copied())
                        .collect::<Vec<SeedRange>>()
                })
                .collect();
        }

        seed_ranges.iter().map(|seeds| seeds.min).min().unwrap()
    }

    fn print_solutions(&self, part1: Seed, part2: Seed) {
        println!("Lowest location number of all seeds: {part1}");
        println!("Lowest location number of all seed ranges: {part2}");
    }
}
