#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<isize> {
    let mut drone = Drone::new(input);
    Some(
        (0..50)
            .filter_map(|y| drone.scan_row(y))
            .map(|row| row.1 - row.0)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut drone = Drone::new(input);
    for y in drone.estimate_row(100).unwrap().. {
        let (_, x2) = drone.scan_row(y).unwrap();
        let (x1, _) = drone.scan_row(y + 99).unwrap();
        let size = x2 - x1;
        assert!(size <= 100);
        if size == 100 {
            return Some(10_000 * x1 + y);
        }
    }
    None
}

struct Drone(Intcode);

impl Drone {
    fn new(input: &str) -> Self {
        Self(Intcode::new(input))
    }

    fn scan_row(&mut self, y: isize) -> Option<(isize, isize)> {
        let mut x_0 = -1;
        let mut x_1 = -1;
        for x in 0..=(y + 10) {
            if self.0.clone().run_simple(&[x, y]) == Some(1) {
                if x_0 < 0 {
                    x_0 = x;
                }
                x_1 = x;
            } else if x_0 >= 0 {
                return Some((x_0, x_1 + 1));
            }
        }
        None
    }

    // Consider a much larger big_y (e.g. y * 10), and find the edges
    // of the row (x1 and x2). Then using those to calculate the slopes
    // of the tractor beam edges, we can use the following formula to
    // find the top of the box:
    //
    //   y_top_of_box = (big_y + x1) / (x2 - x1)
    //
    // However, since we're dealing with integer division, we need to
    // error on the side of caution and expand the tractor beam a bit
    // so that we under-estimate the top of the box.
    fn estimate_row(&mut self, y: isize) -> Option<isize> {
        self.scan_row(y * 10)
            .map(|(x1, x2)| y * (y * 10 + x1 - 1) / (x2 - x1 + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(166));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3790981));
    }
}
