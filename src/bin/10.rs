#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::HashSet;
use num::integer::gcd;
use std::cmp::Ordering;
use std::collections::btree_map::IntoValues;
use std::collections::{BTreeMap, VecDeque};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::new(input);
    let (_pos, most) = grid.best_monitoring_station();
    Some(most)
}

pub fn part_two(input: &str) -> Option<isize> {
    _part_two(input, (13, 17), 200)
}

pub fn _part_two(input: &str, monitoring_station: Pos, steps: usize) -> Option<isize> {
    let grid = Grid::new(input);
    let destroyed = grid.asteroid_destroyed_after(&monitoring_station, steps);
    destroyed.map(|(x, y)| x * 100 + y)
}

struct Grid {
    asteroids: Vec<Pos>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut asteroids = vec![];
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    asteroids.push((col as isize, row as isize));
                }
            }
        }
        Self { asteroids }
    }

    fn best_monitoring_station(&self) -> (Pos, usize) {
        let mut best_pos = (0, 0);
        let mut best_seen = 0;
        for asteroid in &self.asteroids {
            let seen = self.count_asteroids_seen_from(asteroid);
            if seen > best_seen {
                best_pos = *asteroid;
                best_seen = seen;
            }
        }
        (best_pos, best_seen)
    }

    fn count_asteroids_seen_from(&self, pos: &Pos) -> usize {
        let mut seen = HashSet::new();
        for asteroid in &self.asteroids {
            if pos != asteroid {
                seen.insert(Ray::between(pos, asteroid));
            }
        }
        seen.len()
    }

    fn asteroids_from(&self, pos: &Pos) -> IntoValues<Ray, Vec<(isize, isize)>> {
        let mut map = BTreeMap::new();
        for asteroid in &self.asteroids {
            if pos != asteroid {
                let ray = Ray::between(pos, asteroid);
                map.entry(ray).or_insert(vec![]).push(*asteroid);
            }
        }
        for line in map.values_mut() {
            line.sort_by(|a, b| {
                let da = (a.0 - pos.0).abs() + (a.1 - pos.1).abs();
                let db = (b.0 - pos.0).abs() + (b.1 - pos.1).abs();
                db.cmp(&da) // Sort furthest to closest
            });
        }
        map.into_values()
    }

    fn asteroid_destroyed_after(&self, pos: &Pos, steps: usize) -> Option<Pos> {
        let mut targets: VecDeque<Vec<Pos>> = VecDeque::new();
        for target_group in self.asteroids_from(pos) {
            targets.push_back(target_group);
        }

        for step in 1.. {
            if let Some(mut target_group) = targets.pop_front() {
                let target = target_group.pop();
                if step == steps {
                    return target;
                } else if !target_group.is_empty() {
                    targets.push_back(target_group);
                }
            } else {
                break;
            }
        }
        None
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Ray {
    dx: isize,
    dy: isize,
}

impl Ray {
    fn between(pos1: &Pos, pos2: &Pos) -> Self {
        let mut dx = pos2.0 - pos1.0;
        let mut dy = pos1.1 - pos2.1; // Reversed for ease of comprehension
        if dx == 0 {
            dy = dy.signum();
        } else if dy == 0 {
            dx = dx.signum();
        } else {
            let div = gcd(dx, dy);
            dx /= div;
            dy /= div;
        }
        Self { dx, dy }
    }

    fn quadrant(&self) -> usize {
        match (self.dx >= 0, self.dy >= 0) {
            (true, true) => 1,
            (true, false) => 2,
            (false, false) => 3,
            (false, true) => 4,
        }
    }

    fn slope(&self) -> f64 {
        (self.dy as f64) / (self.dx as f64)
    }
}

impl PartialOrd for Ray {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ray {
    fn cmp(&self, other: &Self) -> Ordering {
        self.quadrant().cmp(&other.quadrant()).then_with(||
            // Reverse order
            other.slope().partial_cmp(&self.slope()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_one_c() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_one_d() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(210));
    }

    #[test]
    fn test_part_two_a() {
        let result = _part_two(
            &advent_of_code::template::read_file_part("examples", DAY, 5),
            (8, 3),
            27,
        );
        assert_eq!(result, Some(501));
    }

    #[test]
    fn test_part_two_b() {
        let result = _part_two(
            &advent_of_code::template::read_file_part("examples", DAY, 5),
            (8, 3),
            36,
        );
        assert_eq!(result, Some(1403));
    }

    #[test]
    fn test_part_two_c() {
        let result = _part_two(
            &advent_of_code::template::read_file_part("examples", DAY, 4),
            (11, 13),
            200,
        );
        assert_eq!(result, Some(802));
    }
}
