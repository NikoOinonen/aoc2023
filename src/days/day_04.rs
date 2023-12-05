
use super::Problem;
use aoc2023::filter_input_lines;

pub struct Day;

impl Problem for Day {

    fn part_one(&self, input: &str) -> String {
        let mut total_points = 0;
        let input_lines = filter_input_lines(input);
        for line in input_lines {
            let line = line.trim();
            if line.len() == 0 {
                continue;
            }
            let card_nums: Vec<&str> = line.split(":").last().unwrap().split("|").collect();
            let winning_nums: Vec<usize> = card_nums[0].split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
            let player_nums: Vec<usize> = card_nums[1].split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
            let mut points = 0;
            for num in player_nums {
                if winning_nums.contains(&num) {
                    if points == 0 {
                        points = 1;
                    }
                    else {
                        points = points * 2;
                    }
                }
            }
            total_points += points;
        }
        println!("{total_points}");
        format!("{total_points}")
    }

    fn part_two(&self, input: &str) -> String {

        let input_lines = filter_input_lines(input);
        let num_cards = input_lines.len();
        let mut card_counts = vec![0; num_cards];

        for line in input_lines {

            let (card_num, numbers) = line.split_once(":").unwrap();
            let card_num: usize = card_num.trim().split_ascii_whitespace().last().unwrap().parse().unwrap();
            let (winning_nums, player_nums) = numbers.split_once("|").unwrap();
            let player_nums: Vec<usize> = player_nums.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
            let winning_nums: Vec<usize> = winning_nums.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
            let matching_nums = player_nums.iter().filter(|num| winning_nums.contains(&num)).collect::<Vec<&usize>>().len();

            card_counts[card_num - 1] += 1;
            for i in 0..matching_nums {
                card_counts[card_num + i] += card_counts[card_num - 1];
            }

        }

        let total_card_count: usize = card_counts.iter().sum();

        println!("{total_card_count}");
        format!("{total_card_count}")
        
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let value = Day.part_one(input);
        assert_eq!(value, "13");
    }

    #[test]
    fn test_part_two() {
        let input = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let value = Day.part_two(input);
        assert_eq!(value, "30");
    }

}
