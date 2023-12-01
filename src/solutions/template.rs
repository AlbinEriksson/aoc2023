use super::Solver;

#[derive(Debug)]
pub struct Day1 {
}

const INPUT: &str = include_str!("../../input/day1");

impl Day1 {
}

impl Solver for Day1 {
    type Solution1 = u32;
    type Solution2 = ();

    fn new() -> Self {
    }

    fn reset(&mut self) {
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {

        }
    }

    fn solve_part1(&self) -> u32 {
        0
    }

    fn solve_part2(&self) {
        
    }

    fn print_solutions(&self, part1: u32, part2: ()) {
        println!("Sum of calibration values: {part1}");
        // println!("Sum of 3 most calories carried: {part2}");
    }
}
