use std::fs;

pub fn read_input(day_num: i32) -> String {
    let input_path = format!("./inputs/day_{day_num:02}.txt");
    fs::read_to_string(input_path).expect(&format!("No input file for day {day_num}"))
}

pub fn filter_input_lines(input: &str) -> Vec<&str> {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.len() == 0 {
                None
            } else {
                Some(line)
            }
        })
        .collect::<Vec<&str>>()
}
