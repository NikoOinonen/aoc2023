use std::fs;


pub fn read_input(day_num: i32) -> String {
    let input_path = format!("./inputs/day_{day_num:02}.txt");
    fs::read_to_string(input_path).expect(&format!("No input file for day {day_num}"))
}
