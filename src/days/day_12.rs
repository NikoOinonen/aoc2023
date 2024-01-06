use std::collections::HashMap;

use aoc2023::filter_input_lines;

use super::Problem;

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let lines = filter_input_lines(input);

        let mut num_arrangements = 0;
        for line in lines {
            let (records, counts) = line.split_once(' ').unwrap();
            let target_counts: Vec<usize> = counts.split(',').map(|s| s.parse().unwrap()).collect();
            num_arrangements += find_num_arrangements(records, target_counts, &mut HashMap::new());
        }

        println!("{num_arrangements}");
        format!("{num_arrangements}")
    }

    fn part_two(&self, input: &str) -> String {
        let lines = filter_input_lines(input);

        let mut num_arrangements = 0;
        for line in lines {
            let (records, counts) = line.split_once(' ').unwrap();
            let target_counts: Vec<usize> = counts.split(',').map(|s| s.parse().unwrap()).collect();

            let records = vec![records; 5].join("?");
            let target_counts: Vec<usize> = vec![target_counts; 5].concat().into_iter().collect();

            num_arrangements += find_num_arrangements(&records, target_counts, &mut HashMap::new());
        }

        println!("{num_arrangements}");
        format!("{num_arrangements}")
    }
}

fn find_num_arrangements(records: &str, target_counts: Vec<usize>, counts: &mut HashMap<(String, Vec<usize>), usize>) -> usize {
    if records.len() == 0 {
        if target_counts.len() == 0 {
            return 1;
        } else {
            return 0;
        }
    }
    if target_counts.len() == 0 {
        if records.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }

    let state = (records.to_owned(), target_counts.clone());
    if let Some(c) = counts.get(&state) {
        return *c;
    }

    let mut count = 0;

    let next_record = records.chars().nth(0).unwrap();
    if next_record == '.' || next_record == '?' {
        count += find_num_arrangements(&records[1..], target_counts.clone(), counts);
    }
    if next_record == '#' || next_record == '?' {
        let current_target = target_counts[0];
        let rem_len = records.len();
        if (current_target <= rem_len)
            && records[..current_target].chars().all(|c| c != '.')
            && (current_target == rem_len || records.chars().nth(current_target).unwrap() != '#')
        {
            let new_target_counts = target_counts.into_iter().skip(1).collect();
            let new_pos = rem_len.min(current_target + 1);
            count += find_num_arrangements(&records[new_pos..], new_target_counts, counts);
        }
    }

    counts.insert(state, count);

    count
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";
        let value = Day.part_one(input);
        assert_eq!(value, "21");
    }

    #[test]
    fn test_part_two() {
        let input = "
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";
        let value = Day.part_two(input);
        assert_eq!(value, "525152");
    }
}
