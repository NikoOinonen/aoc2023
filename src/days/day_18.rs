
use aoc2023::filter_input_lines;
use regex::Regex;

use super::Problem;

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let lines = filter_input_lines(input);
        let re = Regex::new(r"(?<dir>.)\s*(?<amount>\d+)\s*\(#[a-fA-F0-9]{6}\)").unwrap();
        let mut pos = (0, 0);
        let mut area = 0;
        let mut num_edge_squares = 0;
        for line in lines {
            let cap = re.captures(line).unwrap();
            let dir = &cap["dir"];
            let amount: i32 = cap["amount"].parse().unwrap();
            let offset = match dir {
                "R" => (0, amount),
                "L" => (0, -amount),
                "U" => (-amount, 0),
                "D" => (amount, 0),
                _ => panic!(),
            };
            pos = (pos.0 + offset.0, pos.1 + offset.1);
            area += pos.1 * offset.0;
            num_edge_squares += amount;
        }

        area = area.abs() + num_edge_squares / 2 + 1;

        println!("{area}");
        format!("{area}")
    }

    fn part_two(&self, input: &str) -> String {
        let lines = filter_input_lines(input);
        let re = Regex::new(r".\s*\d+\s*\(#(?<code>[a-fA-F0-9]{6})\)").unwrap();
        let mut pos = (0, 0);
        let mut area: i64 = 0;
        let mut num_edge_squares = 0;
        for line in lines {
            let cap = re.captures(line).unwrap();
            let code: &str = &cap["code"];
            let amount = i64::from_str_radix(&code[..5], 16).unwrap();
            let dir = &code[5..6];
            let offset = match dir {
                "0" => (0, amount),
                "1" => (amount, 0),
                "2" => (0, -amount),
                "3" => (-amount, 0),
                _ => panic!(),
            };
            pos = (pos.0 + offset.0, pos.1 + offset.1);
            area += pos.1 * offset.0;
            num_edge_squares += amount;
        }

        area = area.abs() + num_edge_squares / 2 + 1;

        println!("{area}");
        format!("{area}")
    }
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
        let value = Day.part_two(input);
        assert_eq!(value, "952408144115");
    }
}
