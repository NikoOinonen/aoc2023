use std::collections::HashMap;

use aoc2023::filter_input_lines;

use super::Problem;

const MAX_STEPS: u32 = 1_000_000;
// const MAX_STEPS: u32 = 20;

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let input = filter_input_lines(input);
        let instructions = input.iter().next().unwrap().chars().cycle();

        let mut map = HashMap::new();
        for line in input.iter().skip(1) {
            let (origin, destinations) = line.split_once('=').unwrap();
            let (dest_left, dest_right) = destinations.trim().split_once(',').unwrap();
            let origin = origin.trim();
            let dest_left = &dest_left[1..4];
            let dest_right = &dest_right[1..4];
            map.insert(origin, (dest_left, dest_right));
        }

        let mut num_steps = 0;
        let mut location = "AAA";
        for instruction in instructions {
            num_steps += 1;
            if num_steps >= MAX_STEPS {
                break;
            }

            let (dest_left, dest_right) = map.get(location).unwrap();
            match instruction {
                'L' => location = dest_left,
                'R' => location = dest_right,
                _ => panic!(),
            }

            if location == "ZZZ" {
                break;
            }
        }

        println!("{num_steps}");
        format!("{num_steps}")
    }

    fn part_two(&self, input: &str) -> String {
        let input = filter_input_lines(input);
        let instructions = input.iter().next().unwrap();
        let num_instructions = instructions.len();

        let mut map = HashMap::new();
        let mut start_locations = Vec::new();
        for line in input.iter().skip(1) {
            let (origin, destinations) = line.split_once('=').unwrap();
            let (dest_left, dest_right) = destinations.trim().split_once(',').unwrap();
            let origin = origin.trim();
            let dest_left = &dest_left[1..4];
            let dest_right = &dest_right[1..4];
            if origin.chars().nth(2).unwrap() == 'A' {
                start_locations.push(origin.clone());
            }
            map.insert(origin, (dest_left, dest_right));
        }

        let mut cycles = Vec::new();
        for start_location in start_locations {
            let mut num_steps = 0;
            let mut visited = Vec::new();
            let mut location = start_location;
            visited.push(location);

            'outer: for instruction in instructions.chars().cycle() {
                num_steps += 1;
                if num_steps >= MAX_STEPS {
                    break;
                }

                let (dest_left, dest_right) = map.get(location).unwrap();
                match instruction {
                    'L' => location = dest_left,
                    'R' => location = dest_right,
                    _ => panic!(),
                }

                visited.push(location);

                let mut pos: i32 = num_steps as i32 - num_instructions as i32;
                while pos >= 0 {
                    if visited[pos as usize] == location {
                        cycles.push(num_steps as u64 - pos as u64);
                        break 'outer;
                    }
                    pos -= num_instructions as i32;
                }
            }
        }

        let mut align_step = num_instructions as u64;
        for cycle_len in cycles {
            let multiple = cycle_len / num_instructions as u64;
            align_step *= multiple as u64;
        }

        println!("{align_step}");
        format!("{align_step}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";
        let value = Day.part_one(input);
        assert_eq!(value, "6");
    }

    #[test]
    fn test_part_two() {
        let input = "
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";
        let value = Day.part_two(input);
        assert_eq!(value, "6");
    }
}
