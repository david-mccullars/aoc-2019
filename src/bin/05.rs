#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i64> {
    Intcode::new(input).run_simple(&[1])
}

pub fn part_two(input: &str) -> Option<i64> {
    Intcode::new(input).run_simple(&[5])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(999));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(999));
    }

    #[test]
    fn test_extra_a() {
        let mut intcode = Intcode::new(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(intcode.run_simple(&[8]), Some(1000));
    }

    #[test]
    fn test_extra_b() {
        let mut intcode = Intcode::new(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(intcode.run_simple(&[91]), Some(1001));
    }
}
