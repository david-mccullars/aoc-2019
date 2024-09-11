#[allow(unused_imports)]
use advent_of_code::*;
use num_modular::{ModularInteger, VanillaInt};

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    let shuffler = Shuffler::new(input, 10007);
    Some(shuffler.new_position_for(2019))
}

pub fn part_two(input: &str) -> Option<u64> {
    let shuffler = InverseShuffler::new(input, 119315717514047);
    Some(shuffler.original_position_after(2020, 101741582076661))
}

//
// The shuffler determines a linear time function
//
//    f(p) = increment * p + offset
//
// which can determine what position a card will be in after
// applying all of the embedded operations.
//
// NOTE: We use the num_modular crate in order to ensure that
// ALL mathematical operations are done MOD modulus - but done
// so in an efficient manner. We can then safely do all basic
// math operations without making the code gross (even finding
// the multiplicative inverse).
//
struct Shuffler {
    offset: VanillaInt<u64>,
    increment: VanillaInt<u64>,
}

impl Shuffler {
    // f(p) = p
    fn identity(modulus: u64) -> Self {
        Self {
            offset: VanillaInt::new(0, &modulus),
            increment: VanillaInt::new(1, &modulus),
        }
    }

    fn new(input: &str, modulus: u64) -> Self {
        let mut s = Self::identity(modulus);
        for op in Op::parse(input).iter() {
            match *op {
                Op::DealIntoNewStack => {
                    s.increment = -s.increment;
                    s.offset = -s.offset - 1
                }
                Op::CutPositive(n) => {
                    s.offset = s.offset - n;
                }
                Op::CutNegative(n) => {
                    s.offset = s.offset + n;
                }
                Op::DealWithIncrement(n) => {
                    s.increment = s.increment * n;
                    s.offset = s.offset * n;
                }
            }
        }
        s
    }

    fn new_position_for(&self, p: u64) -> u64 {
        (self.increment * p + self.offset).residue()
    }
}

//
// If we want to go the other direction and determine what card
// (i.e. the original position) ends up in a given position, we
// need only embed the inverse operations (in reverse order) to
// produce a different function:
//
//    f_inverse(p) = increment * p + offset
//
// We will use this to efficiently calculate not just what ends
// up in a given position after one shuffle but after N shuffles.
//
struct InverseShuffler(Shuffler);

impl InverseShuffler {
    fn new(input: &str, modulus: u64) -> Self {
        let mut s = Shuffler::identity(modulus);
        for op in Op::parse(input).iter().rev() {
            match *op {
                Op::DealIntoNewStack => {
                    s.increment = -s.increment;
                    s.offset = -s.offset - 1
                }
                Op::CutPositive(n) => {
                    s.offset = s.offset + n;
                }
                Op::CutNegative(n) => {
                    s.offset = s.offset - n;
                }
                Op::DealWithIncrement(n) => {
                    let n = s.increment.convert(n);
                    s.increment = s.increment / n;
                    s.offset = s.offset / n;
                }
            }
        }
        InverseShuffler(s)
    }

    //
    // Consider the following ...
    //
    //   f(p)         = i * p + o
    //   f(f(p))      = i^2 * p + (i + 1) * o
    //   f(f(f(p))    = i^3 * p + (i^2 + i + 1) * o
    //   f(f(f(f(p))) = i^4 * p + (i^3 + i^2 + i + 1) * o
    //   f_n(p)       = i^n * p + (i^n-1 + i^n-2 + ... + i + 1) * o
    //
    // Recall that the finite geometric series has the sum
    //
    //   Σi^n-1 = (i^n - 1) / (i - 1)
    //
    // Therefore we end up with:
    //
    //   f_n(p) = i^n * p + (i^n - 1) / (i - 1) * o
    //
    fn original_position_after(&self, p: u64, n: u64) -> u64 {
        let i = self.0.increment;
        let o = self.0.offset;
        let i_n = i.pow(&n);
        (i_n * p + (i_n - 1) / (i - 1) * o).residue()
    }
}

#[derive(Debug)]
enum Op {
    DealIntoNewStack,
    CutPositive(u64),
    CutNegative(u64),
    DealWithIncrement(u64),
}

impl Op {
    fn parse(input: &str) -> Vec<Op> {
        parser!(lines({
            "deal into new stack"           => Op::DealIntoNewStack,
            "cut " v:u64                    => Op::CutPositive(v),
            "cut -" v:u64                   => Op::CutNegative(v),
            "deal with increment " v:u64    => Op::DealWithIncrement(v),
        }))
        .parse(input)
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let shuffler = Shuffler::new(&advent_of_code::template::read_file("examples", DAY), 10);
        for (after, before) in [9, 2, 5, 8, 1, 4, 7, 0, 3, 6].into_iter().enumerate() {
            assert_eq!(shuffler.new_position_for(before), after as u64);
        }
    }

    // Part two can't be tested with mod 10 because we aren't guaranteed of an inverse
}
