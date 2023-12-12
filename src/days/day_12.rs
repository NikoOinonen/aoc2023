use std::collections::{HashSet, VecDeque};

use aoc2023::filter_input_lines;
use itertools::Itertools;

use super::Problem;

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let lines = filter_input_lines(input);

        let mut num_arrangements = 0;
        for line in lines {
            let (records, counts) = line.split_once(' ').unwrap();
            let target_counts: VecDeque<usize> = counts.split(',').map(|s| s.parse().unwrap()).collect();
            let total_len = records.len();

            let mut visited = HashSet::new();
            let mut to_visit = vec![(records.to_owned(), target_counts.clone(), 0)];

            while let Some(state) = to_visit.pop() {
                visited.insert(state.clone());
                let (mut springs, mut rem_counts, pos) = state;

                if let Some(count) = rem_counts.pop_front() {
                    let rem_len: usize = if rem_counts.len() > 0 {
                        rem_counts.iter().sum::<usize>() + rem_counts.len() - 1
                    } else {
                        0
                    };
                    let max_pos = total_len - rem_len - count;
                    let insert_str = vec!['#'; count].iter().collect::<String>();
                    for i in pos..=max_pos {
                        let valid_next_char = match springs.chars().nth(i + count) {
                            Some(c) => c != '#',
                            None => true,
                        };
                        if valid_next_char && springs[i..(i + count)].chars().all(|c| c == '?' || c == '#') {
                            let mut new_springs = springs.clone();
                            new_springs.replace_range(i..(i + count), &insert_str);
                            if springs.len() > (i + count) {
                                new_springs.replace_range((i + count)..(i + count + 1), ".");
                            }
                            let new_state = (new_springs, rem_counts.clone(), i + count + 1);
                            if !visited.contains(&new_state) {
                                to_visit.push(new_state);
                            }
                        }
                        if springs.chars().nth(i).unwrap() == '?' {
                            springs.replace_range(i..i + 1, ".");
                        }
                    }
                } else {
                    let final_springs = springs.replace('?', ".");
                    let final_counts: Vec<usize> = final_springs
                        .split('.')
                        .filter_map(|s| if s.len() == 0 { None } else { Some(s.len()) })
                        .collect();
                    if final_counts.len() != target_counts.len() {
                        continue;
                    }
                    if final_counts.iter().zip_eq(&target_counts).all(|(a, b)| a == b) {
                        num_arrangements += 1;
                    }
                }
            }
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
            let target_counts: VecDeque<usize> = vec![target_counts; 5].concat().into_iter().collect();
            let total_len = records.len();

            println!("{records}");

            let mut visited = HashSet::new();
            let mut to_visit = vec![(records.to_owned(), target_counts.clone(), 0)];

            while let Some(state) = to_visit.pop() {
                visited.insert(state.clone());
                let (mut springs, mut rem_counts, pos) = state;

                if let Some(count) = rem_counts.pop_front() {
                    let rem_len: usize = if rem_counts.len() > 0 {
                        rem_counts.iter().sum::<usize>() + rem_counts.len() - 1
                    } else {
                        0
                    };
                    let max_pos = total_len - rem_len - count;
                    let insert_str = vec!['#'; count].iter().collect::<String>();
                    for i in pos..=max_pos {
                        let valid_next_char = match springs.chars().nth(i + count) {
                            Some(c) => c != '#',
                            None => true,
                        };
                        if valid_next_char && springs[i..(i + count)].chars().all(|c| c == '?' || c == '#') {
                            let mut new_springs = springs.clone();
                            new_springs.replace_range(i..(i + count), &insert_str);
                            if springs.len() > (i + count) {
                                new_springs.replace_range((i + count)..(i + count + 1), ".");
                            }
                            let new_state = (new_springs, rem_counts.clone(), i + count + 1);
                            if !visited.contains(&new_state) {
                                to_visit.push(new_state);
                            }
                        }
                        if springs.chars().nth(i).unwrap() == '?' {
                            springs.replace_range(i..i + 1, ".");
                        }
                    }
                } else {
                    let final_springs = springs.replace('?', ".");
                    let final_counts: Vec<usize> = final_springs
                        .split('.')
                        .filter_map(|s| if s.len() == 0 { None } else { Some(s.len()) })
                        .collect();
                    if final_counts.len() != target_counts.len() {
                        continue;
                    }
                    if final_counts.iter().zip_eq(&target_counts).all(|(a, b)| a == b) {
                        num_arrangements += 1;
                    }
                }
            }
        }

        println!("{num_arrangements}");
        format!("{num_arrangements}")
    }
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
