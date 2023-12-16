use std::collections::HashSet;

use aoc2023::filter_input_lines;

use super::Problem;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Beam {
    pos: (usize, usize),
    dir: Direction,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let tiles: Vec<Vec<char>> = filter_input_lines(input).iter().map(|line| line.chars().collect()).collect();

        let num_energized = run_beam(
            &tiles,
            Beam {
                pos: (0, 0),
                dir: Direction::Right,
            },
        );

        println!("{num_energized}");
        format!("{num_energized}")
    }

    fn part_two(&self, input: &str) -> String {
        let tiles: Vec<Vec<char>> = filter_input_lines(input).iter().map(|line| line.chars().collect()).collect();

        let height = tiles.len();
        let width = tiles[0].len();
        let mut initial_beams = Vec::new();
        initial_beams.extend((0..height).map(|i| Beam { pos: (i         ,         0), dir: Direction::Right}).collect::<Vec<Beam>>());
        initial_beams.extend((0..height).map(|i| Beam { pos: (i         , width - 1), dir: Direction::Left }).collect::<Vec<Beam>>());
        initial_beams.extend((0..width ).map(|j| Beam { pos: (0         ,         j), dir: Direction::Down }).collect::<Vec<Beam>>());
        initial_beams.extend((0..width ).map(|j| Beam { pos: (height - 1,         j), dir: Direction::Up   }).collect::<Vec<Beam>>());
        
        let num_energized = initial_beams.into_iter().map(|beam| run_beam(&tiles, beam)).max().unwrap();
        
        println!("{num_energized}");
        format!("{num_energized}")
    }
}

fn run_beam(tiles: &Vec<Vec<char>>, initial_beam: Beam) -> usize {
    let height = tiles.len();
    let width = tiles[0].len();
    let mut beam_positions = vec![initial_beam];
    let mut energized: HashSet<Beam> = HashSet::new();
    while beam_positions.len() > 0 {
        let mut new_beam_positions = Vec::new();
        while let Some(beam) = beam_positions.pop() {
            if energized.contains(&beam) {
                continue;
            }
            energized.insert(beam);
            let tile = tiles[beam.pos.0][beam.pos.1];
            let new_beam_pos = get_new_beam_positions(beam, tile, width, height);
            new_beam_positions.extend(new_beam_pos);
        }
        beam_positions = new_beam_positions;
    }

    // print_energized(&energized, width, height);

    energized.iter().map(|b| b.pos).collect::<HashSet<(usize, usize)>>().len()
}

fn get_new_beam_positions(beam: Beam, tile: char, width: usize, height: usize) -> Vec<Beam> {
    let Beam { pos, dir } = beam;
    let pos = (pos.0 as i32, pos.1 as i32);
    let new_directions: Vec<Direction> = match tile {
        '.' => vec![dir],
        '/' => match dir {
            Direction::Up => vec![Direction::Right],
            Direction::Down => vec![Direction::Left],
            Direction::Left => vec![Direction::Down],
            Direction::Right => vec![Direction::Up],
        },
        '\\' => match dir {
            Direction::Up => vec![Direction::Left],
            Direction::Down => vec![Direction::Right],
            Direction::Left => vec![Direction::Up],
            Direction::Right => vec![Direction::Down],
        },
        '-' => match dir {
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Left => vec![Direction::Left],
            Direction::Right => vec![Direction::Right],
        },
        '|' => match dir {
            Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            Direction::Down => vec![Direction::Down],
            Direction::Up => vec![Direction::Up],
        },
        _ => panic!(),
    };
    new_directions
        .into_iter()
        .filter_map(|dir| {
            let offset = dir.offset();
            let i = pos.0 + offset.0;
            let j = pos.1 + offset.1;
            if (i >= 0) && (i < (height as i32)) && (j >= 0) && (j < (width as i32)) {
                Some(Beam {
                    pos: (i as usize, j as usize),
                    dir,
                })
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....";
        let value = Day.part_one(input);
        assert_eq!(value, "46");
    }

    #[test]
    fn test_part_two() {
        let input = r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....";
        let value = Day.part_two(input);
        assert_eq!(value, "51");
    }
}
