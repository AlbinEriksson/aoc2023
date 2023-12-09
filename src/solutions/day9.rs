use super::Solver;

type Value = i64;

struct History {
    values: Vec<Value>,
}

pub struct Day9 {
    histories: Vec<History>,
}

struct Pascal {
    rows: Vec<Vec<Value>>,
}

const INPUT: &str = include_str!("../../input/day9");

impl Pascal {
    fn new() -> Pascal {
        Pascal { rows: vec![vec![1]] }
    }

    fn add_row(&mut self, depth: usize) {
        if depth < self.rows.len() {
            return;
        }
        self.add_row(depth - 1);
        let new_row = (0..=depth)
            .map(|index| self.get(depth - 1, index as isize - 1) + self.get(depth - 1, index as isize))
            .collect();
        self.rows.push(new_row);
    }

    fn get(&mut self, depth: usize, index: isize) -> Value {
        if index < 0 || index > depth as isize {
            return 0;
        } else if depth >= self.rows.len() {
            self.add_row(depth);
        }
        self.rows[depth][index as usize]
    }
}

fn odd_sign(x: usize) -> Value {
    1 - (x as Value % 2) * 2
}

impl Day9 {
    #[allow(dead_code)]
    fn solve_intuitive<TermFn>(&self, term: TermFn) -> Value
    where
        TermFn: Fn(usize, &Vec<Value>) -> Value,
    {
        // My first solution. Calculates all the differential sequences until it's all zeroes, then adds terms
        // from the left (part 2) or right (part 1) side of the sequences.
        // Part 2 subtracts every other term instead of adding all of them.
        self.histories
            .iter()
            .map(|history| {
                let mut sequence: Vec<Value> = history.values.clone();
                let mut depth = 0;
                let mut extrapolation = term(depth, &sequence);
                loop {
                    let diff = sequence.windows(2).map(|x| x[1] - x[0]).collect();
                    depth += 1;
                    sequence = diff;
                    extrapolation += term(depth, &sequence);
                    if sequence.iter().all(|&value| value == 0) {
                        break;
                    }
                }
                extrapolation
            })
            .sum()
    }

    fn solve_pascal<TermFn>(&self, term: TermFn) -> Value
    where
        TermFn: Fn(usize, usize, &mut Pascal) -> Value,
    {
        // My second solution, approx 2-3 times faster. Exploits the fact that when solving the problem algebraically, the
        // solution can be reduced to a linear combination of values in the input multiplied by terms in Pascal's triangle.
        // Check this out:
        // a b c d e f   V
        //  g h i j k   U
        //   l m n o   Z
        //    p q r   Y
        //     s t   X
        //      0   0
        // Solving for V yields:
        // V = f + U
        // V = f + (k + Z)
        // V = f + (k + (o + Y))
        // V = f + (k + (o + (r + X)))
        // V = f + (k + (o + (r + (t))))
        // V = f + (k + (o + (r + (r - q))))
        // V = f + (k + (o + (2r - q)))
        // V = f + (k + (o + (2o - 2n - n + m)))
        // V = f + (k + (3o - 3n + m))
        // V = f + (k + (3k - 3j - 3j + 3i + i - h))
        // V = f + (4k - 6j + 4i - h)
        // V = f + (4f - 4e - 6e + 6d + 4d - 4c - c + b)
        // V = 5f - 10e + 10d - 5c + b
        // Which consists of coefficients from the 6th row of Pascal's triangle. So, instead of calculating all rows starting
        // from g for EVERY INPUT, we can just calculate Pascal's triangle ONE TIME. It turns out that this is fast.

        let mut pascal = Pascal::new();
        self.histories
            .iter()
            .map(|history| {
                let depth = history.values.len();
                history
                    .values
                    .iter()
                    .enumerate()
                    .map(|(index, value)| value * term(depth, index, &mut pascal))
                    .sum::<Value>()
            })
            .sum()
    }
}

impl Solver for Day9 {
    type Solution1 = Value;
    type Solution2 = Value;

    fn new() -> Self {
        Day9 { histories: vec![] }
    }

    fn reset(&mut self) {
        self.histories.clear();
    }

    fn parse_input(&mut self) {
        self.histories = INPUT
            .lines()
            .map(|line| {
                let values = line
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|value| value.parse().unwrap())
                    .collect();
                History { values }
            })
            .collect();
    }

    fn solve_part1(&self) -> Value {
        // self.solve_intuitive(|_, sequence| *sequence.last().unwrap())
        self.solve_pascal(|depth, index, pascal| -pascal.get(depth, index as isize) * odd_sign(index + depth))
    }

    fn solve_part2(&self) -> Value {
        // self.solve_intuitive(|index, sequence| odd_sign(index) * *sequence.first().unwrap())
        self.solve_pascal(|depth, index, pascal| -pascal.get(depth, index as isize + 1) * odd_sign(index + depth))
    }

    fn print_solutions(&self, part1: Value, part2: Value) {
        println!("Sum of extrapolated values: {part1}");
        println!("Sum of backwards extrapolated values: {part2}");
    }
}
