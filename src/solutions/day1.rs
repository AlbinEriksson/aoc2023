use super::Solver;

pub struct Day1<'a> {
    lines: Vec<&'a str>,
}

const INPUT: &str = include_str!("../../input/day1");

fn get_digit<Iter>(iter: &mut Iter) -> Option<u32>
where
    Iter: Iterator<Item = char> + Sized,
{
    iter.find(|ch| ch.is_ascii_digit())?.to_digit(10)
}

const DIGITS: &[&str] = &["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn get_real_digit<Iter>(line: &str, search_order: &mut Iter) -> Option<u32>
where
    Iter: Iterator<Item = usize>,
{
    let chars: Vec<char> = line.chars().collect();
    search_order.find_map(|start_index| {
        chars[start_index].to_digit(10).or_else(|| {
            DIGITS.iter().enumerate().find_map(|(value, digit_str)| {
                if chars.len() - start_index < digit_str.len() {
                    return None;
                }
                if digit_str
                    .chars()
                    .enumerate()
                    .all(|(index, ch)| chars[start_index + index] == ch)
                {
                    Some(value as u32)
                } else {
                    None
                }
            })
        })
    })
}

impl<'a> Solver for Day1<'a> {
    type Solution1 = u32;
    type Solution2 = u32;

    fn new() -> Self {
        Day1 { lines: vec![] }
    }

    fn reset(&mut self) {
        self.lines.clear();
    }

    fn parse_input(&mut self) {
        self.lines = INPUT.lines().collect();
    }

    fn solve_part1(&self) -> u32 {
        self.lines
            .iter()
            .map(|line| {
                let first_digit = get_digit(&mut line.chars()).unwrap();
                let last_digit = get_digit(&mut line.chars().rev()).unwrap();
                first_digit * 10 + last_digit
            })
            .sum()
    }

    fn solve_part2(&self) -> u32 {
        self.lines
            .iter()
            .map(|line| {
                let first_digit = get_real_digit(line, &mut (0..line.len())).unwrap();
                let last_digit = get_real_digit(line, &mut (0..line.len()).rev()).unwrap();
                first_digit * 10 + last_digit
            })
            .map(|value| {
                println!("{value}");
                value
            })
            .sum()
    }

    fn print_solutions(&self, part1: u32, part2: u32) {
        println!("Sum of calibration values: {part1}");
        println!("Sum of real calibration values: {part2}");
    }
}
