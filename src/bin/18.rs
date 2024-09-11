#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::HashMap;
use pathfinding::directed::dijkstra::dijkstra;
use pathfinding::prelude::dijkstra_reach;
use std::hash::Hash;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    Map::new(input).shortest_path()
}

pub fn part_two(input: &str) -> Option<u32> {
    Map4::new(input).shortest_path()
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct WithKeys<T: Clone + Eq + Hash> {
    item: T,
    keys: u32,
}

impl<T: Clone + Eq + Hash> WithKeys<T> {
    fn new(item: T) -> Self {
        Self { item, keys: 0 }
    }

    fn copy(&self, item: T) -> Self {
        Self {
            item,
            keys: self.keys,
        }
    }

    fn add_key(&mut self, key: u32) {
        self.keys |= key;
    }

    fn has_key(&self, key: u32) -> bool {
        (self.keys & key) > 0
    }

    fn has_all_keys(&self, keys: u32) -> bool {
        (self.keys & keys) == keys
    }
}

struct Map {
    map: Vec<Vec<char>>,
    entrance: Pos,
    neighbors: HashMap<Pos, HashMap<Pos, usize>>,
    all_keys: u32,
}

impl Map {
    fn new(input: &str) -> Self {
        let map = parse(input);
        let mut entrance = Pos::default();
        let mut neighbors = HashMap::new();
        let mut all_keys = 0;
        for (y, row) in map.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                let pos = (x as isize, y as isize);
                if c.is_ascii_lowercase() {
                    all_keys |= tokey(c);
                }
                if c == '@' {
                    entrance = pos;
                }
                if c != '#' && c != '.' {
                    // NOTE: This is doing twice the work we actually need
                    Self::find_neighbors(
                        pos,
                        &map,
                        neighbors.entry(pos).or_insert_with(HashMap::new),
                    );
                }
            }
        }
        Self {
            map,
            entrance,
            neighbors,
            all_keys,
        }
    }

    fn shortest_path(&self) -> Option<u32> {
        let start: WithKeys<Pos> = WithKeys::new(self.entrance);
        let successors = |pwk: &WithKeys<Pos>| self.traverse(pwk);
        let success = |pwk: &WithKeys<Pos>| pwk.keys == self.all_keys;

        let (_, min) = dijkstra(&start, successors, success).expect("Failed to find shortest path");
        Some(min as u32)
    }

    fn traverse(&self, pwk: &WithKeys<Pos>) -> Vec<(WithKeys<Pos>, usize)> {
        self.neighbors
            .get(&pwk.item)
            .unwrap()
            .iter()
            .filter_map(move |(pos2, cost)| {
                let mut pwk2 = pwk.copy(pwk.item);
                pwk2.item = *pos2;
                let c = at(&self.map, pos2);
                match c {
                    'a'..='z' => {
                        pwk2.add_key(tokey(c));
                        true
                    }
                    'A'..='Z' => pwk.has_key(tokey(c)),
                    _ => panic!("Unexpected traversal point: {} @ {:?}", &c, &pos2),
                }
                .then_some((pwk2, *cost))
            })
            .collect()
    }

    fn find_neighbors(start: Pos, map: &[Vec<char>], neighbors: &mut HashMap<Pos, usize>) {
        for node in dijkstra_reach(&start, |&pos, _| {
            let limit = if pos != start && at(map, &pos).is_alphabetic() {
                0
            } else {
                4
            };
            DIRECTIONS.iter().take(limit).filter_map(move |dir| {
                let pos2 = dir.forward_from(&pos);
                (at(map, &pos2) != '#').then_some((pos2, 1))
            })
        }) {
            if node.node != start && at(map, &node.node).is_alphabetic() {
                neighbors.insert(node.node, node.total_cost);
            }
        }
    }
}

struct Map4 {
    map: Vec<Vec<char>>,
    entrances: [Pos; 4],
    neighbors: HashMap<Pos, HashMap<WithKeys<Pos>, usize>>,
    all_keys: u32,
}

impl Map4 {
    fn new(input: &str) -> Self {
        let mut map = parse(input);
        let entrances = Self::replace_vaults(&mut map);
        let mut neighbors = HashMap::new();
        let mut all_keys = 0;
        for (y, row) in map.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                let pos = (x as isize, y as isize);
                if c.is_ascii_lowercase() {
                    all_keys |= tokey(c);
                }
                if c != '#' && c != '.' {
                    // NOTE: This is doing twice the work we actually need
                    Self::find_neighbors(
                        pos,
                        &map,
                        neighbors.entry(pos).or_insert_with(HashMap::new),
                    );
                }
            }
        }
        Self {
            map,
            entrances,
            neighbors,
            all_keys,
        }
    }

    fn replace_vaults(map: &mut [Vec<char>]) -> [Pos; 4] {
        for (y, row) in map.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c == '@' {
                    map[y][x] = '#';
                    map[y - 1][x] = '#';
                    map[y][x - 1] = '#';
                    map[y][x + 1] = '#';
                    map[y + 1][x] = '#';
                    map[y - 1][x - 1] = '@';
                    map[y - 1][x + 1] = '@';
                    map[y + 1][x - 1] = '@';
                    map[y + 1][x + 1] = '@';
                    return [
                        pos_from(x - 1, y - 1),
                        pos_from(x + 1, y - 1),
                        pos_from(x - 1, y + 1),
                        pos_from(x + 1, y + 1),
                    ];
                }
            }
        }
        panic!("Can not find entrances!");
    }

    fn shortest_path(&self) -> Option<u32> {
        let start: WithKeys<[Pos; 4]> = WithKeys::new(self.entrances);
        let successors = |pwk: &WithKeys<[Pos; 4]>| self.traverse(pwk);
        let success = |pwk: &WithKeys<[Pos; 4]>| pwk.keys == self.all_keys;

        let (_, min) = dijkstra(&start, successors, success).expect("Failed to find shortest path");
        Some(min as u32)
    }

    fn traverse(&self, pwk: &WithKeys<[Pos; 4]>) -> Vec<(WithKeys<[Pos; 4]>, usize)> {
        (0..4)
            .flat_map(|i| {
                self.neighbors
                    .get(&pwk.item[i])
                    .unwrap()
                    .iter()
                    .filter(|(pwk2, _)| pwk.has_all_keys(pwk2.keys))
                    .map(move |(WithKeys { item: pos2, .. }, cost)| {
                        let key = at(&self.map, pos2);
                        assert!(key.is_ascii_lowercase());

                        let mut pwk2 = pwk.copy(pwk.item);
                        pwk2.item[i] = *pos2;
                        pwk2.add_key(tokey(key));
                        (pwk2, *cost)
                    })
            })
            .collect()
    }

    fn find_neighbors(
        start: Pos,
        map: &[Vec<char>],
        neighbors: &mut HashMap<WithKeys<Pos>, usize>,
    ) {
        let swk: WithKeys<Pos> = WithKeys::new(start);
        for node in dijkstra_reach(&swk, |&pwk, _| {
            let limit = if pwk.item != start && at(map, &pwk.item).is_ascii_lowercase() {
                0
            } else {
                4
            };
            DIRECTIONS.iter().take(limit).filter_map(move |dir| {
                let mut pwk2 = pwk.copy(dir.forward_from(&pwk.item));
                let c = at(map, &pwk2.item);
                if c.is_ascii_uppercase() {
                    pwk2.add_key(tokey(c));
                }
                (c != '#').then_some((pwk2, 1))
            })
        }) {
            if node.node.item != start && at(map, &node.node.item).is_ascii_lowercase() {
                neighbors.insert(node.node, node.total_cost);
            }
        }
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[inline]
fn at(map: &[Vec<char>], pos: &Pos) -> char {
    map[pos.1 as usize][pos.0 as usize]
}

#[inline]
fn tokey(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        1 << ((c as u32) - 97)
    } else if c.is_ascii_uppercase() {
        1 << ((c as u32) - 65)
    } else {
        panic!("Invalid key/door character: {}", c);
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
        assert_eq!(result, Some(132));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_one_c() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(81));
    }

    #[test]
    fn test_part_two_d() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_e() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two_f() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(32));
    }

    #[test]
    fn test_part_two_g() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 7,
        ));
        assert_eq!(result, Some(72));
    }
}
