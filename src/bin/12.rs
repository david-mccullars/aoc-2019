#[allow(unused_imports)]
use advent_of_code::*;
use core::cmp::Ordering;
use itertools::Itertools;
use num::integer::lcm;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<i32> {
    _part_one(input, 1000)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut moons = Moons::parse(input);
    Some(moons.cycle_len())
}

fn _part_one(input: &str, n: usize) -> Option<i32> {
    let mut moons = Moons::parse(input);
    for _ in 0..n {
        moons.step();
    }
    Some(moons.energy())
}

#[derive(Default)]
struct Moons {
    x: Moons1d,
    y: Moons1d,
    z: Moons1d,
}

impl Moons {
    fn parse(input: &str) -> Self {
        let parser = parser!(lines("<x=" i32 ", y=" i32 ", z=" i32 ">"));
        let mut moons = Self::default();
        for (x, y, z) in parser.parse(input).unwrap() {
            moons.x.add(x, 0);
            moons.y.add(y, 0);
            moons.z.add(z, 0);
        }
        moons
    }

    fn step(&mut self) {
        self.x.step();
        self.y.step();
        self.z.step();
    }

    fn energy(&self) -> i32 {
        let mut sum = 0;
        for i in 0..self.x.pos.len() {
            let pe = self.x.pos[i].abs() + self.y.pos[i].abs() + self.z.pos[i].abs();
            let ke = self.x.vel[i].abs() + self.y.vel[i].abs() + self.z.vel[i].abs();
            sum += pe * ke;
        }
        sum
    }

    fn cycle_len(&mut self) -> usize {
        let x = self.x.cycle_len().unwrap();
        let y = self.y.cycle_len().unwrap();
        let z = self.z.cycle_len().unwrap();
        lcm(lcm(x, y), z)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Moons1d {
    pos: Vec<i32>,
    vel: Vec<i32>,
}

impl Moons1d {
    fn add(&mut self, pos: i32, vel: i32) {
        self.pos.push(pos);
        self.vel.push(vel);
    }

    fn step(&mut self) {
        self.apply_gravity_to_vel();
        self.apply_vel_to_pos();
    }

    fn apply_gravity_to_vel(&mut self) {
        for (i1, i2) in (0..self.pos.len()).tuple_combinations() {
            match self.pos[i1].cmp(&self.pos[i2]) {
                Ordering::Less => {
                    self.vel[i1] += 1;
                    self.vel[i2] -= 1;
                }
                Ordering::Greater => {
                    self.vel[i1] -= 1;
                    self.vel[i2] += 1;
                }
                _ => {}
            }
        }
    }

    fn apply_vel_to_pos(&mut self) {
        for i in 0..self.pos.len() {
            self.pos[i] += self.vel[i];
        }
    }

    fn cycle_len(&mut self) -> Option<usize> {
        let start = self.clone();
        for i in 1..1_000_000 {
            self.step();
            if self == &start {
                return Some(i);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_b() {
        let result = _part_one(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(179));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2772));
    }
}
