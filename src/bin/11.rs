#[allow(unused_imports)]
use advent_of_code::*;
use advent_of_code_ocr::parse_string_to_letters;
use hashbrown::HashMap;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::ops::RangeInclusive;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let mut robot = Robot::new();
    let mut intcode = Intcode::new(input);
    intcode.run_with_io(&mut robot);
    Some(robot.panels_painted())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut robot = Robot::new();
    robot.paint(1);
    let mut intcode = Intcode::new(input);
    intcode.run_with_io(&mut robot);
    Some(parse_string_to_letters(&robot.string()))
}

struct Robot {
    painted: HashMap<(isize, isize), u8>,
    pos: (isize, isize),
    dir: u8,
    paint_next: bool,
}

impl Robot {
    fn new() -> Self {
        let painted = HashMap::new();
        let pos = (0, 0);
        let dir = 0; // Facing up
        let paint_next = true;
        Self {
            painted,
            pos,
            dir,
            paint_next,
        }
    }

    fn paint(&mut self, value: u8) {
        self.painted.entry(self.pos).insert(value);
    }

    fn mv(&mut self) {
        self.pos = match self.dir {
            0 => (self.pos.0, self.pos.1 - 1),
            1 => (self.pos.0 + 1, self.pos.1),
            2 => (self.pos.0, self.pos.1 + 1),
            3 => (self.pos.0 - 1, self.pos.1),
            _ => panic!("Invalid direction"),
        }
    }

    fn turn(&mut self, value: u8) {
        self.dir = if value == 0 {
            self.dir + 3
        } else {
            self.dir + 1
        } % 4;
    }

    fn panels_painted(&self) -> usize {
        self.painted.len()
    }

    fn range(&self, mapping: fn(&(isize, isize)) -> isize) -> RangeInclusive<isize> {
        match self
            .painted
            .iter()
            .filter_map(|(pos, color)| {
                if *color == 1 {
                    Some(mapping(pos))
                } else {
                    None
                }
            })
            .minmax()
        {
            MinMax(min, max) => min..=max,
            _ => panic!("Invalid range"),
        }
    }

    fn string(&self) -> String {
        let x_range = self.range(|pos| pos.0);
        let y_range = self.range(|pos| pos.1);
        y_range
            .map(|y| {
                x_range
                    .clone()
                    .map(|x| {
                        if self.painted.get(&(x, y)) == Some(&1) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .join("")
            })
            .join("\n")
    }
}

impl IntcodeIO for Robot {
    fn input(&mut self) -> Option<i64> {
        Some((*self.painted.get(&self.pos).unwrap_or(&0)).into())
    }

    fn output(&mut self, value: i64) {
        if value < 0 {
            panic!("Value should not be negative");
        }
        if self.paint_next {
            self.paint(value as u8);
        } else {
            self.turn(value as u8);
            self.mv();
        }
        self.paint_next = !self.paint_next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2594));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("AKERJFHK")));
    }
}
