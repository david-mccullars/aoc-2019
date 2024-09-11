#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;
use num::integer::binomial;
use num::Num;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<String> {
    let mut input: Vec<isize> = parse(input);
    apply_phases(&mut input, 100);
    Some(input.iter().take(8).join(""))
}

fn apply_phases(input: &mut Vec<isize>, count: usize) {
    let mut next = input.clone();
    for _ in 0..count {
        for (pos, v2) in next.iter_mut().enumerate() {
            *v2 = 0;
            for (i, v) in input.iter().enumerate().skip(pos) {
                match ((i - pos) / (pos + 1)) % 4 {
                    0 => {
                        *v2 += *v;
                    }
                    2 => {
                        *v2 -= *v;
                    }
                    _ => {}
                };
            }
            *v2 = v2.abs() % 10;
        }
        std::mem::swap(input, &mut next);
    }
}

/*
    Solution adapted from https://github.com/akalin/advent-of-code-2019-python3/blob/master/day16.py

    The key to part two is that our given offset is more than halfway into the "extended" input.

    Explanation:

    We can think of each phase as multiplying the input vector by the matrix (except with the mod 10 rule).

        |	1	0	-1	0	1	 0	-1	|
        |	0	1	 1	0	0	-1	-1	|
        |	0	0	 1	1	1	 0	 0	|
        |	0	0	 0	1	1	 1	 1	|   *   |   i0  i1  i2  i3  i4  i5  i6  |
        |	0	0	 0	0	1	 1	 1	|
        |	0	0	 0	0	0	 1	 1	|
        |	0	0	 0	0	0	 0	 1	|

    Looking at the entire matrix things are messy, but we can observe this is a triangular matrix. This means
    that the last half of the output only relies on the last half of the input. And if we're only concerned
    with the last half of the matrix, we can only consider:

        |	1	1	 1	 1	|
        |	0	1	 1	 1	|   *   |   i3  i4  i5  i6  |
        |	0	0	 1	 1	|
        |	0	0	 0	 1	|

    This is much better because everything is positive, and we can use binomial coefficients to efficiently
    calculate the results. See below ...
*/
pub fn part_two(input: &str) -> Option<String> {
    let input: Vec<usize> = parse(input);
    let offset = input.iter().take(7).fold(0, |n, a| n * 10 + a);
    let end = input.len() * 10_000;
    assert!(offset * 2 >= input.len()); // CRITICAL ASSUMPTION: We're more than half-way

    let extended_from_offset: Vec<_> = input
        .iter()
        .cycle()
        .skip(offset % input.len())
        .take(end - offset)
        .cloned()
        .collect();

    let coeffs = t_nk_mod_10(extended_from_offset.len(), 100); // See below for explanation

    let result = (0..8)
        .map(|i| {
            extended_from_offset[i..]
                .iter()
                .zip(coeffs.iter())
                .map(|(x, y)| (x * y) % 10)
                .sum::<usize>()
                % 10
        })
        .join("");
    Some(result)
}

/*
     We want to compute A^count where

         [ 1 1 ... 1 1 ]
         [ 0 1 ... 1 1 ]
     A = [ ...     ... ]
         [ 0 0 ... 1 1 ]
         [ 0 0 ... 0 1 ]

     i.e. where A has 1s on and above the diagonal.

     If you compute A^2, you notice that it looks like

           [ 1 2 ... n-1   n ]
           [ 0 1 ... n-2 n-1 ]
     A^2 = [ ... ... ... ... ]
           [ 0 0 ...   1   2 ]
           [ 0 0 ...   0   1 ]

     i.e., each row is the row above shifted 1 to the right, and the entries
     in the first row are the sum of the columns of A. Similarly, the rows of
     A^3 are shifts of the first row, and the first row are the sums of the
     columns of A^2, namely the sums of the first n integers. Therefore, the
     first row of A^3 are the triangular numbers:

           [ 1 3 6 10  ... ]
           [ 0 1 3  6  ... ]
     A^3 = [       ...     ]
           [ 0 0 0 ... 1 3 ]
           [ 0 0 0 ... 0 1 ]

     The formula for the nth triangular number is:

       T_{n,3} = B(n+1, 2) = n*(n+1)/2,

     where B(n, k) = n!/(k!*(n-k)!) are the binomial coefficients.

     Similarly, the first row of A^4 are the tetrahedral numbers, which are
     the sum of the first n triangular numbers. The formula for the nth
     tetrahedral number is:

       T_{n,4} = B(n+2, 3) = n*(n+1)*(n+2)/3.

     We can then guess that the first row of A^k follows the formula:

       T_{n,k} = B(n+k-2, k-1).

     This is in fact true, and we can show this by showing:

      B(n+k-1, k) = ∑_{m=1}^n B(m+k-2, k-1),

     which follows from https://en.wikipedia.org/wiki/Hockey-stick_identity .
*/

fn t_nk_mod_10(max_n: usize, k: usize) -> Vec<usize> {
    (0..=max_n)
        .map(|n| binom_mod_10(n + k - 1, k - 1))
        .collect()
}

/*
    Finally, use https://en.wikipedia.org/wiki/Chinese_remainder_theorem to
    compute binom(n, k) % 10 in terms of binom(n, k) % 2 and binom(n, k) % 5.

    Since (-2)*2 + 1*5 = 1, the CRT says that if n = a_1 mod 2 and n = a_2 mod 5,
    then n = (5*a_1 - 4*a_2) mod 10.

    This is actually slower than T_k_fast above for small inputs,
    though. It's only used for
    https://www.reddit.com/r/adventofcode/comments/ebb8w6/2019_day_16_part_three_a_fanfiction_by_askalski/ .
*/
fn binom_mod_10(n: usize, k: usize) -> usize {
    (20 + 5 * binom_mod_2(n, k) - 4 * binom_mod_5(n, k)) % 10
}

/*
    Compute binom(n, k) % 2 with bitwise operators.
*/
fn binom_mod_2(n: usize, k: usize) -> usize {
    if (!n & k) == 0 {
        1
    } else {
        0
    }
}

/*
    Use https://en.wikipedia.org/wiki/Lucas%27s_theorem
    to compute binom(n, k) % 5 (since 5 is prime).
*/
fn binom_mod_5(mut n: usize, mut k: usize) -> usize {
    let mut prod: usize = 1;
    while n > 0 || k > 0 {
        let n_i = n % 5;
        n /= 5;
        let k_i = k % 5;
        k /= 5;
        prod = (prod * binomial(n_i, k_i)) % 5;
    }
    prod
}

fn parse<T: Num>(input: &str) -> Vec<T> {
    input
        .trim()
        .char_indices()
        .map(|(i, _)| T::from_str_radix(&input[i..(i + 1)], 10).ok().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_phases() {
        let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        apply_phases(&mut input, 4);
        assert_eq!(input, vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn test_part_one_a() {
        let result = part_one("80871224585914546619083218645595");
        assert_eq!(result, Some(String::from("24176176")));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one("19617804207202209144916044189917");
        assert_eq!(result, Some(String::from("73745418")));
    }

    #[test]
    fn test_part_one_c() {
        let result = part_one("69317163492948606335995924319873");
        assert_eq!(result, Some(String::from("52432133")));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two("03036732577212944063491565474664");
        assert_eq!(result, Some(String::from("84462026")));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two("02935109699940807407585447034323");
        assert_eq!(result, Some(String::from("78725270")));
    }

    #[test]
    fn test_part_two_c() {
        let result = part_two("03081770884921959731165446850517");
        assert_eq!(result, Some(String::from("53553731")));
    }
}
