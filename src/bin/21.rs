#[allow(unused_imports)]
use advent_of_code::*;
use unindent::unindent;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<i64> {
    // ((!b || !c) && d) || !a
    run(
        input,
        "
            NOT B J
            NOT C T
            OR T J
            AND D J
            NOT A T
            OR T J
            WALK
        ",
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    // ((!b || !c) && d && h) || !a
    run(
        input,
        "
            NOT B J
            NOT C T
            OR T J
            AND D J
            AND H J
            NOT A T
            OR T J
            RUN
        ",
    )
}

fn run(input: &str, program: &str) -> Option<i64> {
    Intcode::new(input).run_ascii(&unindent(program))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19350375));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1143990055));
    }
}
