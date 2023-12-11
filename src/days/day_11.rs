use aoc2023::filter_input_lines;

use super::Problem;

// const EXPANSION_FACTOR: usize = 10;
const EXPANSION_FACTOR: usize = 1_000_000;

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let lines = filter_input_lines(input);

        let mut galaxy_coordinates = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxy_coordinates.push((i, j));
                }
            }
        }

        let height = lines.len();
        let width = lines[0].len();

        let empty_rows: Vec<usize> = (0..height)
            .filter(|row| galaxy_coordinates.iter().all(|coord| coord.0 != *row))
            .collect();
        for (i, row) in empty_rows.iter().enumerate() {
            let row = row + i;
            for coord in galaxy_coordinates.iter_mut() {
                if coord.0 > row {
                    coord.0 += 1;
                }
            }
        }


        let empty_cols: Vec<usize> = (0..width)
            .filter(|col| galaxy_coordinates.iter().all(|coord| coord.1 != *col))
            .collect();
        for (j, col) in empty_cols.iter().enumerate() {
            let col = col + j;
            for coord in galaxy_coordinates.iter_mut() {
                if coord.1 > col {
                    coord.1 += 1;
                }
            }
        }

        let distance_sum: usize = galaxy_coordinates.iter().map(|coord1| {
            galaxy_coordinates.iter().map(|coord2| {
                coord1.0.abs_diff(coord2.0) + coord1.1.abs_diff(coord2.1)
            }).sum::<usize>()
        }).sum();
        let distance_sum = distance_sum / 2;

        println!("{distance_sum}");
        format!("{distance_sum}")
    }

    fn part_two(&self, input: &str) -> String {
        let lines = filter_input_lines(input);

        let mut galaxy_coordinates = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxy_coordinates.push((i, j));
                }
            }
        }

        let height = lines.len();
        let width = lines[0].len();

        let empty_rows: Vec<usize> = (0..height)
            .filter(|row| galaxy_coordinates.iter().all(|coord| coord.0 != *row))
            .collect();
        let empty_cols: Vec<usize> = (0..width)
            .filter(|col| galaxy_coordinates.iter().all(|coord| coord.1 != *col))
            .collect();

        let distance_sum: usize = galaxy_coordinates.iter().map(|coord1| {
            galaxy_coordinates.iter().map(|coord2| {
                let (row1, row2) = if coord1.0 > coord2.0 {(coord2.0, coord1.0)} else {(coord1.0, coord2.0)};
                let (col1, col2) = if coord1.1 > coord2.1 {(coord2.1, coord1.1)} else {(coord1.1, coord2.1)};
                let num_empty_rows = empty_rows.iter().filter(|row| **row > row1 && **row < row2).count();
                let num_empty_cols = empty_cols.iter().filter(|col| **col > col1 && **col < col2).count();
                row2 - row1 + col2 - col1 + (EXPANSION_FACTOR - 1) * (num_empty_rows + num_empty_cols)
            }).sum::<usize>()
        }).sum();
        let distance_sum = distance_sum / 2;

        println!("{distance_sum}");
        format!("{distance_sum}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";
        let value = Day.part_one(input);
        assert_eq!(value, "374");
    }

    // #[test]
    // fn test_part_two() {
    //     let input = "
    //     ...#......
    //     .......#..
    //     #.........
    //     ..........
    //     ......#...
    //     .#........
    //     .........#
    //     ..........
    //     .......#..
    //     #...#.....";
    //     let value = Day.part_two(input);
    //     assert_eq!(value, "1030");
    // }
}
