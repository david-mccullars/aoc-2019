#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::HashSet;
use std::hash::Hash;
use std::ops::Range;
use Direction::*;

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<usize> {
    let mut bugs = Bugs::parse(input, |x, y| (x, y));
    let mut seen = HashSet::new();
    loop {
        let bio = bugs.biodiversity(|(x, y)| 1 << (5 * y + x));
        if !seen.insert(bio) {
            return Some(bio);
        }
        bugs.cycle(adjacent_simple);
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    _part_two(input, 200)
}

pub fn _part_two(input: &str, minutes: usize) -> Option<usize> {
    let mut bugs = Bugs::parse(input, |x, y| (x, y, 0));
    for _ in 0..minutes {
        bugs.cycle(adjacent_recursive);
    }
    Some(bugs.current.len())
}

// Avoid a lot of memory allocation by keeping around all of the buffers
// we'll need to process the game of life. "current" is the main element.
#[derive(Default)]
struct Bugs<T> {
    current: HashSet<T>,
    prev: HashSet<T>,
    visited: HashSet<T>,
    to_visit: Vec<(T, bool)>,
    adjacent: Vec<T>,
}

impl<T: Eq + Hash + Copy + Default> Bugs<T> {
    fn parse(input: &str, to_t: impl Fn(isize, isize) -> T) -> Self {
        let mut bugs = Bugs::<T>::default();
        for (y, row) in parser!(lines(any_char+))
            .parse(input)
            .unwrap()
            .iter()
            .enumerate()
        {
            for (x, c) in row.iter().enumerate() {
                if *c == '#' {
                    bugs.current.insert(to_t(x as isize, y as isize));
                }
            }
        }
        bugs
    }

    fn cycle(&mut self, find_adjacent: impl Fn(&T, &mut Vec<T>)) {
        std::mem::swap(&mut self.prev, &mut self.current);
        self.current.clear();

        for pos in &self.prev {
            self.to_visit.push((*pos, true));
        }
        while let Some((pos, is_bug)) = self.to_visit.pop() {
            if self.visited.contains(&pos) {
                continue;
            }
            find_adjacent(&pos, &mut self.adjacent);
            match (self.count_adjacent_bugs(is_bug), is_bug) {
                (1, _) | (2, false) => {
                    self.current.insert(pos);
                }
                _ => {}
            }
            self.visited.insert(pos);
        }
        self.visited.clear();
    }

    fn count_adjacent_bugs(&mut self, is_bug: bool) -> usize {
        self.adjacent
            .iter()
            .filter(|adj_pos| {
                let is_adj_bug = self.prev.contains(*adj_pos);
                if is_bug && !is_adj_bug {
                    self.to_visit.push((**adj_pos, false));
                }
                is_adj_bug
            })
            .count()
    }

    fn biodiversity(&self, score: impl Fn(&T) -> usize) -> usize {
        self.current.iter().map(score).sum()
    }
}

const CARDINAL_DIRS: [Direction; 4] = [North, South, East, West];
const TO5: Range<isize> = 0..5;

fn adjacent_simple(pos: &Pos, adj: &mut Vec<Pos>) {
    adj.clear();
    for dir in CARDINAL_DIRS {
        let pos2 = dir.forward_from(pos);
        if TO5.contains(&pos2.0) && TO5.contains(&pos2.1) {
            adj.push(pos2);
        }
    }
}

type PosWithLevel = (isize, isize, isize);

fn adjacent_recursive(pos: &PosWithLevel, adj: &mut Vec<PosWithLevel>) {
    adj.clear();
    let r = pos.2;
    for dir in CARDINAL_DIRS {
        match (pos.0, pos.1, dir) {
            (1, 2, East) => adj.extend(TO5.map(|y| (0, y, r + 1))),
            (3, 2, West) => adj.extend(TO5.map(|y| (4, y, r + 1))),
            (2, 1, South) => adj.extend(TO5.map(|x| (x, 0, r + 1))),
            (2, 3, North) => adj.extend(TO5.map(|x| (x, 4, r + 1))),

            (0, _, West) => adj.push((1, 2, r - 1)),
            (4, _, East) => adj.push((3, 2, r - 1)),
            (_, 0, North) => adj.push((2, 1, r - 1)),
            (_, 4, South) => adj.push((2, 3, r - 1)),

            _ => {
                let p2 = dir.forward_from(&(pos.0, pos.1));
                adj.push((p2.0, p2.1, r));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2129920));
    }

    #[test]
    fn test_part_two() {
        let result = _part_two(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(99));
    }
}
