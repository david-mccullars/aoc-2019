#[allow(unused_imports)]
use advent_of_code::*;
use core::cmp::Ordering;
use hashbrown::HashSet;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let mut arcade = Arcade::new();
    let mut intcode = Intcode::new(input);
    intcode.run_with_io(&mut arcade);
    Some(arcade.blocks.len())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut arcade = Arcade::new();
    let mut intcode = Intcode::new(input);
    intcode.set_code(0, 2); // Play for free
    intcode.run_with_io(&mut arcade);
    assert_eq!(arcade.blocks.len(), 0);
    Some(arcade.score)
}

struct Arcade {
    ball: (i64, i64),
    paddle: (i64, i64),
    blocks: HashSet<(i64, i64)>,
    walls: HashSet<(i64, i64)>,
    x: Option<i64>,
    y: Option<i64>,
    score: i64,
}

impl Arcade {
    fn new() -> Self {
        Self {
            ball: (0, 0),
            paddle: (0, 0),
            blocks: HashSet::new(),
            walls: HashSet::new(),
            x: None,
            y: None,
            score: 0,
        }
    }

    fn draw(&mut self, value: i64) {
        let pos = (self.x.unwrap(), self.y.unwrap());
        if pos == (-1, 0) {
            self.score = value;
        } else {
            match value {
                0 => {
                    self.blocks.remove(&pos);
                    self.walls.remove(&pos);
                }
                1 => {
                    self.walls.insert(pos);
                }
                2 => {
                    self.blocks.insert(pos);
                }
                3 => {
                    self.paddle = pos;
                }
                4 => {
                    self.ball = pos;
                }
                _ => panic!("Invalid value"),
            }
        }
    }
}

impl IntcodeIO for Arcade {
    fn input(&mut self) -> Option<i64> {
        Some(match self.paddle.0.cmp(&self.ball.0) {
            Ordering::Less => 1,     // Move joystick right
            Ordering::Greater => -1, // Move joystick left
            _ => 0,                  // Leave joystick alone
        })
    }

    fn output(&mut self, value: i64) {
        if self.x.is_none() {
            self.x = Some(value);
        } else if self.y.is_none() {
            self.y = Some(value);
        } else {
            self.draw(value);
            self.x = None;
            self.y = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(207));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10247));
    }
}
