use std::{collections::HashMap, str::FromStr};

use aoc2023::filter_input_lines;

use super::Problem;

#[derive(Debug)]
struct Brick {
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

impl FromStr for Brick {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('~').unwrap();
        let start: Vec<usize> = start.split(',').map(|s| s.parse().unwrap()).collect();
        let end: Vec<usize> = end.split(',').map(|s| s.parse().unwrap()).collect();
        Ok(Brick {
            start: (start[0], start[1], start[2]),
            end: (end[0], end[1], end[2]),
        })
    }
}

impl Brick {
    fn coords(&self) -> Vec<(usize, usize, usize)> {
        let mut coords = Vec::new();
        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                for z in self.start.2..=self.end.2 {
                    coords.push((x, y, z));
                }
            }
        }
        coords
    }
    fn zmin(&self) -> usize {
        self.start.2.min(self.end.2)
    }
    // fn contains(&self, coord: (usize, usize, usize)) -> bool {
    //     for x in self.start.0..=self.end.0 {
    //         if x != coord.0 {
    //             continue;
    //         }
    //         for y in self.start.1..=self.end.1 {
    //             if y != coord.1 {
    //                 continue;
    //             }
    //             for z in self.start.2..=self.end.2 {
    //                 if z == coord.2 {
    //                     return true;
    //                 }
    //             }
    //         }
    //     }
    //     false
    //     // self.coords().contains(&coord)
    // }

    fn contains(&self, coord: (usize, usize, usize)) -> bool {
        let r = (
            self.end.0 as i32 - self.start.0 as i32,
            self.end.1 as i32 - self.start.1 as i32,
            self.end.2 as i32 - self.start.2 as i32,
        );
        let r2 = (r.0 * r.0 + r.1 * r.1 + r.2 * r.2) as f64;
        if r2 < 0.5 {
            return (coord.0 == self.start.0) && (coord.1 == self.start.1) && (coord.2 == self.start.2);
        }
        let c = (
            coord.0 as i32 - self.start.0 as i32,
            coord.1 as i32 - self.start.1 as i32,
            coord.2 as i32 - self.start.2 as i32,
        );
        let c2 = (c.0 * c.0 + c.1 * c.1 + c.2 * c.2) as f64;
        let dot = (r.0 * c.0 + r.1 * c.1 + r.2 * c.2) as f64;
        let proj_frac = dot / r2;
        let proj2 = dot * proj_frac;
        let dist = (c2 - proj2).sqrt();
        // println!("c = {c:?}, c2 = {c2}, r = {r:?}, r2 = {r2}, dot = {dot}, proj_frac = {proj_frac}, proj2 = {proj2}, dist = {dist}");
        (dist < 1e-6) && (proj_frac > -1e-6) && (proj_frac < 1.0000001)
    }
}

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let bricks: Vec<Brick> = filter_input_lines(input).iter().map(|line| line.parse().unwrap()).collect();

        let bricks = drop_bricks(bricks);

        let disintegratable_bricks: Vec<&Brick> = bricks
            .iter()
            .enumerate()
            .filter_map(|(i, test_brick)| {
                let all_supported = bricks.iter().enumerate().all(|(j, brick)| {
                    if i == j {
                        return true;
                    }
                    for coord in brick.coords() {
                        let coord_under = (coord.0, coord.1, coord.2 - 1);
                        if coord_under.2 == 0 {
                            return true;
                        }
                        if bricks
                            .iter()
                            .enumerate()
                            .any(|(k, b)| (k != i) && (k != j) && b.contains(coord_under))
                        {
                            return true;
                        }
                    }
                    false
                });
                if all_supported {
                    Some(test_brick)
                } else {
                    None
                }
            })
            .collect();

        let num_bricks = disintegratable_bricks.len();
        println!("{num_bricks}");
        format!("{num_bricks}")
    }

    fn part_two(&self, input: &str) -> String {
        println!("{input}");
        format!("{input}")
    }
}

fn drop_bricks(mut bricks: Vec<Brick>) -> Vec<Brick> {
    bricks.sort_by(|a, b| a.zmin().cmp(&b.zmin()));
    let mut dropped_zmax: HashMap<(usize, usize), usize> = HashMap::new();
    bricks
        .iter()
        .map(|brick| {
            let zmax = brick
                .coords()
                .iter()
                .map(|(x, y, _)| dropped_zmax.get(&(*x, *y)).unwrap_or(&0))
                .max()
                .unwrap();
            let brick_z_size = brick.start.2.abs_diff(brick.end.2);
            let z_start = zmax + 1;
            let z_end = z_start + brick_z_size;
            let start = (brick.start.0, brick.start.1, z_start);
            let end = (brick.end.0, brick.end.1, z_end);
            for (x, y, _) in brick.coords() {
                dropped_zmax.insert((x, y), z_end);
            }
            Brick { start, end }
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9";
        let value = Day.part_one(input);
        assert_eq!(value, "5");
    }

    #[test]
    fn test_part_two() {
        let input = "Part 2";
        let value = Day.part_two(input);
        assert_eq!(value, "Part 2");
    }
}
