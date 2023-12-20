use std::collections::HashSet;

use aoc2023::filter_input_lines;
use regex::Regex;

use super::Problem;

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let lines = filter_input_lines(input);
        let re = Regex::new(r"(?<dir>.)\s*(?<amount>\d+)\s*\((?<color>#[a-fA-F0-9]{6})\)").unwrap();
        let mut pos = (0, 0);
        let mut squares: HashSet<(i32, i32)> = HashSet::new();
        for line in lines {
            let cap = re.captures(line).unwrap();
            let dir = &cap["dir"];
            let amount: i32 = cap["amount"].parse().unwrap();
            let offset = match dir {
                "R" => (0, 1),
                "L" => (0, -1),
                "U" => (-1, 0),
                "D" => (1, 0),
                _ => panic!(),
            };
            for _ in 0..amount {
                pos.0 += offset.0;
                pos.1 += offset.1;
                squares.insert(pos.clone());
            }
        }

        print_squares(&squares);

        let start_pos = find_inside_square(&squares).unwrap();
        let mut visited = HashSet::new();
        let mut to_visit = vec![start_pos];

        while let Some(pos) = to_visit.pop() {
            visited.insert(pos);
            for neighbour in get_neighbours(pos, &squares) {
                if !visited.contains(&neighbour) {
                    to_visit.push(neighbour);
                }
            }
        }

        squares.extend(visited);
        let num_squares = squares.len();

        println!("{num_squares}");
        format!("{num_squares}")
    }

    fn part_two(&self, input: &str) -> String {
        println!("{input}");
        format!("{input}")
    }
}

fn print_squares(squares: &HashSet<(i32, i32)>) {
    let min_row = squares.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_row = squares.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let min_col = squares.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let max_col = squares.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let mut s = String::new();
    for i in min_row..=max_row {
        for j in min_col..=max_col {
            if squares.contains(&(i, j)) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    println!("{s}")
}

fn find_inside_square(squares: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    let min_row = squares.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_row = squares.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let min_col = squares.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let max_col = squares.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    for i in (min_row+1)..=max_row {
        let mut outside = true;
        for j in min_col..=max_col {
            if squares.contains(&(i, j)) {
                outside = false;
            } else if !outside {
                return Some((i, j));
            }
        }
    }
    None
}

fn get_neighbours(pos: (i32, i32), squares: &HashSet<(i32, i32)>) -> Vec<(i32, i32)> {
    vec![(1, 0), (-1, 0), (0, 1), (0, -1)].into_iter().filter_map(|offset| {
        let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
        if squares.contains(&new_pos) {
            None
        } else {
            Some(new_pos)
        }
    }).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)";
        let value = Day.part_one(input);
        assert_eq!(value, "62");
    }

    #[test]
    fn test_part_two() {
        let input = "Part 2";
        let value = Day.part_two(input);
        assert_eq!(value, "Part 2");
    }
}
