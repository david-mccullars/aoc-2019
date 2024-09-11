#[allow(unused_imports)]
use advent_of_code::*;
use fancy_regex::Regex;
use hashbrown::HashSet;
use itertools::Itertools;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<isize> {
    let map = Map::from_intcode(input);
    Some(map.alignment_score())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut map = Map::from_intcode(input);
    let path = map.traverse();
    let program = compile_program(path, 20);

    let mut intcode = Intcode::new(input);
    intcode.set_code(0, 2);
    intcode.run_ascii(&program)
}

#[derive(Default)]
struct Map {
    scaffold: HashSet<Pos>,
    dims: Pos,
    pos: Pos,
    dir: Direction,
}

impl Map {
    fn from_intcode(input: &str) -> Self {
        let mut output = String::new();
        Intcode::new(input).run(|| None, |value| output.push((value as u8) as char));
        Self::from_str(&output)
    }

    fn from_str(input: &str) -> Self {
        let mut map = Map::default();
        for (row, line) in input.trim().lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                map.update(pos_from(col, row), c);
            }
        }
        map
    }

    fn update(&mut self, pos: Pos, value: char) {
        if value == '.' {
            return;
        }

        self.scaffold.insert(pos);
        self.dims.0 = std::cmp::max(self.dims.0, pos.0 + 1);
        self.dims.1 = std::cmp::max(self.dims.1, pos.1 + 1);

        if value != '#' {
            self.pos = pos;
            self.dir = Direction::from_char(value);
        }
    }

    fn alignment_score(&self) -> isize {
        (0..self.dims.0)
            .flat_map(|x| {
                (0..self.dims.1).filter_map(move |y| self.is_intersection(&(x, y)).then_some(x * y))
            })
            .sum()
    }

    fn is_intersection(&self, pos: &Pos) -> bool {
        self.scaffold.contains(pos)
            && DIRECTIONS
                .iter()
                .filter(|dir| self.scaffold.contains(&dir.forward_from(pos)))
                .nth(3)
                .is_some()
    }

    fn traverse(&mut self) -> String {
        let mut path = String::from("");
        let mut dist = 0;
        loop {
            let forward_pos = self.dir.forward_from(&self.pos);
            let left_dir = self.dir.turn_left();
            let right_dir = self.dir.turn_right();
            if self.scaffold.contains(&forward_pos) {
                self.pos = forward_pos;
                dist += 1;
            } else if self.scaffold.contains(&left_dir.forward_from(&self.pos)) {
                if dist > 0 {
                    path.push_str(&format!("{},", dist));
                }
                path.push_str("L,");
                self.dir = left_dir;
                dist = 0;
            } else if self.scaffold.contains(&right_dir.forward_from(&self.pos)) {
                if dist > 0 {
                    path.push_str(&format!("{},", dist));
                }
                path.push_str("R,");
                self.dir = right_dir;
                dist = 0;
            } else {
                break;
            }
        }
        if dist > 0 {
            path.push_str(&format!("{},", dist));
        }
        path
    }
}

fn compile_program(code: String, limit: usize) -> String {
    let abc = format!("(.{{1,{}}},)", limit - 1);
    let re = Regex::new(&format!(
        r"^{}\1*{}(?:\1|\2)*{}(?:\1|\2|\3)*$",
        &abc, &abc, &abc
    ))
    .unwrap();
    let captures = re
        .captures(&code)
        .expect("Error running regex")
        .expect("No match found");

    // Sort the groups so that the longest gets replaced first (so we don't get unlucky)
    let mut groups: Vec<_> = (1..=3)
        .map(|g| captures.get(g).unwrap().as_str().trim_end_matches(","))
        .collect();
    groups.sort_by_key(|b| std::cmp::Reverse(b.len()));
    let (a, b, c) = groups.iter().tuples().next().unwrap();

    let code = code
        .replacen(a, "A", limit)
        .replacen(b, "B", limit)
        .replacen(c, "C", limit);
    let main = code.trim_end_matches(",");

    format!("{}\n{}\n{}\n{}\nn\n", main, a, b, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample() {
        let map = Map::from_str(
            r#"
..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..
        "#,
        );
        assert_eq!(map.alignment_score(), 76);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2660));
    }

    #[test]
    fn test_part_two_sample() {
        let mut map = Map::from_str(
            r#"
#######...#####
#.....#...#...#
#.....#...#...#
......#...#...#
......#...###.#
......#.....#.#
^########...#.#
......#.#...#.#
......#########
........#...#..
....#########..
....#...#......
....#...#......
....#...#......
....#####......
        "#,
        );
        let path = map.traverse();
        assert_eq!(
            &path,
            "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2,"
        );
        let program = compile_program(path, 12);
        assert_eq!(
            &program,
            r#"B,A,C,A,B,C
R,4,R,4,R,8
R,8,R,8
L,6,L,2
n
"#
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(790595));
    }
}
