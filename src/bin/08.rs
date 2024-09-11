#[allow(unused_imports)]
use advent_of_code::*;
use advent_of_code_ocr::parse_string_to_letters;
use itertools::join;
use ndarray::{s, ArrayBase, Dim, OwnedArcRepr};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    _part_one(input, 25, 6)
}

pub fn _part_one(input: &str, width: usize, height: usize) -> Option<usize> {
    let layers = parse(input, width, height);
    let min_layer = min_count(&layers, '0');
    let ones = count(&layers, min_layer, '1');
    let twos = count(&layers, min_layer, '2');
    Some(ones * twos)
}

pub fn part_two(input: &str) -> Option<String> {
    Some(parse_string_to_letters(&_part_two(input, 25, 6)))
}

pub fn _part_two(input: &str, width: usize, height: usize) -> String {
    let layers = parse(input, width, height);
    join(
        (0..height).map(|row| join((0..width).map(|col| overlay(&layers, row, col)), "")),
        "\n",
    )
}

type Layers = ArrayBase<OwnedArcRepr<char>, Dim<[usize; 3]>>;

fn parse(input: &str, width: usize, height: usize) -> Layers {
    let chars: Vec<char> = input.trim().chars().collect();
    ndarray::rcarr1(&chars)
        .into_shape_with_order((chars.len() / height / width, height, width))
        .expect("Input data has incorrect number of elements")
}

fn min_count(layers: &Layers, elem: char) -> usize {
    (0..layers.shape()[0])
        .map(|layer| count(layers, layer, elem))
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .expect("Min value not found")
        .0
}

fn count(layers: &Layers, layer: usize, elem: char) -> usize {
    layers
        .slice(s![layer, .., ..])
        .iter()
        .filter(|c| **c == elem)
        .count()
}

fn overlay(layers: &Layers, row: usize, col: usize) -> char {
    layers
        .slice(s![.., row, col])
        .iter()
        .flat_map(|c| match c {
            '0' => Some('.'),
            '1' => Some('#'),
            '2' => None,
            c => panic!("Unsupported character {:} found", c),
        })
        .next()
        .expect("Fully transparent")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = _part_one(
            &advent_of_code::template::read_file_part("examples", DAY, 1),
            3,
            2,
        );
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 2);
        let result = _part_two(&input, 2, 2);
        assert_eq!(result, String::from(".#\n#."));
    }
}
