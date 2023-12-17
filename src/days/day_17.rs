use std::{
    collections::{BinaryHeap, HashMap},
    vec,
};

use aoc2023::filter_input_lines;

use super::Problem;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
    fn opposite_direction(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct State {
    row: usize,
    col: usize,
    heat_loss: u32,
    straight_steps: u32,
    direction: Direction,
}

impl State {
    fn get_neighbours(&self, tiles: &Vec<Vec<u32>>, width: usize, height: usize, min_steps: u32, max_steps: u32) -> Vec<State> {
        let height = height as i32;
        let width = width as i32;
        vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right]
            .into_iter()
            .filter_map(|dir| {
                if !(self.row == 0 && self.col == 0) && ((dir == self.direction.opposite_direction()) || (dir == self.direction)) {
                    return None;
                }
                let offset = dir.offset();
                let mut neighbours = Vec::new();
                let mut new_row = self.row as i32;
                let mut new_col = self.col as i32;
                let mut new_heat_loss = self.heat_loss;
                for dist in 1..=max_steps {
                    new_row += offset.0;
                    new_col += offset.1;
                    if (new_row < 0) || (new_col < 0) || (new_row >= height) || (new_col >= width) {
                        break;
                    }
                    new_heat_loss += tiles[new_row as usize][new_col as usize];
                    if dist >= min_steps {
                        neighbours.push(State {
                            row: new_row as usize,
                            col: new_col as usize,
                            heat_loss: new_heat_loss,
                            straight_steps: dist,
                            direction: dir,
                        })
                    }
                }
                Some(neighbours)
            })
            .flatten()
            .collect()
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .heat_loss
            .cmp(&self.heat_loss)
            .then_with(|| self.row.cmp(&other.row))
            .then_with(|| self.col.cmp(&other.col))
            .then_with(|| self.straight_steps.cmp(&other.straight_steps))
            .then_with(|| (self.direction as u32).cmp(&(other.direction as u32)))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let min_heat_loss = get_min_heat_loss(input, 1, 3);
        println!("{min_heat_loss}");
        format!("{min_heat_loss}")
    }

    fn part_two(&self, input: &str) -> String {
        let min_heat_loss = get_min_heat_loss(input, 4, 10);
        println!("{min_heat_loss}");
        format!("{min_heat_loss}")
    }
}

fn get_min_heat_loss(input: &str, min_steps: u32, max_steps: u32) -> u32 {
    let tiles: Vec<Vec<u32>> = filter_input_lines(input)
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let height = tiles.len();
    let width = tiles[0].len();

    let init_state = State {
        row: 0,
        col: 0,
        heat_loss: 0,
        straight_steps: 0,
        direction: Direction::Right,
    };

    let mut to_visit: BinaryHeap<State> = vec![init_state].into_iter().collect();
    let mut heat_losses: HashMap<(usize, usize, Direction), (u32, u32)> = HashMap::new();
    let mut prev: HashMap<State, State> = HashMap::new();

    while let Some(state) = to_visit.pop() {
        let neighbours = state.get_neighbours(&tiles, width, height, min_steps, max_steps);
        for neighbour in neighbours {
            let pos = (neighbour.row, neighbour.col, neighbour.direction);
            if let Some((heat_loss, _)) = heat_losses.get(&pos) {
                if *heat_loss <= neighbour.heat_loss {
                    continue;
                }
            }
            heat_losses.insert(pos, (neighbour.heat_loss, neighbour.straight_steps));
            prev.insert(neighbour, state);
            to_visit.push(neighbour);
        }
    }

    let final_state = heat_losses
        .into_iter()
        .filter_map(|((row, col, direction), (heat_loss, straight_steps))| {
            if (row == (height - 1)) && (col == (width - 1)) {
                Some(State {
                    row,
                    col,
                    heat_loss,
                    straight_steps,
                    direction,
                })
            } else {
                None
            }
        })
        .max()
        .unwrap();

    let mut path = vec![final_state.clone()];
    let mut state = final_state;
    while let Some(s) = prev.get(&state) {
        state = *s;
        path.push(s.clone());
    }

    print_path(&tiles, path);

    final_state.heat_loss
}

fn print_path(tiles: &Vec<Vec<u32>>, path: Vec<State>) {
    let mut tiles: Vec<String> = tiles.iter().map(|row| row.iter().map(|tile| format!("{tile}")).collect()).collect();
    for State {
        row,
        col,
        heat_loss: _,
        straight_steps,
        direction,
    } in path
    {
        let straight_steps = straight_steps as usize;
        match direction {
            Direction::Up => {
                for i in row..(row + straight_steps) {
                    tiles[i].replace_range(col..(col + 1), "^");
                }
            }
            Direction::Down => {
                for i in (row - straight_steps + 1)..=row {
                    tiles[i].replace_range(col..(col + 1), "v");
                }
            }
            Direction::Left => {
                for j in col..=(col + straight_steps) {
                    tiles[row].replace_range(j..(j + 1), "<");
                }
            }
            Direction::Right => {
                for j in (col - straight_steps + 1)..=col {
                    tiles[row].replace_range(j..(j + 1), ">");
                }
            }
        }
    }
    for line in tiles {
        println!("{line}");
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533";
        let value = Day.part_one(input);
        assert_eq!(value, "102");
    }

    #[test]
    fn test_part_two() {
        let input = "
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533";
        let value = Day.part_two(input);
        assert_eq!(value, "94");

        let input = "
        111111111111
        999999999991
        999999999991
        999999999991
        999999999991";
        let value = Day.part_two(input);
        assert_eq!(value, "71");
    }
}
