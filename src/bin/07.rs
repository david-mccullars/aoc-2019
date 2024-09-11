#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<i64> {
    find_max(input, 0, 4)
}

pub fn part_two(input: &str) -> Option<i64> {
    find_max(input, 5, 9)
}

fn find_max(input: &str, s: i64, e: i64) -> Option<i64> {
    let to_consider: Vec<_> = (s..=e).permutations((e - s + 1) as usize).collect();
    to_consider
        .par_iter()
        .map(|phase_seq| run_amps(input, phase_seq))
        .max()
}

fn run_amps(codestr: &str, phases: &[i64]) -> i64 {
    Amp::chained(codestr, phases)
        .into_iter()
        .map(|mut amp| thread::spawn(move || amp.run()))
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|thread| thread.join().unwrap())
        .last()
        .unwrap()
        .unwrap()
}

struct Amp {
    intcode: Intcode,
    input: Receiver<i64>,
    output: Sender<i64>,
}

impl Amp {
    fn chained(input: &str, phases: &[i64]) -> Vec<Self> {
        let mut result = Vec::<Self>::with_capacity(phases.len());
        for (pos, phase) in phases.iter().enumerate() {
            let t = if pos == 0 {
                Self::new(input, *phase)
            } else {
                result[pos - 1].chain(*phase)
            };
            result.push(t);
        }
        result
    }

    fn new(input: &str, phase: i64) -> Self {
        let intcode = Intcode::new(input);
        let (output, input): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        output.send(phase).unwrap();
        output.send(0).unwrap();
        Self {
            intcode,
            input,
            output,
        }
    }

    fn chain(&mut self, phase: i64) -> Self {
        let intcode = self.intcode.clone();
        let (prev_output, input): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        prev_output.send(phase).unwrap();
        let output = std::mem::replace(&mut self.output, prev_output);
        Self {
            intcode,
            input,
            output,
        }
    }

    fn run(&mut self) -> Option<i64> {
        self.intcode.run_async(&self.input, &self.output);
        self.input.recv().ok()
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
        assert_eq!(result, Some(43210));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(54321));
    }

    #[test]
    fn test_part_one_c() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(65210));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(139629729));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(18216));
    }
}
