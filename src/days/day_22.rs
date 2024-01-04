use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use aoc2023::filter_input_lines;
use itertools::Itertools;

use super::Problem;

#[derive(Debug)]
struct Brick {
    start: (usize, usize, usize),
    end: (usize, usize, usize),
    supports: HashSet<usize>,
    supported_by: HashSet<usize>,
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
            supports: HashSet::new(),
            supported_by: HashSet::new(),
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
}

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let bricks: Vec<Brick> = filter_input_lines(input).iter().map(|line| line.parse().unwrap()).collect();
        let bricks = drop_bricks(bricks);

        let num_bricks: usize = bricks
            .iter()
            .enumerate()
            .filter(|(i, _)| {
                bricks
                    .iter()
                    .enumerate()
                    .all(|(j, brick)| (*i == j) || !(brick.supported_by.contains(i) && brick.supported_by.len() == 1))
            })
            .count();

        println!("{num_bricks}");
        format!("{num_bricks}")
    }

    fn part_two(&self, input: &str) -> String {
        let bricks: Vec<Brick> = filter_input_lines(input).iter().map(|line| line.parse().unwrap()).collect();
        let bricks = drop_bricks(bricks);

        let total_fallen_bricks: usize = bricks
            .iter()
            .enumerate()
            .map(|(i, disintegrated_brick)| {
                let mut fallen_bricks: HashSet<usize> = HashSet::from([i]);
                let mut potential_fallen_bricks: VecDeque<usize> = VecDeque::new();
                potential_fallen_bricks.extend(disintegrated_brick.supports.iter());
                while let Some(brick_ind) = potential_fallen_bricks.pop_front() {
                    let brick = &bricks[brick_ind];
                    if brick.supported_by.difference(&fallen_bricks).count() == 0 {
                        fallen_bricks.insert(brick_ind);
                        potential_fallen_bricks.extend(brick.supports.iter());
                    }
                }
                fallen_bricks.len() - 1
            })
            .sum();

        println!("{total_fallen_bricks}");
        format!("{total_fallen_bricks}")
    }
}

fn drop_bricks(mut bricks: Vec<Brick>) -> Vec<Brick> {
    bricks.sort_by(|a, b| a.zmin().cmp(&b.zmin()));
    let mut dropped_zmax: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut dropped_bricks: Vec<Brick> = Vec::new();
    for (i, brick) in bricks.iter().enumerate() {
        let zmax_with_inds: Vec<&(usize, usize)> = brick
            .coords()
            .iter()
            .map(|(x, y, _)| dropped_zmax.get(&(*x, *y)).unwrap_or(&(0, 0)))
            .collect();
        let zmax = zmax_with_inds.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
        let zmax_inds: HashSet<usize> = if zmax > 0 {
            zmax_with_inds
                .iter()
                .filter_map(|v| if v.0 == zmax { Some(v.1) } else { None })
                .unique()
                .collect()
        } else {
            HashSet::new()
        };
        let brick_z_size = brick.start.2.abs_diff(brick.end.2);
        let z_start = zmax + 1;
        let z_end = z_start + brick_z_size;
        let start = (brick.start.0, brick.start.1, z_start);
        let end = (brick.end.0, brick.end.1, z_end);
        for (x, y, _) in brick.coords() {
            dropped_zmax.insert((x, y), (z_end, i));
        }
        for ind in zmax_inds.iter() {
            dropped_bricks[*ind].supports.insert(i);
        }
        dropped_bricks.push(Brick {
            start,
            end,
            supported_by: zmax_inds,
            supports: HashSet::new(),
        });
    }
    dropped_bricks
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
        let input = "
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9";
        let value = Day.part_two(input);
        assert_eq!(value, "7");
    }
}
