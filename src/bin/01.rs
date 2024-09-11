#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(sum(input, fuel_cost))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(sum(input, recursive_fuel_cost))
}

fn sum(input: &str, cost: impl Fn(u32) -> u32) -> u32 {
    parser!(lines(u32))
        .parse(input)
        .unwrap()
        .into_iter()
        .map(cost)
        .sum()
}

fn fuel_cost(input: u32) -> u32 {
    (input / 3).saturating_sub(2)
}

fn recursive_fuel_cost(input: u32) -> u32 {
    let mut total = 0;
    let mut cost = input;
    while cost > 0 {
        cost = fuel_cost(cost);
        total += cost;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34241));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51316));
    }
}
