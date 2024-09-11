#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::HashMap;
use std::cmp::max;
use topological_sort::TopologicalSort;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u64> {
    let reactions = Reactions::parse(input);
    Some(reactions.ore_for_fuel(1))
}

pub fn part_two(input: &str) -> Option<u64> {
    let reactions = Reactions::parse(input);
    Some(reactions.fuel_for_ore(1_000_000_000_000))
}

#[derive(Debug)]
struct Reactions {
    elements: usize,
    reactions: Vec<Reaction>,
}

impl Reactions {
    fn parse(input: &str) -> Self {
        let element = parser!(u64 " " string(upper+));
        let parsed = parser!(lines(
            repeat_sep(element, ", ") " => " element
        ))
        .parse(input)
        .unwrap();

        let mut ts = TopologicalSort::<String>::new();
        for (prereqs, (_, element)) in &parsed {
            for (_, prereq) in prereqs {
                ts.add_dependency(prereq.clone(), element.clone());
            }
        }

        let mut elements = HashMap::new();
        for (pos, element) in ts.enumerate() {
            elements.insert(element, pos);
        }

        let mut reactions: Vec<_> = parsed
            .into_iter()
            .map(|(prereqs, result)| Reaction::new(prereqs, result, &elements))
            .collect();
        reactions.sort_by(|a, b| b.element.cmp(&a.element));

        Self {
            elements: elements.len(),
            reactions,
        }
    }

    fn ore_for_fuel(&self, fuel: u64) -> u64 {
        let mut need = vec![0; self.elements];
        need[self.elements - 1] = fuel;
        for reaction in &self.reactions {
            reaction.reverse(&mut need);
        }
        need[0]
    }

    fn fuel_for_ore(&self, ore: u64) -> u64 {
        let mut fuel = 1;
        loop {
            let actual_ore = self.ore_for_fuel(fuel + 1);
            if actual_ore > ore {
                return fuel;
            } else {
                let big = ((fuel + 1) as u128) * (ore as u128);
                fuel = max(fuel + 1, (big / (actual_ore as u128)) as u64);
            }
        }
    }
}

#[derive(Debug)]
struct Reaction {
    element: usize,
    produces: u64,
    requires: Vec<u64>,
}

impl Reaction {
    fn new(
        prereqs: Vec<(u64, String)>,
        result: (u64, String),
        elements: &HashMap<String, usize>,
    ) -> Self {
        let element = elements.get(&result.1).copied().unwrap();
        let produces = result.0;
        let mut requires = vec![0; elements.len()];
        for (amount, element) in prereqs {
            requires[*elements.get(&element).unwrap()] = amount;
        }
        Self {
            element,
            produces,
            requires,
        }
    }

    fn reverse(&self, need: &mut [u64]) {
        if need[self.element] == 0 {
            return;
        }
        let multiplier = need[self.element].div_ceil(self.produces);
        need[self.element] = 0;
        for (i, q) in self.requires.iter().enumerate() {
            need[i] += q * multiplier;
        }
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
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(165));
    }

    #[test]
    fn test_part_one_c() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(13312));
    }

    #[test]
    fn test_part_one_d() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(180697));
    }

    #[test]
    fn test_part_one_e() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(2210736));
    }

    #[test]
    fn test_part_two_c() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(82892753));
    }

    #[test]
    fn test_part_two_d() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(5586022));
    }

    #[test]
    fn test_part_two_e() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(460664));
    }
}
