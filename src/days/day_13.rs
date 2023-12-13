
use regex::Regex;

use super::Problem;

#[derive(PartialEq)]
enum Mirror {
    Vertical(usize),
    Horizontal(usize)
}

pub struct Day;

impl Problem for Day {

    fn part_one(&self, input: &str) -> String {

        let separator = Regex::new(r"\n[ ]*\n").unwrap();
        let patterns: Vec<Vec<&str>> = separator.split(input).map(|s| s.split_ascii_whitespace().collect()).collect();

        let mut vertical_mirrors = Vec::new();
        let mut horizontal_mirrors = Vec::new();

        for pattern in patterns {
            match find_mirrors(&pattern)[0] {
                Mirror::Horizontal(row) => horizontal_mirrors.push(row),
                Mirror::Vertical(col) => vertical_mirrors.push(col),
            }
        }

        let total_number: usize = vertical_mirrors.iter().sum::<usize>() + 100 * horizontal_mirrors.iter().sum::<usize>();

        println!("{total_number}");
        format!("{total_number}")
    }

    fn part_two(&self, input: &str) -> String {
        let separator = Regex::new(r"\n[ ]*\n").unwrap();
        let patterns: Vec<Vec<&str>> = separator.split(input).map(|s| s.split_ascii_whitespace().collect()).collect();

        let mut vertical_mirrors = Vec::new();
        let mut horizontal_mirrors = Vec::new();

        'outer: for pattern in patterns {

            let original_mirror = find_mirrors(&pattern).pop().unwrap();

            for i in 0..pattern.len() {
                for j in 0..pattern[0].len() {
                    let mut new_pattern = pattern.clone();
                    let mut new_line = pattern[i].to_owned();
                    match new_line.chars().nth(j).unwrap() {
                        '#' => new_line.replace_range(j..(j+1), "."),
                        '.' => new_line.replace_range(j..(j+1), "#"),
                        _ => panic!()
                    }
                    new_pattern[i] = &new_line;
                    let new_mirrors = find_mirrors(&new_pattern);
                    for mirror in new_mirrors {
                        if mirror != original_mirror {
                            match mirror {
                                Mirror::Horizontal(row) => horizontal_mirrors.push(row),
                                Mirror::Vertical(col) => vertical_mirrors.push(col),
                            }
                            continue 'outer;
                        }
                    }
                }
            }

        }

        let total_number: usize = vertical_mirrors.iter().sum::<usize>() + 100 * horizontal_mirrors.iter().sum::<usize>();

        println!("{total_number}");
        format!("{total_number}")
    }

}


fn find_mirrors(pattern: &Vec<&str>) -> Vec<Mirror> {

    let mut mirrors = Vec::new();

    // Check vertical mirror
    for split_col in 1..(pattern[0].len()) {
        let is_mirrored = pattern.iter().all(|line| {
            let (left, right) = line.split_at(split_col);
            left.chars().rev().zip(right.chars()).all(|(l, r)| l == r)
        });
        if is_mirrored {
            mirrors.push(Mirror::Vertical(split_col));
        }
    }

    // Check horizontal mirror
    for split_row in 1..(pattern.len()) {
        let is_mirrored = (0..pattern[0].len()).all(|col| {
            let col_chars: String = pattern.iter().map(|line| line.chars().nth(col).unwrap()).collect();
            let (top, bottom) = col_chars.split_at(split_row);
            top.chars().rev().zip(bottom.chars()).all(|(l, r)| l == r)
        });
        if is_mirrored {
            mirrors.push(Mirror::Horizontal(split_row));
        }
    }

    mirrors

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#";
        let value = Day.part_one(input);
        assert_eq!(value, "405");
    }

    #[test]
    fn test_part_two() {
        let input = "
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#";
        let value = Day.part_two(input);
        assert_eq!(value, "400");
    }

}
