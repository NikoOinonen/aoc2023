use std::collections::{HashMap, HashSet, VecDeque};

use aoc2023::filter_input_lines;

use super::Problem;

enum Tile {
    Path,
    Forest,
    Up,
    Down,
    Left,
    Right,
}

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let map = get_map(input);

        let mut queue = VecDeque::new();
        queue.push_back(((0, 1), HashSet::new()));
        let mut end_paths = Vec::new();

        let target_coord = (map.len() - 1, map[0].len() - 2);
        while let Some((coord, visited)) = queue.pop_front() {
            if coord == target_coord {
                end_paths.push(visited);
                continue;
            }
            for neighbour in get_neighbours_part1(coord, &map).into_iter() {
                if visited.contains(&neighbour) {
                    continue;
                }
                let mut new_visited = visited.clone();
                new_visited.insert(neighbour);
                queue.push_back((neighbour, new_visited));
            }
        }

        let longest_path_length = end_paths.into_iter().map(|path| path.len()).max().unwrap();

        println!("{longest_path_length}");
        format!("{longest_path_length}")
    }

    fn part_two(&self, input: &str) -> String {
        let map = get_map(input);

        let mut queue = VecDeque::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        queue.push_back((0, 1));

        let mut map_contracted: HashMap<(usize, usize), HashSet<((usize, usize), usize)>> = HashMap::new();

        while let Some(coord) = queue.pop_front() {
            let mut contracted_neighbours = HashSet::new();
            for neighbour in get_neighbours_part2(coord, &map).into_iter() {
                if visited.contains(&neighbour) {
                    continue;
                }
                let (new_coord, path_len, last_coord) = find_next_fork(coord, neighbour, &map);
                visited.insert(last_coord);
                contracted_neighbours.insert((new_coord, path_len));
                queue.push_back(new_coord);
                match map_contracted.get_mut(&new_coord) {
                    Some(vec) => {
                        vec.insert((coord, path_len));
                    },
                    None => {
                        map_contracted.insert(new_coord, vec![(coord, path_len)].into_iter().collect());
                    }
                };
            }
            match map_contracted.get_mut(&coord) {
                Some(vec) => {
                    vec.extend(contracted_neighbours.into_iter());
                },
                None => {
                    map_contracted.insert(coord, contracted_neighbours);
                }
            };
        }


        let mut queue: VecDeque<((usize, usize), HashSet<&(usize, usize)>, usize)> = VecDeque::new();
        queue.push_back(((0, 1), HashSet::new(), 0));
        let mut end_paths = Vec::new();

        let target_coord = (map.len() - 1, map[0].len() - 2);
        while let Some((coord, visited, path_len)) = queue.pop_front() {
            if coord == target_coord {
                end_paths.push((visited, path_len));
                continue;
            }
            for (neighbour, added_path_len) in map_contracted.get(&coord).unwrap() {
                if visited.contains(&neighbour) {
                    continue;
                }
                let mut new_visited = visited.clone();
                new_visited.insert(neighbour);
                queue.push_back((*neighbour, new_visited, path_len + added_path_len));
            }
        }

        let longest_path_length = end_paths.into_iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

        println!("{longest_path_length}");
        format!("{longest_path_length}")
    }
}

fn get_map(input: &str) -> Vec<Vec<Tile>> {
    let lines = filter_input_lines(input);

    let mut map = Vec::new();
    for line in lines {
        let mut tiles = Vec::new();
        for c in line.chars() {
            let tile = match c {
                '.' => Tile::Path,
                '#' => Tile::Forest,
                '^' => Tile::Up,
                'v' => Tile::Down,
                '<' => Tile::Left,
                '>' => Tile::Right,
                _ => panic!(),
            };
            tiles.push(tile);
        }
        map.push(tiles);
    }

    map
}

fn find_next_fork(prev: (usize, usize), start: (usize, usize), map: &Vec<Vec<Tile>>) -> ((usize, usize), usize, (usize, usize)) {
    let mut visited = vec![prev, start];
    let mut current_coord = start;
    loop {
        let mut neighbours: Vec<(usize, usize)> = get_neighbours_part2(current_coord, map)
            .into_iter()
            .filter(|coord| !visited.contains(&coord))
            .collect();
        if neighbours.len() == 1 {
            let neighbour = neighbours.pop().unwrap();
            visited.push(neighbour);
            current_coord = neighbour;
        } else {
            let path_len = visited.len() - 1;
            visited.pop();
            let last_coord = visited.pop().unwrap();
            return (current_coord, path_len, last_coord);
        }
    }
}

fn get_neighbours_part1(coord: (usize, usize), map: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let coord = (coord.0 as i32, coord.1 as i32);
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(row_offset, col_offset)| {
            let new_row = coord.0 + row_offset;
            let new_col = coord.1 + col_offset;
            if new_row < 0 || new_row >= height || new_col < 0 || new_col >= width {
                return None;
            }
            let new_coord = (new_row as usize, new_col as usize);
            let tile = &map[new_coord.0][new_coord.1];
            match tile {
                Tile::Path => Some(new_coord),
                Tile::Forest => None,
                Tile::Up => {
                    if row_offset == 1 {
                        None
                    } else {
                        Some(new_coord)
                    }
                }
                Tile::Down => {
                    if row_offset == -1 {
                        None
                    } else {
                        Some(new_coord)
                    }
                }
                Tile::Left => {
                    if col_offset == 1 {
                        None
                    } else {
                        Some(new_coord)
                    }
                }
                Tile::Right => {
                    if col_offset == -1 {
                        None
                    } else {
                        Some(new_coord)
                    }
                }
            }
        })
        .collect()
}

fn get_neighbours_part2(coord: (usize, usize), map: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let coord = (coord.0 as i32, coord.1 as i32);
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(row_offset, col_offset)| {
            let new_row = coord.0 + row_offset;
            let new_col = coord.1 + col_offset;
            if new_row < 0 || new_row >= height || new_col < 0 || new_col >= width {
                return None;
            }
            let new_coord = (new_row as usize, new_col as usize);
            let tile = &map[new_coord.0][new_coord.1];
            match tile {
                Tile::Forest => None,
                _ => Some(new_coord),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#";
        let value = Day.part_one(input);
        assert_eq!(value, "94");
    }

    #[test]
    fn test_part_two() {
        let input = "
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#";
        let value = Day.part_two(input);
        assert_eq!(value, "154");
    }
}
