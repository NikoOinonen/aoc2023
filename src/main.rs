use std::env;

use days::Problem;
use aoc2023::read_input;

mod days;

fn parse_args() -> (i32, Vec<i32>) {

    let mut args = env::args();
    
    args.next();
    let day_num = match args.next() {
        None => panic!("No day number given"),
        Some(a) => {
            let num = match a.parse::<i32>() {
                Ok(n) => n,
                Err(_) => panic!("Invalid day number {a}")
            };
            num
        }
    };

    let parts = match args.next() {
        None => vec![1, 2],
        Some(a) => {
            let num = match a.parse::<i32>() {
                Ok(n) => n,
                Err(_) => panic!("Invalid part number {a}")
            };
            if num <= 0 || num > 2 {
                panic!("Invalid part number {num}");
            }
            vec![num]
        }
    };

    return (day_num, parts)

}

fn get_day(day_num: i32) -> Box<dyn Problem> {
    match day_num {
        1 => Box::new(days::day_01::Day),
        2 => Box::new(days::day_02::Day),
        3 => Box::new(days::day_03::Day),
        4 => Box::new(days::day_04::Day),
        5 => Box::new(days::day_05::Day),
        6 => Box::new(days::day_06::Day),
        7 => Box::new(days::day_07::Day),
        _ => panic!("Day {day_num} not implemented")
    }
}

fn run(day_num: i32, parts: Vec<i32>) {
    let day = get_day(day_num);
    for part in parts {
        let input = read_input(day_num);
        match part {
            1 => day.part_one(&input),
            2 => day.part_two(&input),
            _ => panic!("Invalid part")
        };
    }
}

fn main() {
    let (day_num, parts) = parse_args();
    run(day_num, parts);
}
