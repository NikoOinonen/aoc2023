
use super::Problem;

#[derive(Debug)]
struct Number {
    val: usize,
    pos: Vec<(usize, usize)>
}

struct Part {
    symbol: char,
    pos: (usize, usize)
}

pub struct Day;

impl Problem for Day {

    fn part_one(&self, input: &str) -> String {

        let (numbers, parts) = find_numbers_parts(input);

        let mut vals_sum = 0;
        for num in numbers {
            let mut is_close = false;
            for part in parts.iter() {
                if num_part_distance(&num, &part) <= 1 {
                    is_close = true;
                    break;
                }
            }
            if is_close {
                vals_sum += num.val;
            }
        }
        println!("{vals_sum}");
        format!("{vals_sum}")
    }

    fn part_two(&self, input: &str) -> String {
        
        let (numbers, parts) = find_numbers_parts(input);

        let mut vals_sum = 0;
        for part in parts {
            if part.symbol != '*' {
                continue;
            }
            let mut close_nums = Vec::new();
            for num in numbers.iter() {
                if num_part_distance(&num, &part) <= 1 {
                    close_nums.push(num.clone());
                }
            }
            if close_nums.len() == 2 {
                vals_sum += close_nums[0].val * close_nums[1].val;
            }
        }
        println!("{vals_sum}");
        format!("{vals_sum}")
    }

}

fn find_numbers_parts(input: &str) -> (Vec<Number>, Vec<Part>) {

    let mut parts: Vec<Part> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();
    let mut num_char_buffer = String::new();
    let mut num_pos_buffer = Vec::new();

    for (row, line) in input.trim().lines().enumerate() {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        for (col, c) in line.chars().enumerate() {
            match c {
                x if x.is_numeric() => {
                    num_char_buffer.push(x);
                    num_pos_buffer.push((row, col))
                },
                x => {
                    if num_char_buffer.len() > 0 {
                        let val: usize = num_char_buffer.parse().unwrap();
                        numbers.push( Number{ val, pos: num_pos_buffer.clone() } );
                        num_char_buffer.clear();
                        num_pos_buffer.clear();
                    }
                    match x {
                        '.' => (),
                        _ => parts.push( Part { symbol: x, pos: (row, col) } )
                    }
                    
                }
            }
        }
    }

    (numbers, parts)

}

fn num_part_distance(num: &Number, part: &Part) -> usize {
    let sym_pos = part.pos;
    let mut min_row_dist = usize::MAX;
    let mut min_col_dist = usize::MAX;
    for num_pos in num.pos.iter() {
        let row_dist = sym_pos.0.abs_diff(num_pos.0);
        let col_dist = sym_pos.1.abs_diff(num_pos.1);
        min_row_dist = min_row_dist.min(row_dist);
        min_col_dist = min_col_dist.min(col_dist);
    }
    min_row_dist.max(min_col_dist)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        let value = Day.part_one(input);
        assert_eq!(value, "4361");
    }

    #[test]
    fn test_part_two() {
        let input = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        let value = Day.part_two(input);
        assert_eq!(value, "467835");
    }

}
