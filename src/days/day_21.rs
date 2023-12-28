use std::collections::{HashSet, VecDeque};

use aoc2023::filter_input_lines;

use super::Problem;

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let final_positions = get_final_positions_part1(input, 64);
        let num_positions = final_positions.len();
        println!("{num_positions}");
        format!("{num_positions}")
    }

    fn part_two(&self, input: &str) -> String {
        let (rocks, start_pos, width, height) = parse_input(input);
        let (evens, odds) = get_even_odd_squares(start_pos, &rocks, width, height, usize::MAX);
        let num_odds = odds.len();
        let num_evens = evens.len();

        let total_steps = 26501365;
        assert_eq!(width, height);
        let num_map_steps = total_steps / width;
        let num_even_maps = (num_map_steps - 1).pow(2);
        let num_odd_maps = num_map_steps.pow(2);

        let complete_map_squares = num_even_maps * num_odds + num_odd_maps * num_evens;

        let middle = width / 2;
        let num_steps_left = total_steps - (num_map_steps - 1) * width - middle - 1;
        let side_odd_squares: usize = vec![(middle, width - 1), (height - 1, middle), (middle, 0), (0, middle)]
            .into_iter()
            .map(|pos| {
                let (e, _) = get_even_odd_squares(pos, &rocks, width, height, num_steps_left);
                e.len()
            })
            .sum();

        let num_steps_left1 = total_steps - (num_map_steps - 1) * width - 2 * middle - 2;
        let num_steps_left2 = total_steps - (num_map_steps - 2) * width - 2 * middle - 2;
        let corner_odd_squares: usize = vec![(height - 1, width - 1), (height - 1, 0), (0, 0), (0, width - 1)]
            .into_iter()
            .map(|pos| {
                let (e1, _) = get_even_odd_squares(pos, &rocks, width, height, num_steps_left1);
                let (_, o2) = get_even_odd_squares(pos, &rocks, width, height, num_steps_left2);
                num_map_steps * e1.len() + (num_map_steps - 1) * o2.len()
            })
            .sum();

        let num_garden_squares = complete_map_squares + side_odd_squares + corner_odd_squares;
        println!("{num_garden_squares}");
        format!("{num_garden_squares}")
    }
}

fn get_final_positions_part1(input: &str, num_steps: usize) -> Vec<(usize, usize)> {
    let (rocks, start_pos, width, height) = parse_input(input);

    let reachable_positions = get_reachable_positions(start_pos, &rocks, width, height, num_steps);

    let parity = (num_steps as usize) % 2;
    let final_positions: Vec<(usize, usize)> = reachable_positions
        .into_iter()
        .filter(|pos| {
            let dist = start_pos.0.abs_diff(pos.0) + start_pos.1.abs_diff(pos.1);
            (dist % 2) == parity
        })
        .collect();

    // print_board(&final_positions, &rocks, width, height);

    final_positions
}

fn get_reachable_positions(
    start_pos: (usize, usize),
    rocks: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
    max_steps: usize,
) -> HashSet<(usize, usize)> {
    let mut to_visit: VecDeque<((usize, usize), usize)> = vec![(start_pos, 0)].into_iter().collect();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some((pos, time_step)) = to_visit.pop_front() {
        if time_step >= max_steps {
            continue;
        }
        for neighbour in get_neighbours(pos, &rocks, width, height) {
            if !visited.contains(&neighbour) {
                to_visit.push_back((neighbour, time_step + 1));
                visited.insert(neighbour);
            }
        }
    }

    visited
}

fn get_even_odd_squares(
    start_pos: (usize, usize),
    rocks: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
    max_steps: usize,
) -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>) {
    let reachable_positions = get_reachable_positions(start_pos, rocks, width, height, max_steps);

    let mut evens = HashSet::new();
    let mut odds = HashSet::new();
    for pos in reachable_positions.into_iter() {
        let dist = start_pos.0.abs_diff(pos.0) + start_pos.1.abs_diff(pos.1);
        if (dist % 2) == 0 {
            evens.insert(pos);
        } else {
            odds.insert(pos);
        }
    }

    (evens, odds)
}

fn parse_input(input: &str) -> (HashSet<(usize, usize)>, (usize, usize), usize, usize) {
    let lines = filter_input_lines(input);
    let height = lines.len();
    let width = lines[0].len();

    let mut rocks = HashSet::new();
    let mut start_pos = (0, 0);
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    rocks.insert((i, j));
                }
                'S' => {
                    start_pos = (i, j);
                }
                _ => (),
            }
        }
    }

    (rocks, start_pos, width, height)
}

fn get_neighbours(pos: (usize, usize), rocks: &HashSet<(usize, usize)>, width: usize, height: usize) -> Vec<(usize, usize)> {
    let width = width as i32;
    let height = height as i32;
    vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|offset| {
            let new_pos = (pos.0 as i32 + offset.0, pos.1 as i32 + offset.1);
            if new_pos.0 < 0 || new_pos.0 >= height || new_pos.1 < 0 || new_pos.1 >= width {
                return None;
            }
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            if rocks.contains(&new_pos) {
                None
            } else {
                Some(new_pos)
            }
        })
        .collect()
}

#[allow(dead_code)]
fn print_board(positions: &Vec<(usize, usize)>, rocks: &HashSet<(usize, usize)>, width: usize, height: usize) {
    let mut board = String::new();
    for i in 0..height {
        for j in 0..width {
            let pos = (i, j);
            if positions.contains(&pos) {
                board.push('O');
            } else if rocks.contains(&pos) {
                board.push('#');
            } else {
                board.push('.');
            }
        }
        board.push('\n');
    }
    println!("{board}");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........";
        let final_positions = get_final_positions_part1(input, 6);
        assert_eq!(final_positions.len(), 16);
    }

}
