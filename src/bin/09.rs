#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    Intcode::new(input).run_simple(&[1])
}

pub fn part_two(input: &str) -> Option<i64> {
    Intcode::new(input).run_simple(&[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1219070632396864));
    }

    #[test]
    fn test_part_one_c() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1125899906842624));
    }
}
