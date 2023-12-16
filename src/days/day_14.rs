use std::collections::HashSet;

use aoc2023::filter_input_lines;

use super::Problem;

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let lines = filter_input_lines(input);

        let (mut round_rock_positions, square_rock_positions) = get_rock_positions(&lines);

        let height = lines.len();
        let width = lines[0].len();
        print_board(&round_rock_positions, &square_rock_positions, lines.len(), lines[0].len());
        tilt_board_north(&mut round_rock_positions, &square_rock_positions, width, height);

        print_board(&round_rock_positions, &square_rock_positions, lines.len(), lines[0].len());

        let total_load: usize = round_rock_positions.iter().map(|(i, _)| height - i).sum();

        println!("{total_load}");
        format!("{total_load}")
    }

    fn part_two(&self, input: &str) -> String {
        let lines = filter_input_lines(input);

        let (mut round_rock_positions, square_rock_positions) = get_rock_positions(&lines);

        let height = lines.len();
        let width = lines[0].len();
        let mut visited = Vec::new();
        let total_cycles = 1_000_000_000;
        for cycle in 0..total_cycles {
            tilt_board_north(&mut round_rock_positions, &square_rock_positions, width, height);
            tilt_board_west(&mut round_rock_positions, &square_rock_positions, width, height);
            tilt_board_south(&mut round_rock_positions, &square_rock_positions, width, height);
            tilt_board_east(&mut round_rock_positions, &square_rock_positions, width, height);
            match visited.iter().position(|x| x == &round_rock_positions) {
                None => visited.push(round_rock_positions.clone()),
                Some(ind) => {
                    let repeat_length = cycle - ind;
                    let rem = (total_cycles - (cycle + 1)) % repeat_length;
                    round_rock_positions = visited[ind + rem].clone();
                    break;
                }
            }
        }

        let total_load: usize = round_rock_positions.iter().map(|(i, _)| height - i).sum();

        println!("{total_load}");
        format!("{total_load}")
    }
}

fn get_rock_positions(lines: &Vec<&str>) -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>) {
    let mut round_rock_positions = HashSet::new();
    let mut square_rock_positions = HashSet::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                'O' => {
                    round_rock_positions.insert((i, j));
                }
                '#' => {
                    square_rock_positions.insert((i, j));
                }
                _ => (),
            }
        }
    }
    (round_rock_positions, square_rock_positions)
}

fn print_board(
    round_rock_positions: &HashSet<(usize, usize)>,
    square_rock_positions: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) {
    let mut board = String::new();
    for i in 0..height {
        for j in 0..width {
            if round_rock_positions.contains(&(i, j)) {
                board.push('O');
            } else if square_rock_positions.contains(&(i, j)) {
                board.push('#');
            } else {
                board.push('.');
            }
        }
        board.push('\n');
    }
    print!("\n{board}");
}

fn tilt_board_north(
    round_rock_positions: &mut HashSet<(usize, usize)>,
    square_rock_positions: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) {
    for j in 0..width {
        let mut last_open_pos = 0;
        for i in 0..height {
            if round_rock_positions.remove(&(i, j)) {
                round_rock_positions.insert((last_open_pos, j));
                last_open_pos += 1;
            } else if square_rock_positions.contains(&(i, j)) {
                last_open_pos = i + 1;
            }
        }
    }
}

fn tilt_board_west(
    round_rock_positions: &mut HashSet<(usize, usize)>,
    square_rock_positions: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) {
    for i in 0..height {
        let mut last_open_pos = 0;
        for j in 0..width {
            if round_rock_positions.remove(&(i, j)) {
                round_rock_positions.insert((i, last_open_pos));
                last_open_pos += 1;
            } else if square_rock_positions.contains(&(i, j)) {
                last_open_pos = j + 1;
            }
        }
    }
}

fn tilt_board_south(
    round_rock_positions: &mut HashSet<(usize, usize)>,
    square_rock_positions: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) {
    for j in 0..width {
        let mut last_blocked_pos = height;
        for i in (0..height).rev() {
            if round_rock_positions.remove(&(i, j)) {
                last_blocked_pos -= 1;
                round_rock_positions.insert((last_blocked_pos, j));
            } else if square_rock_positions.contains(&(i, j)) {
                last_blocked_pos = i;
            }
        }
    }
}

fn tilt_board_east(
    round_rock_positions: &mut HashSet<(usize, usize)>,
    square_rock_positions: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) {
    for i in 0..height {
        let mut last_blocked_pos = width;
        for j in (0..width).rev() {
            if round_rock_positions.remove(&(i, j)) {
                last_blocked_pos -= 1;
                round_rock_positions.insert((i, last_blocked_pos));
            } else if square_rock_positions.contains(&(i, j)) {
                last_blocked_pos = j;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....";
        let value = Day.part_one(input);
        assert_eq!(value, "136");
    }

    #[test]
    fn test_part_two() {
        let input = "
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....";
        let value = Day.part_two(input);
        assert_eq!(value, "64");
    }
}
