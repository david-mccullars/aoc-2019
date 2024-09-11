#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::HashMap;
use pathfinding::directed::dijkstra::dijkstra;
use pathfinding::prelude::dijkstra_reach;
use std::ops::Index;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<usize> {
    DonutMaze::new(input).shortest_path("AA", "ZZ")
}

pub fn part_two(input: &str) -> Option<usize> {
    DonutMaze::new(input).shortest_path_recursive("AA", "ZZ")
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Portal {
    label: String,
    outer: bool,
}

impl Portal {
    fn new(c1: char, c2: char, outer: bool) -> Self {
        Self {
            label: format!("{}{}", c1, c2),
            outer,
        }
    }

    fn from_str(label: &str) -> Self {
        Self {
            label: String::from(label),
            outer: true,
        }
    }

    fn invert(&self) -> Self {
        if self.label == "AA" || self.label == "ZZ" {
            self.clone()
        } else {
            Self {
                label: self.label.clone(),
                outer: !self.outer,
            }
        }
    }

    fn can_recurse(&self, level: usize) -> bool {
        match self.label.as_str() {
            "AA" => false,
            "ZZ" => level == 0,
            _ => level > 0 || !self.outer,
        }
    }

    fn recurse(&self, level: usize) -> RecursivePortal {
        if self.label == "AA" || self.label == "ZZ" {
            (self.clone(), level)
        } else if self.outer {
            (self.invert(), level - 1)
        } else {
            (self.invert(), level + 1)
        }
    }
}

type RecursivePortal = (Portal, usize);

#[derive(Default)]
struct DonutMaze {
    map: Vec<Vec<char>>,
    portals: BiMap<Portal, Pos>,
    neighbors: HashMap<Portal, HashMap<Portal, usize>>,
}

impl DonutMaze {
    fn new(input: &str) -> Self {
        Self::default().parse(input).find_portals().find_neighbors()
    }

    fn parse(mut self, input: &str) -> Self {
        self.map = input.lines().map(|line| line.chars().collect()).collect();
        self
    }

    fn find_portals(mut self) -> Self {
        for (y, row) in self.map.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if let Some((portal, pos)) = self.find_portal(pos_from(x, y)) {
                    assert!(!self.portals.contains_key(&portal));
                    self.portals.insert(portal, pos);
                }
            }
        }
        self
    }

    fn find_portal(&self, pos: Pos) -> Option<(Portal, Pos)> {
        let c2 = self[pos];
        if !c2.is_alphabetic() {
            return None;
        }

        let outer = self.is_outer(&pos);
        for dir in [Direction::West, Direction::North] {
            let pos2 = dir + &pos;
            let c1 = self[pos2];
            if !c1.is_alphabetic() {
                continue;
            }

            let mut portal_pos = dir + &pos2;
            if self[portal_pos] != '.' {
                portal_pos = dir - &pos;
                assert_eq!(self[portal_pos], '.');
            }
            return Some((Portal::new(c1, c2, outer), portal_pos));
        }

        None
    }

    fn is_outer(&self, pos: &Pos) -> bool {
        pos.0 == 1
            || pos.0 + 1 == (self.map[0].len() as isize)
            || pos.1 == 1
            || pos.1 + 1 == (self.map.len() as isize)
    }

    fn find_neighbors(mut self) -> Self {
        let mut neighbors = HashMap::new();
        for (portal, start) in self.portals.iter() {
            let start_neighbors = neighbors.entry(portal.clone()).or_insert_with(HashMap::new);
            self.find_neighbors_from(start, start_neighbors);
        }
        self.neighbors = neighbors;
        self
    }

    fn find_neighbors_from(&self, start: &Pos, neighbors: &mut HashMap<Portal, usize>) {
        for node in dijkstra_reach(start, |&pos, _| {
            let limit = if &pos != start && self.portals.contains_value(&pos) {
                0
            } else {
                4
            };
            DIRECTIONS.iter().take(limit).filter_map(move |dir| {
                let pos2 = dir.forward_from(&pos);
                (self[pos2] == '.').then_some((pos2, 1))
            })
        }) {
            if &node.node != start && self.portals.contains_value(&node.node) {
                if let Some(portal) = self.portals.iget(&node.node) {
                    neighbors.insert(portal.clone(), node.total_cost);
                }
            }
        }
    }

    fn shortest_path(&self, start: &str, finish: &str) -> Option<usize> {
        let start = Portal::from_str(start);
        let finish = Portal::from_str(finish);
        let successors = |portal: &Portal| {
            self.neighbors
                .get(portal)
                .unwrap()
                .iter()
                .map(|(portal2, cost)| (portal2.invert(), cost + 1))
        };
        let success = |portal: &Portal| portal == &finish;

        dijkstra(&start, successors, success).map(|node| node.1 - 1)
    }

    fn shortest_path_recursive(&self, start: &str, finish: &str) -> Option<usize> {
        let start = Portal::from_str(start).recurse(0);
        let finish = Portal::from_str(finish).recurse(0);
        let successors = |node: &RecursivePortal| {
            let level = node.1;
            self.neighbors
                .get(&node.0)
                .unwrap()
                .iter()
                .filter(move |(portal2, _)| portal2.can_recurse(level))
                .map(move |(portal2, cost)| (portal2.recurse(level), cost + 1))
        };
        let success = |portal: &RecursivePortal| portal == &finish;

        dijkstra(&start, successors, success).map(|node| node.1 - 1)
    }
}

impl Index<Pos> for DonutMaze {
    type Output = char;

    fn index(&self, pos: Pos) -> &Self::Output {
        if pos.0 < 0 || pos.1 < 0 || pos.1 >= (self.map.len() as isize) {
            return &'#';
        }
        let row = &self.map[pos.1 as usize];
        if pos.0 >= (row.len() as isize) {
            &'#'
        } else {
            &row[pos.0 as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(58));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(396));
    }
}
