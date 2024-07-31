pub mod template;

use nalgebra::DMatrix;
use num::Num;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::fmt::Display;

mod digits;
mod direction;
mod graph;
mod line_segment;
mod maps;

#[allow(unused_imports)]
pub use aoc_parse::{parser, prelude::*, ParseIter};
pub use digits::*;
pub use direction::*;
pub use graph::*;
pub use line_segment::*;
pub use maps::*;

#[allow(dead_code)]
pub fn name_to_digit(s: &str) -> Option<u32> {
    match s {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

// xs:      🟩🟩🟩        🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧              🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥
// ys[0]:                                                 ⬜⬜⬜⬜⬜
//          🟩🟩🟩        🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧              🟥🟥🟦🟦🟦🟦🟦🟦🟦🟦🟦🟦🟦🟦
// ys[1]:                                                                           ⬜⬜⬜⬜⬜⬜⬜⬜⬜
//          🟩🟩🟩        🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧              🟥🟥🟦🟦🟦🟦🟦🟦🟦🟦🟪🟪🟪🟪
// ys[2]:                                                                                            ⬜⬜
//          🟩🟩🟩        🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧🟧              🟥🟥🟦🟦🟦🟦🟦🟦🟦🟦🟪🟪🟪🟪
// ys[3]:                 ⬜
//          🟩🟩🟩        🟧🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫🟫              🟥🟥🟦🟦🟦🟦🟦🟦🟦🟦🟪🟪🟪🟪
// ys[4]:                             ⬜⬜⬜
//          🟩🟩🟩        🟧🟫🟫🟫🟫🟫🟪🟪🟪🟨🟨🟨              🟥🟥🟦🟦🟦🟦🟦🟦🟦🟦🟪🟪🟪🟪
// ys[5]: ⬜⬜⬜⬜⬜⬜
//          🟩🟩🟩        🟦🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪              🟥🟥🟦🟦🟦🟦🟦🟦🟦🟦🟪🟪🟪🟪
#[allow(dead_code)]
pub fn range_split<T: Num + Ord + Copy + Debug + Display>(
    xs: Vec<(T, T)>,
    ys: Vec<(T, T)>,
) -> Vec<(T, T)> {
    let mut changed = xs;
    for y in ys {
        changed = changed.iter().flat_map(|x| range_split1(x, &y)).collect();
    }
    changed
}

#[allow(dead_code)]
#[inline]
pub fn range_split1<T: Num + Ord + Copy + Display>(x: &(T, T), y: &(T, T)) -> Vec<(T, T)> {
    //     |  x  |                      |  x  |
    //              |  y  |         |     y       |
    //     |     |                      |     |
    if (x.1 <= y.0 || y.1 <= x.0) || (y.0 <= x.0 && x.1 <= y.1) {
        vec![*x]

    //     |      x      |
    //         |  y  |
    //     |   |     |   |
    } else if x.0 < y.0 && y.1 < x.1 {
        vec![(x.0, y.0), (y.0, y.1), (y.1, x.1)]

    //     |  x  |
    //  |  y  |
    //     |  |  |
    } else if y.0 <= x.0 {
        vec![(x.0, y.1), (y.1, x.1)]

    //     |  x  |
    //        |  y  |
    //     |  |  |
    } else if x.0 <= y.0 {
        vec![(x.0, y.0), (y.0, x.1)]
    } else {
        eprintln!(
            "Unexpected ranges: ({}, {}) and ({}, {})",
            x.0, x.1, y.0, y.1
        );
        std::process::exit(1);
    }
}

#[allow(dead_code)]
#[inline]
pub fn range_overlap1<T: Num + Ord + Copy + Display>(x: &(T, T), y: &(T, T)) -> bool {
    x.0 < y.1 && y.0 < x.1
}

#[allow(dead_code)]
pub fn transpose<T: Copy>(data: &[Vec<T>]) -> Vec<Vec<T>> {
    let rows = data.len();
    let cols = data[0].len();
    (0..cols)
        .map(|col| (0..rows).map(|row| data[row][col]).collect())
        .collect()
}

#[allow(dead_code)]
pub fn transpose_text(s: &str) -> String {
    let lines: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
    let transposed = transpose(&lines);
    transposed
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

#[allow(dead_code)]
pub fn manhattan_distance<T: Num + Copy + PartialOrd>(p1: &(T, T), p2: &(T, T)) -> T {
    let d1 = if p1.0 > p2.0 {
        p1.0 - p2.0
    } else {
        p2.0 - p1.0
    };
    let d2 = if p1.1 > p2.1 {
        p1.1 - p2.1
    } else {
        p2.1 - p1.1
    };
    d1 + d2
}

#[allow(dead_code)]
pub fn rotate<T: Copy>(data: &[Vec<T>]) -> Vec<Vec<T>> {
    let n = data.len();
    let mut new_data: Vec<Vec<T>> = vec![vec![data[0][0]; n]; n];

    for (i, row) in data.iter().enumerate().take(n) {
        for (j, d) in row.iter().enumerate().take(n) {
            new_data[j][n - i - 1] = *d;
        }
    }

    new_data
}

#[allow(dead_code)]
pub fn rotate_mut<T: Copy>(data: &mut [Vec<T>]) {
    let n = data.len();
    for i in 0..n / 2 {
        for j in i..n - i - 1 {
            let temp = data[i][j];
            data[i][j] = data[n - j - 1][i];
            data[n - j - 1][i] = data[n - i - 1][n - j - 1];
            data[n - i - 1][n - j - 1] = data[j][n - i - 1];
            data[j][n - i - 1] = temp;
        }
    }
}

#[allow(dead_code)]
pub fn polynomial_interpolate<F>(x: Vec<f64>, f: F) -> Box<dyn Fn(f64) -> f64>
where
    F: Fn(f64) -> f64,
{
    let n = x.len();

    let y: Vec<f64> = x.iter().map(|x| f(*x)).collect();
    let y = DMatrix::from_column_slice(n, 1, &y);

    let vandermonde: Vec<f64> = x
        .iter()
        .flat_map(|x| (0..n).map(|i| x.powf(i as f64)).rev())
        .collect();
    let vandermonde = DMatrix::from_row_slice(n, n, &vandermonde);

    let a = vandermonde.lu().solve(&y).unwrap();
    let a = a.data.as_vec().clone();

    Box::new(move |x: f64| {
        let powers = std::iter::successors(Some(1.0), move |&x_i| Some(x_i * x));
        a.iter().rev().zip(powers).map(|(a_i, x_i)| a_i * x_i).sum()
    })
}
