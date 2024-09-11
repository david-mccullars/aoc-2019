#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::HashMap;
use std::cmp::min;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    let mut droid = RepairDroid::new(input);
    droid.sweep();
    droid.dist_to_o2()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut droid = RepairDroid::new(input);
    droid.sweep();
    droid.longest_dist_from_o2()
}

#[derive(Clone, Copy)]
enum Status {
    HitWall,
    Moved,
    OxygenSystemFound,
}

impl Status {
    fn from(value: Option<i64>) -> Self {
        match value {
            Some(0) => Status::HitWall,
            Some(1) => Status::Moved,
            Some(2) => Status::OxygenSystemFound,
            _ => panic!("Invalid status: {:?}", value),
        }
    }
}

const WALL: usize = usize::MAX;

#[derive(Clone)]
struct RepairDroid {
    map: HashMap<Pos, usize>,
    o2_pos: Option<Pos>,
    o2_path: Option<Vec<Direction>>,
    pos: Pos,
    intcode: Intcode,
}

impl RepairDroid {
    fn new(codestr: &str) -> Self {
        let map = HashMap::new();
        let o2_pos = None;
        let o2_path = None;
        let pos = (0, 0);
        let mut intcode = Intcode::new(codestr);
        intcode.halt_after_output(true);
        Self {
            map,
            o2_pos,
            o2_path,
            pos,
            intcode,
        }
    }

    fn sweep(&mut self) {
        let mut path = vec![];
        self._sweep(&mut path);
    }

    fn _sweep(&mut self, path: &mut Vec<Direction>) {
        for dir in &DIRECTIONS {
            let pos2 = dir.forward_from(&self.pos);
            path.push(*dir);
            match self.map.get(&pos2) {
                Some(&WALL) => {}
                Some(_) => {
                    self.track(&pos2, path);
                }
                None => {
                    if self.mv(dir) {
                        self.track(&pos2, path);
                        self._sweep(path);
                        self.mv(&dir.invert());
                    } else {
                        self.map.insert(pos2, WALL);
                    }
                }
            }
            path.pop();
        }
    }

    fn mv(&mut self, dir: &Direction) -> bool {
        let pos2 = dir.forward_from(&self.pos);
        match self.run_intcode(*dir) {
            Status::HitWall => {
                return false;
            }
            Status::Moved => {}
            Status::OxygenSystemFound => {
                self.o2_pos = Some(pos2);
            }
        };
        self.pos = pos2;
        true
    }

    fn track(&mut self, pos: &Pos, path: &[Direction]) {
        let shortest = self.map.entry(*pos).or_insert(WALL);
        *shortest = min(*shortest, path.len());

        if self.o2_pos == Some(*pos) && path.len() == *shortest {
            self.o2_path = Some(path.to_vec());
        }
    }

    fn dist_to_o2(&self) -> Option<usize> {
        self.o2_pos.map(|o2| self.map.get(&o2).unwrap()).copied()
    }

    fn longest_dist_from_o2(&mut self) -> Option<usize> {
        // Move to O2
        for dir in self.o2_path.clone().as_ref().unwrap() {
            self.mv(dir);
        }
        // Clear everything in the map except walls
        self.map.retain(|_, len| *len == WALL);
        self.sweep();
        self.map.values().filter(|v| **v < WALL).max().copied()
    }

    fn run_intcode(&mut self, dir: Direction) -> Status {
        let input = match dir {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        };
        Status::from(self.intcode.run_simple(&[input]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(224));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(284));
    }
}
