use std::collections::{HashSet, VecDeque};

use aoc2023::filter_input_lines;

use super::Problem;

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let lines = filter_input_lines(input);
        let (tiles, start_tile, _) = build_map(&lines);

        let mut visited: HashSet<(usize, usize)> = vec![start_tile].into_iter().collect();
        let mut to_visit: VecDeque<((usize, usize), usize)> =
            tiles[start_tile.0][start_tile.1].clone().into_iter().map(|n| (n, 1)).collect();
        let mut max_depth = 0;
        while let Some((current_tile, depth)) = to_visit.pop_front() {
            max_depth = max_depth.max(depth);
            visited.insert(current_tile);
            for neighbour in tiles[current_tile.0][current_tile.1].iter() {
                if !visited.contains(neighbour) {
                    to_visit.push_back((neighbour.clone(), depth + 1));
                }
            }
        }

        println!("{max_depth}");
        format!("{max_depth}")
    }

    fn part_two(&self, input: &str) -> String {
        let lines = filter_input_lines(input);
        let (tiles, start_tile, start_tile_type) = build_map(&lines);

        let mut loop_tiles: HashSet<(usize, usize)> = vec![start_tile].into_iter().collect();
        let mut to_visit: VecDeque<(usize, usize)> = tiles[start_tile.0][start_tile.1].clone().into_iter().collect();
        while let Some(current_tile) = to_visit.pop_front() {
            loop_tiles.insert(current_tile);
            for neighbour in tiles[current_tile.0][current_tile.1].iter() {
                if !loop_tiles.contains(neighbour) {
                    to_visit.push_back(neighbour.clone());
                }
            }
        }

        let mut last_loop_tile = ' ';
        let mut inside = false;
        let mut num_inside_tiles = 0;
        for (i, line) in lines.iter().enumerate() {
            for (j, mut c) in line.chars().enumerate() {
                if c == 'S' {
                    c = start_tile_type;
                }
                if loop_tiles.contains(&(i, j)) {
                    match c {
                        '|' => inside = !inside,
                        'F' | 'L' => last_loop_tile = c,
                        'J' => {
                            if last_loop_tile == 'F' {
                                inside = !inside;
                            }
                        },
                        '7' => {
                            if last_loop_tile == 'L' {
                                inside = !inside;
                            }
                        },
                        _ => (),
                    }
                } else if inside {
                    num_inside_tiles += 1;
                }
            }
        }

        println!("{num_inside_tiles}");
        format!("{num_inside_tiles}")
    }
}

fn build_map(lines: &Vec<&str>) -> (Vec<Vec<Vec<(usize, usize)>>>, (usize, usize), char) {
    let mut tiles = Vec::new();
    let mut start_tile = (0, 0);
    for (i, line) in lines.iter().enumerate() {
        let mut line_tiles = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start_tile = (i, j);
                    line_tiles.push(vec![]);
                }
                '|' => {
                    if i == 0 {
                        line_tiles.push(vec![]);
                    } else {
                        line_tiles.push(vec![(i - 1, j), (i + 1, j)])
                    }
                }
                '-' => {
                    if j == 0 {
                        line_tiles.push(vec![]);
                    } else {
                        line_tiles.push(vec![(i, j - 1), (i, j + 1)])
                    }
                }
                'L' => {
                    if i == 0 {
                        line_tiles.push(vec![]);
                    } else {
                        line_tiles.push(vec![(i - 1, j), (i, j + 1)])
                    }
                }
                'J' => {
                    if (i == 0) || (j == 0) {
                        line_tiles.push(vec![]);
                    } else {
                        line_tiles.push(vec![(i - 1, j), (i, j - 1)])
                    }
                }
                '7' => {
                    if j == 0 {
                        line_tiles.push(vec![]);
                    } else {
                        line_tiles.push(vec![(i + 1, j), (i, j - 1)])
                    }
                }
                'F' => line_tiles.push(vec![(i + 1, j), (i, j + 1)]),
                _ => line_tiles.push(vec![]),
            }
        }
        tiles.push(line_tiles);
    }

    let height = tiles.len();
    let width = tiles[0].len();
    let mut start_neighbours = Vec::new();
    let mut offsets = Vec::new();
    for offset in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let i = start_tile.0 as i32 + offset.0;
        let j = start_tile.1 as i32 + offset.1;
        if (i < 0) || (j < 0) || (i >= height as i32) || (j >= width as i32) {
            continue;
        }
        let neighbour = (i as usize, j as usize);
        if tiles[neighbour.0][neighbour.1].contains(&start_tile) {
            offsets.push(offset);
            start_neighbours.push(neighbour);
        }
    }
    tiles[start_tile.0][start_tile.1] = start_neighbours;

    let start_tile_type = match offsets[..] {
        [(-1, 0), (1, 0)] => '|',
        [(0, -1), (0, 1)] => '-',
        [(-1, 0), (0, 1)] => 'L',
        [(-1, 0), (0, -1)] => 'J',
        [(1, 0), (0, -1)] => '7',
        [(1, 0), (0, 1)] => 'F',
        _ => panic!(),
    };

    (tiles, start_tile, start_tile_type)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let value = Day.part_one(
            "
            .....
            .S-7.
            .|.|.
            .L-J.
            .....",
        );
        assert_eq!(value, "4");
        let value = Day.part_one(
            "
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...",
        );
        assert_eq!(value, "8");
    }

    #[test]
    fn test_part_two() {
        let value = Day.part_two(
            "
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........",
        );
        assert_eq!(value, "4");
        let value = Day.part_two(
            "
            ..........
            .S------7.
            .|F----7|.
            .||....||.
            .||....||.
            .|L-7F-J|.
            .|..||..|.
            .L--JL--J.
            ..........",
        );
        assert_eq!(value, "4");
        let value = Day.part_two(
            "
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...",
        );
        assert_eq!(value, "8");
    }
}
