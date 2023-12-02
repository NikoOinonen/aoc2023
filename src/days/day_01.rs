
use super::Problem;


const NUM_STRS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "1", "2", "3", "4", "5", "6", "7", "8", "9"
];

pub struct Day;

impl Problem for Day {

    fn part_one(&self, input: &str) -> String {
        let mut value_sum = 0;
        for line in input.lines() {
            let line = line.trim();
            let nums: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
            let first_digit = nums.first().unwrap();
            let last_digit = nums.last().unwrap();
            let value = format!("{first_digit}{last_digit}").parse::<i32>().unwrap();
            value_sum += value;
        }
        println!("Value sum: {value_sum}");
        format!("{value_sum}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut value_sum = 0;
        for line in input.lines() {
            let line = line.trim();
            let (first_digit, last_digit) = find_first_last(line);
            let value = format!("{first_digit}{last_digit}").parse::<i32>().unwrap();
            value_sum += value;
        }
        println!("Value sum: {value_sum}");
        format!("{value_sum}")
    }

}

fn find_first_last(line: &str) -> (i32, i32) {
    let mut nums_positions: Vec<(usize, i32)> = Vec::new();
    for search_str in NUM_STRS {
        let num = str_into_num(search_str);
        for m in line.match_indices(search_str){
            nums_positions.push((m.0, num));
        }
    }
    nums_positions.sort();
    let first_digit = nums_positions[0].1;
    let last_digit = nums_positions.pop().unwrap().1;
    (first_digit, last_digit)
}

fn str_into_num(s: &str) -> i32 {
    match s {
        "one"   => 1,
        "two"   => 2,
        "three" => 3,
        "four"  => 4,
        "five"  => 5,
        "six"   => 6,
        "seven" => 7,
        "eight" => 8,
        "nine"  => 9,
        "1"     => 1,
        "2"     => 2,
        "3"     => 3,
        "4"     => 4,
        "5"     => 5,
        "6"     => 6,
        "7"     => 7,
        "8"     => 8,
        "9"     => 9,
        _       => panic!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::seq::SliceRandom;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_part_one() {
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        let value = Day.part_one(input);
        assert_eq!(value, "142");
    }

    #[test]
    fn test_part_two() {
        let input = "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";
        let value = Day.part_two(input);
        assert_eq!(value, "281");
    }

    #[test]
    fn test_find_first_last_random() {
        let mut rng = StdRng::seed_from_u64(0);
        for _ in 0..100 {
            let mut line = String::new();
            let mut nums = Vec::new();
            for _ in 0..5 {
                let s = NUM_STRS.choose(&mut rng).unwrap().to_owned();
                let num = str_into_num(s);
                line.push_str(s);
                nums.push(num);
            }
            let (first_digit, last_digit) = find_first_last(&line);
            assert_eq!(first_digit, nums[0]);
            assert_eq!(last_digit, nums[4]);
        }
    }

}
