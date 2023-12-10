use aoc2023::filter_input_lines;

use super::Problem;

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let input = filter_input_lines(input);

        let mut total_value = 0;
        for line in input {
            let mut values: Vec<i32> = line.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
            let mut differences = Vec::new();
            differences.push(values.clone());
            while !values.iter().all(|v| *v == 0) {
                values = values.windows(2).map(|s| s[1] - s[0]).collect();
                differences.push(values.clone());
            }

            let mut value = 0;
            for values in differences.iter().rev() {
                value += values.last().unwrap();
            }

            total_value += value;
        }

        println!("{total_value}");
        format!("{total_value}")
    }

    fn part_two(&self, input: &str) -> String {
        let input = filter_input_lines(input);

        let mut total_value = 0;
        for line in input {
            let mut values: Vec<i32> = line.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
            let mut differences = Vec::new();
            differences.push(values.clone());
            while !values.iter().all(|v| *v == 0) {
                values = values.windows(2).map(|s| s[1] - s[0]).collect();
                differences.push(values.clone());
            }

            let mut value = 0;
            for values in differences.iter().rev() {
                value = values[0] - value;
            }

            total_value += value;
        }

        println!("{total_value}");
        format!("{total_value}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        let value = Day.part_one(input);
        assert_eq!(value, "114");
    }

    #[test]
    fn test_part_two() {
        let input = "
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        let value = Day.part_two(input);
        assert_eq!(value, "2");
    }
}
