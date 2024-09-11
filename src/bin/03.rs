#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<isize> {
    let (path1, path2) = parse_paths(input);
    closest_intersection(&path1, &path2, |(pos, _)| manhattan_distance(&(0, 0), &pos))
}

pub fn part_two(input: &str) -> Option<isize> {
    let (path1, path2) = parse_paths(input);
    closest_intersection(&path1, &path2, |(_, d)| d)
}

fn closest_intersection(
    path1: &[PathSegment],
    path2: &[PathSegment],
    dist: fn(PathPos) -> isize,
) -> Option<isize> {
    path1
        .iter()
        .flat_map(|a| {
            path2
                .iter()
                .flat_map(|b| a.intersections(b, |(x, y)| *x != 0 || *y != 0))
        })
        .map(dist)
        .min()
}

fn parse_paths(input: &str) -> (Vec<PathSegment>, Vec<PathSegment>) {
    parser!(lines(
        repeat_sep(upper isize, ",")
    ))
    .parse(input)
    .unwrap()
    .into_iter()
    .map(parse_path)
    .collect_tuple()
    .unwrap()
}

fn parse_path(path: Vec<(char, isize)>) -> Vec<PathSegment> {
    let mut p1 = pos_from(0, 0);
    let mut distance = 0;

    let mut lines = Vec::with_capacity(path.len());
    for (dir, d) in path {
        let p2 = Direction::from_char(dir).forward_n_from(&p1, d);
        lines.push(PathSegment::new(p1, p2, distance));
        p1 = p2;
        distance += d;
    }
    lines
}

type PathPos = (Pos, isize);

struct PathSegment {
    line: LineSegment,
    total_distance: isize,
}

impl PathSegment {
    fn new(p1: Pos, p2: Pos, total_distance: isize) -> Self {
        Self {
            line: LineSegment { p1, p2 },
            total_distance,
        }
    }

    fn intersections(&self, other: &PathSegment, filter: fn(&Pos) -> bool) -> Vec<PathPos> {
        self.line
            .intersections(&other.line)
            .filter(filter)
            .map(|pos| (pos, self.distance_to(&pos) + other.distance_to(&pos)))
            .collect()
    }

    fn distance_to(&self, pos: &Pos) -> isize {
        self.total_distance + manhattan_distance(&self.line.p1, pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_one_b() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let result = part_one(input);
        assert_eq!(result, Some(159));
    }

    #[test]
    fn test_part_one_c() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let result = part_one(input);
        assert_eq!(result, Some(135));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_part_two_b() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let result = part_two(input);
        assert_eq!(result, Some(610));
    }

    #[test]
    fn test_part_two_c() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let result = part_two(input);
        assert_eq!(result, Some(410));
    }
}
