#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    count_valid_passwords(input, is_ascending_and_has_duplicate)
}

pub fn part_two(input: &str) -> Option<usize> {
    count_valid_passwords(input, is_ascending_and_has_single_pair)
}

fn count_valid_passwords<P>(input: &str, is_valid: P) -> Option<usize>
where
    P: Fn(&u32) -> bool,
{
    let range = parser!(line(a:u32 "-" b:u32 => a..=b))
        .parse(input)
        .unwrap();
    Some(range.filter(is_valid).count())
}

fn is_ascending_and_has_duplicate(password: &u32) -> bool {
    let mut digits = digits(*password, 10); // In reverse order
    let mut duplicate = false;
    let mut prev = digits.next().unwrap_or(0);
    for d in digits {
        if d > prev {
            return false;
        }
        if d == prev {
            duplicate = true;
        }
        prev = d;
    }
    duplicate
}

fn is_ascending_and_has_single_pair(password: &u32) -> bool {
    let mut digits = digits(*password, 10); // In reverse order
    let mut duplicate = false;
    let mut prev = digits.next().unwrap_or(0);
    let mut prev_count = 1;
    for d in digits {
        if d > prev {
            return false;
        }
        if d == prev {
            prev_count += 1;
        } else {
            if prev_count == 2 {
                duplicate = true;
            }
            prev_count = 1;
        }
        prev = d;
    }
    duplicate || prev_count == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));

        assert_eq!(part_one("1128-1247"), Some(40));
        assert_eq!(part_one("1128-3247"), Some(179));
        assert_eq!(part_one("128-247"), Some(17));

        assert_eq!(part_one("593737-619357"), Some(1));
        assert_eq!(part_one("102764-998505"), Some(2918));
        assert_eq!(part_one("246234-483968"), Some(860));
        assert_eq!(part_one("156195-638239"), Some(1694));
        assert_eq!(part_one("308973-995910"), Some(916));
        assert_eq!(part_one("726178-757359"), Some(0));
        assert_eq!(part_one("359032-880742"), Some(511));

        assert_eq!(part_one("211-209"), Some(0));
        assert_eq!(part_one("311-249"), Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));

        assert_eq!(part_two("1128-1247"), Some(39));
        assert_eq!(part_two("1128-3247"), Some(156));
        assert_eq!(part_two("128-247"), Some(16));

        assert_eq!(part_two("593737-619357"), Some(0));
        assert_eq!(part_two("102764-998505"), Some(2046));
        assert_eq!(part_two("246234-483968"), Some(563));
        assert_eq!(part_two("156195-638239"), Some(1148));
        assert_eq!(part_two("308973-995910"), Some(602));
        assert_eq!(part_two("726178-757359"), Some(0));
        assert_eq!(part_two("359032-880742"), Some(316));

        assert_eq!(part_two("211-209"), Some(0));
        assert_eq!(part_two("311-249"), Some(0));
    }
}
