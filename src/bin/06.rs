#[allow(unused_imports)]
use advent_of_code::*;
use std::collections::HashMap;

advent_of_code::solution!(6);

lazy_static::lazy_static! {
    static ref COM: String = String::from("COM");
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(Orbits::parse(input).total_orbits())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Orbits::parse(input).transfer_distance("YOU", "SAN"))
}

struct Orbits(HashMap<String, String>);

impl Orbits {
    fn parse(input: &str) -> Self {
        let orbits = parser!(hash_map(lines(
            a:string(any_char+) ")" b:string(any_char+) => (b, a)
        )))
        .parse(input)
        .unwrap();
        Self(orbits)
    }

    fn lineage<'a>(&'a self, object: &'a str) -> OrbitsIterator<'a> {
        OrbitsIterator {
            orbits: &self.0,
            curr: object,
        }
    }

    fn total_orbits(&self) -> usize {
        self.0
            .keys()
            .map(|object| self.lineage(object).count())
            .sum()
    }

    fn transfer_distance(&self, from: &str, to: &str) -> usize {
        let mut from = self.lineage(from).collect::<Vec<_>>().into_iter().rev();
        let mut to = self.lineage(to).collect::<Vec<_>>().into_iter().rev();
        while from.next() == to.next() {
            // Share same lineage
        }
        from.count() + to.count()
    }
}

struct OrbitsIterator<'a> {
    orbits: &'a HashMap<String, String>,
    curr: &'a str,
}

impl<'a> Iterator for OrbitsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        if current == *COM {
            None
        } else {
            self.curr = self.orbits.get(current).unwrap_or(&COM);
            Some(current)
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
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }
}
