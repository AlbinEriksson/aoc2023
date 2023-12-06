use super::Solver;

struct Race {
    time: u64,
    record: u64,
}

pub struct Day6 {
    races: Vec<Race>,
}

const INPUT: &str = include_str!("../../input/day6");

impl Day6 {
    fn parse_line(line: &str) -> impl Iterator<Item = u64> + '_ {
        line.trim().split(' ').filter(|t| !t.is_empty()).map(|t| t.parse().unwrap())
    }

    fn first_greater_number(num: f64) -> f64 {
        if num == num.floor() {
            num + 1.0
        } else {
            num.ceil()
        }
    }

    fn first_lesser_number(num: f64) -> f64 {
        if num == num.floor() {
            num - 1.0
        } else {
            num.floor()
        }
    }

    fn ways_to_win(race: &Race) -> u64 {
        // v: velocity / time spent charging
        // t: race time
        // d: race record
        // v = (t +- sqrt(t^2 - 4d)) / 2
        // The above formula is derived from d = v * (t - v) and calculates the lower and upper bound velocities which
        // yield a greater distance than d, the race record.

        let determinant = ((race.time * race.time - 4 * race.record) as f64).sqrt();
        let lower_bound = (race.time as f64 - determinant) / 2.0;
        let upper_bound = (race.time as f64 + determinant) / 2.0;

        let lower_bound = Self::first_greater_number(lower_bound);
        let upper_bound = Self::first_lesser_number(upper_bound);

        (upper_bound - lower_bound + 1.0) as u64
    }
}

impl Solver for Day6 {
    type Solution1 = u64;
    type Solution2 = u64;

    fn new() -> Self {
        Day6 { races: vec![] }
    }

    fn reset(&mut self) {
        self.races.clear();
    }

    fn parse_input(&mut self) {
        let (times, records) = INPUT.split_once('\n').unwrap();
        let (_, times) = times.split_once(':').unwrap();
        let (_, records) = records.split_once(':').unwrap();
        let times = Self::parse_line(times);
        let records = Self::parse_line(records);
        self.races = times.zip(records).map(|(time, record)| Race { time, record }).collect();
    }

    fn solve_part1(&self) -> u64 {
        self.races.iter().map(Self::ways_to_win).product()
    }

    fn solve_part2(&self) -> u64 {
        let actual_race = self.races.iter().fold(Race { time: 0, record: 0 }, |acc, race| {
            let time_digits = 10u64.pow(1 + (race.time - 1).ilog10());
            let record_digits = 10u64.pow(1 + (race.record - 1).ilog10());
            let time = acc.time * time_digits + race.time;
            let record = acc.record * record_digits + race.record;
            Race { time, record }
        });
        Self::ways_to_win(&actual_race)
    }

    fn print_solutions(&self, part1: u64, part2: u64) {
        println!("Product of number of ways to beat each record: {part1}");
        println!("Number of ways to beat the actual record: {part2}");
    }
}
