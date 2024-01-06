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
        8 => Box::new(days::day_08::Day),
        9 => Box::new(days::day_09::Day),
        10 => Box::new(days::day_10::Day),
        11 => Box::new(days::day_11::Day),
        12 => Box::new(days::day_12::Day),
        13 => Box::new(days::day_13::Day),
        14 => Box::new(days::day_14::Day),
        15 => Box::new(days::day_15::Day),
        16 => Box::new(days::day_16::Day),
        17 => Box::new(days::day_17::Day),
        18 => Box::new(days::day_18::Day),
        19 => Box::new(days::day_19::Day),
        20 => Box::new(days::day_20::Day),
        21 => Box::new(days::day_21::Day),
        22 => Box::new(days::day_22::Day),
        23 => Box::new(days::day_23::Day),
        24 => Box::new(days::day_24::Day),
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
