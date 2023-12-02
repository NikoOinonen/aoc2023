
use std::str::FromStr;

use super::Problem;

struct GameSet {
    red: usize,
    green: usize,
    blue: usize
}

impl FromStr for GameSet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for cubes in s.split(",") {
            let split: Vec<&str> = cubes.trim().split(" ").collect();
            let num = match split[0].parse::<usize>() {
                Ok(n) => n,
                Err(_) => return Err(())
            };
            let color = split[1];
            match color {
                "red"   => red = num,
                "green" => green = num,
                "blue"  => blue = num,
                _ => return Err(())
            }
        }
        Ok(GameSet { red, green, blue })
    }
}

pub struct Day;

impl Problem for Day {

    fn part_one(&self, input: &str) -> String {
        let mut id_sum = 0;
        let mut id_game = 0;
        for line in input.trim().lines() {
            let line = line.trim();
            if line.len() == 0 {
                continue;
            }
            id_game += 1;
            let (max_red, max_green, max_blue) = find_max_nums(line);
            if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
                id_sum += id_game;
            }
            
        }
        println!("{id_sum}");
        format!("{id_sum}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut power_total = 0;
        for line in input.trim().lines() {
            let line = line.trim();
            if line.len() == 0 {
                continue;
            }
            let (max_red, max_green, max_blue) = find_max_nums(line);
            let power = max_red * max_green * max_blue;
            power_total += power;
        }
        println!("{power_total}");
        format!("{power_total}")
    }

}

fn find_max_nums(s: &str) -> (usize, usize, usize) {
    let game_sets: Vec<GameSet> = s
        .split(":").last().unwrap()
        .split(";").into_iter().map(|s| s.parse().unwrap())
        .collect();
    let max_red = game_sets.iter().map(|g| g.red).max().unwrap();
    let max_green = game_sets.iter().map(|g| g.green).max().unwrap();
    let max_blue = game_sets.iter().map(|g| g.blue).max().unwrap();
    (max_red, max_green, max_blue)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let value = Day.part_one(input);
        assert_eq!(value, "8");
    }

    #[test]
    fn test_part_two() {
        let input = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let value = Day.part_two(input);
        assert_eq!(value, "2286");
    }

}
