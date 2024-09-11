#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i64> {
    let mut alarm = Alarm::new(input);
    alarm.reset(12, 2);
    Some(alarm.run())
}

pub fn part_two(input: &str) -> Option<usize> {
    _part_two(input, 19690720)
}

pub fn _part_two(input: &str, value: i64) -> Option<usize> {
    Alarm::new(input).find_noun_and_verb(value)
}

#[derive(Clone)]
struct Alarm(Intcode);

impl Alarm {
    fn new(codestr: &str) -> Self {
        Self(Intcode::new(codestr))
    }

    fn reset(&mut self, noun: usize, verb: usize) {
        self.0.set_code(1, noun as i64);
        self.0.set_code(2, verb as i64);
    }

    fn run(&mut self) -> i64 {
        self.0.run_with_no_io();
        self.0.code_at(0)
    }

    fn find_noun_and_verb(&mut self, value: i64) -> Option<usize> {
        let rng = 0..self.0.len();
        rng.clone()
            .cartesian_product(rng)
            .find(|(noun, verb)| {
                let mut alarm = self.clone();
                alarm.reset(*noun, *verb);
                alarm.run() == value
            })
            .map(|(noun, verb)| 100 * noun + verb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(150));
    }

    #[test]
    fn test_part_two() {
        let result = _part_two(&advent_of_code::template::read_file("examples", DAY), 4950);
        assert_eq!(result, Some(708));
    }
}
