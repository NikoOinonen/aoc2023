use std::{
    fmt::Display,
    ops::{Div, Neg, Mul, Add},
};

use aoc2023::filter_input_lines;
use regex::Regex;

use super::Problem;

#[derive(Debug)]
struct Hailstone {
    x: i128,
    y: i128,
    z: i128,
    vx: i128,
    vy: i128,
    vz: i128,
}

#[derive(Debug, Clone, Copy)]
struct Fraction {
    int: i128,
    num: i128,
    den: i128,
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}/{}", self.int, self.num, self.den)
    }
}

impl Neg for Fraction {
    type Output = Fraction;
    fn neg(self) -> Self::Output {
        Fraction {
            int: -self.int,
            num: -self.num,
            den: self.den,
        }.reduce()
    }
}

impl Div for Fraction {
    type Output = Fraction;
    fn div(self, rhs: Self) -> Self::Output {
        let frac1 = Fraction {
            int: 0,
            num: self.int * rhs.den,
            den: rhs.int * rhs.den + rhs.num,
        }.reduce();
        let gcd = greatest_common_divisor(self.den, rhs.den);
        let rhs_den = rhs.den / gcd;
        let self_den = self.den / gcd;
        let frac2 = Fraction {
            int: 0,
            num: self.num * rhs_den,
            den: rhs.int * self.den * rhs_den + self_den * rhs.num,
        }.reduce();
        frac1 + frac2
    }
}

impl Mul for Fraction {
    type Output = Fraction;
    fn mul(self, rhs: Self) -> Self::Output {
        let frac1 = Fraction {
            int: self.int * rhs.int,
            num: self.num * rhs.num,
            den: self.den * rhs.den,
        }.reduce();
        let frac2 = Fraction {
            int: 0,
            num: self.int * rhs.num,
            den: rhs.den,
        }.reduce();
        let frac3 = Fraction {
            int: 0,
            num: rhs.int * self.num,
            den: self.den,
        }.reduce();
        frac1 + frac2 + frac3
    }
}

impl Add for Fraction {
    type Output = Fraction;
    fn add(self, rhs: Self) -> Self::Output {
        let gcd = greatest_common_divisor(self.den, rhs.den);
        let rhs_den = rhs.den / gcd;
        let self_den = self.den / gcd;
        Fraction {
            int: self.int + rhs.int,
            num: self.num * rhs_den + rhs.num * self_den,
            den: self.den * rhs_den
        }.reduce()
    }
}

impl Fraction {
    fn reduce(mut self) -> Self {
        let gcd = greatest_common_divisor(self.num, self.den);
        self.num /= gcd;
        self.den /= gcd;
        if self.den < 0 {
            self.num *= -1;
            self.den *= -1;
        }
        if self.num.abs() >= self.den.abs() {
            self.int += self.num / self.den;
            self.num %= self.den;
        }
        self
    }
    fn as_int(&self) -> Option<i128> {
        let frac = self.reduce();
        if !(frac.den == 1 && frac.num == 0) {
            None
        } else {
            Some(self.int)
        }
    }
}

fn greatest_common_divisor(a: i128, b: i128) -> i128 {
    if b == 0 {
        a
    } else {
        greatest_common_divisor(b, a % b)
    }
}

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let hailstones = parse_input(input);
        let num_collisions = find_collisions(&hailstones, 200000000000000.0, 400000000000000.0);
        println!("{num_collisions}");
        format!("{num_collisions}")
    }

    fn part_two(&self, input: &str) -> String {
        let hailstones = parse_input(input);
        let hailstones = find_nonparallel_hailstones(hailstones);
        let aug_mat = construct_augmented_matrix(hailstones);
        let rock = gaussian_elimination(aug_mat);
        let coord_sum = rock[0] + rock[1] + rock[2];
        println!("{coord_sum}");
        format!("{coord_sum}")
    }
}

fn parse_input(input: &str) -> Vec<Hailstone> {
    let lines = filter_input_lines(input);
    let re = Regex::new(r"[+-]?\d+").unwrap();
    lines
        .into_iter()
        .map(|line| {
            let cap: Vec<i128> = re.find_iter(line).map(|m| m.as_str().parse().unwrap()).collect();
            Hailstone {
                x: cap[0],
                y: cap[1],
                z: cap[2],
                vx: cap[3],
                vy: cap[4],
                vz: cap[5],
            }
        })
        .collect()
}

fn find_collisions(hailstones: &Vec<Hailstone>, min_time: f64, max_time: f64) -> u32 {
    let len = hailstones.len();
    let mut num_collisions = 0;
    for i in 0..(len - 1) {
        for j in (i + 1)..len {
            let a = &hailstones[i];
            let b = &hailstones[j];
            let a_slope = (a.vy as f64) / (a.vx as f64);
            let b_slope = (b.vy as f64) / (b.vx as f64);
            if (a_slope - b_slope).abs() < 1e-6 {
                continue;
            }
            let x_intersect = ((b.y - a.y) as f64 + a_slope * (a.x as f64) - b_slope * (b.x as f64)) / (a_slope - b_slope);
            if (x_intersect - a.x as f64).signum() != (a.vx.signum() as f64)
                || (x_intersect - b.x as f64).signum() != (b.vx.signum() as f64)
            {
                continue;
            }
            let y_intersect = (a.y as f64) + a_slope * (x_intersect - a.x as f64);
            if x_intersect >= min_time && x_intersect <= max_time && y_intersect >= min_time && y_intersect <= max_time {
                num_collisions += 1;
            }
        }
    }
    num_collisions
}

fn find_nonparallel_hailstones(hailstones: Vec<Hailstone>) -> Vec<Hailstone> {
    let mut hailstones_nonparallel: Vec<Hailstone> = Vec::new();
    'outer: for a in hailstones {
        for b in hailstones_nonparallel.iter() {
            let sx = (a.vx as f64) / (b.vx as f64);
            let sy = (a.vy as f64) / (b.vy as f64);
            let sz = (a.vz as f64) / (b.vz as f64);
            if (sx - sy).abs() < 1e-6 && (sx - sz).abs() < 1e-6 {
                continue 'outer;
            }
        }
        hailstones_nonparallel.push(a);
        if hailstones_nonparallel.len() == 3 {
            break;
        }
    }
    hailstones_nonparallel
}

fn construct_augmented_matrix(hailstones: Vec<Hailstone>) -> Vec<Vec<i128>> {
    let mut aug_mat = Vec::new();
    for j_ind in [1, 2] {
        let hi = &hailstones[0];
        let hj = &hailstones[j_ind];
        let (pix, piy, piz) = (hi.x, hi.y, hi.z);
        let (pjx, pjy, pjz) = (hj.x, hj.y, hj.z);
        let (vix, viy, viz) = (hi.vx, hi.vy, hi.vz);
        let (vjx, vjy, vjz) = (hj.vx, hj.vy, hj.vz);
        let b1 = viy * pix - vix * piy - vjy * pjx + vjx * pjy;
        let b2 = viz * pix - vix * piz - vjz * pjx + vjx * pjz;
        let b3 = viz * piy - viy * piz - vjz * pjy + vjy * pjz;
        aug_mat.push(vec![(viy - vjy), -(vix - vjx), 0, -(piy - pjy), (pix - pjx), 0, b1]);
        aug_mat.push(vec![(viz - vjz), 0, -(vix - vjx), -(piz - pjz), 0, (pix - pjx), b2]);
        aug_mat.push(vec![0, (viz - vjz), -(viy - vjy), 0, -(piz - pjz), (piy - pjy), b3]);
    }
    aug_mat
}

fn gaussian_elimination(aug_mat: Vec<Vec<i128>>) -> Vec<i128> {
    let mut aug_mat: Vec<Vec<Fraction>> = aug_mat
        .into_iter()
        .map(|row| row.into_iter().map(|n| Fraction { int: n, num: 0, den: 1 }).collect())
        .collect();

    let num_rows = aug_mat.len();
    let num_cols = aug_mat[0].len();

    for col in 0..(num_cols - 2) {

        // Swap rows so that current diagonal element is non-zero
        let diag_elem = aug_mat[col][col];
        if diag_elem.int == 0 && diag_elem.num == 0 {
            for i in (col + 1)..num_rows {
                let col_elem = aug_mat[i][col];
                if !(col_elem.int == 0 && col_elem.num == 0) {
                    aug_mat.swap(i, col);
                    break;
                }
            }
        }

        let a_row = aug_mat[col].clone();
        let a = a_row[col];

        // Subtract current diagonal row from lower rows
        for row in (col + 1)..num_rows {
            let b = aug_mat[row][col];
            if b.int == 0 && b.num == 0 {
                continue;
            }
            let m = -b / a;
            for j in 0..num_cols {
                aug_mat[row][j] = aug_mat[row][j] + m * a_row[j];
            }
        }
    }

    // Do substitution in reverse
    let mut x = vec![Fraction { int: 0, num: 0, den: 1 }; num_rows];
    for row in (0..num_rows).rev() {
        let a = aug_mat[row][row];
        let mut b = aug_mat[row].last().unwrap().clone();
        for col in (row + 1)..(num_cols - 1) {
            b = b + -aug_mat[row][col] * x[col];
        }
        x[row] = b / a;
    }

    x.into_iter().map(|v| v.as_int().unwrap()).collect()

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3";
        let hailstones = parse_input(input);
        let num_collisions = find_collisions(&hailstones, 7.0, 27.0);
        assert_eq!(num_collisions, 2);
    }

    #[test]
    fn test_part_two() {
        let input = "
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3";
        let value = Day.part_two(input);
        assert_eq!(value, "47");
    }
}
