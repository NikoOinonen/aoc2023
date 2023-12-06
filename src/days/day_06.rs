use std::iter::zip;

use aoc2023::filter_input_lines;

use super::Problem;

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let input_lines = filter_input_lines(input);
        let mut parsed: Vec<Vec<u32>> = input_lines
            .iter()
            .map(|line| {
                line.split_once(':')
                    .unwrap()
                    .1
                    .trim()
                    .split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .collect();
        let distances = parsed.pop().unwrap();
        let times = parsed.pop().unwrap();

        let mut number_of_wins: Vec<u32> = Vec::new();
        for (max_time, max_distance) in zip(times, distances) {
            let num_wins = (0..=max_time)
                .filter(|hold_time| {
                    let distance = hold_time * (max_time - hold_time);
                    distance > max_distance
                })
                .count();
            number_of_wins.push(num_wins as u32);
        }

        let score: u32 = number_of_wins.iter().product();

        println!("{score}");
        format!("{score}")
    }

    fn part_two(&self, input: &str) -> String {
        let input_lines = filter_input_lines(input);
        let mut parsed: Vec<u64> = input_lines
            .iter()
            .map(|line| {
                line.split_once(':')
                    .unwrap()
                    .1
                    .trim()
                    .replace(" ", "")
                    .parse().unwrap()
            })
            .collect();
        let max_distance = parsed.pop().unwrap();
        let max_time = parsed.pop().unwrap();

        let temp = ((max_time as f64).powf(2.0) - 4.0 * max_distance as f64).sqrt();
        let min_hold_time = (0.5 * (max_time as f64) - 0.5 * temp).ceil() as u64;
        let max_hold_time = (0.5 * (max_time as f64) + 0.5 * temp).floor() as u64;
        let max_hold_time = max_hold_time.min(max_time);
        let number_of_wins = max_hold_time - min_hold_time + 1;

        println!("{number_of_wins}");
        format!("{number_of_wins}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
            Time:      7  15   30
            Distance:  9  40  200";
        let value = Day.part_one(input);
        assert_eq!(value, "288");
    }

    #[test]
    fn test_part_two() {
        let input = "
            Time:      7  15   30
            Distance:  9  40  200";
        let value = Day.part_two(input);
        assert_eq!(value, "71503");
    }
}
