use aoc2023::filter_input_lines;

use super::Problem;

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let input = filter_input_lines(input);
        let seeds: Vec<u64> = input[0]
            .split(":")
            .last()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let (
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location
        ) = get_maps(input);

        let mut min_location = u64::MAX;
        for seed in seeds {
            let soil = map_number(seed, &seed_to_soil);
            let fertilizer = map_number(soil, &soil_to_fertilizer);
            let water = map_number(fertilizer, &fertilizer_to_water);
            let light = map_number(water, &water_to_light);
            let temperature = map_number(light, &light_to_temperature);
            let humidity = map_number(temperature, &temperature_to_humidity);
            let location = map_number(humidity, &humidity_to_location);
            min_location = min_location.min(location);
        }

        println!("{min_location}");
        format!("{min_location}")
    }

    fn part_two(&self, input: &str) -> String {
        let input = filter_input_lines(input);

        let seed_ranges: Vec<(u64, u64)> = input[0]
            .split(":")
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .chunks(2)
            .map(|chunk| {
                let start: u64 = chunk[0].parse().unwrap();
                let range: u64 = chunk[1].parse().unwrap();
                (start, range)
            })
            .collect();

        let (
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location
        ) = get_maps(input);

        let soil = map_ranges(seed_ranges, &seed_to_soil);
        let fertilizer = map_ranges(soil, &soil_to_fertilizer);
        let water = map_ranges(fertilizer, &fertilizer_to_water);
        let light = map_ranges(water, &water_to_light);
        let temperature = map_ranges(light, &light_to_temperature);
        let humidity = map_ranges(temperature, &temperature_to_humidity);
        let location = map_ranges(humidity, &humidity_to_location);
    
        let min_location = location.iter().min().unwrap().0;

        println!("{min_location}");
        format!("{min_location}")
    }
}

fn get_maps(input: Vec<&str>) -> (Vec<(u64, u64, u64)>, Vec<(u64, u64, u64)>, Vec<(u64, u64, u64)>, Vec<(u64, u64, u64)>, Vec<(u64, u64, u64)>, Vec<(u64, u64, u64)>, Vec<(u64, u64, u64)>) {
    let mut seed_to_soil: Vec<(u64, u64, u64)> = Vec::new();
    let mut soil_to_fertilizer: Vec<(u64, u64, u64)> = Vec::new();
    let mut fertilizer_to_water: Vec<(u64, u64, u64)> = Vec::new();
    let mut water_to_light: Vec<(u64, u64, u64)> = Vec::new();
    let mut light_to_temperature: Vec<(u64, u64, u64)> = Vec::new();
    let mut temperature_to_humidity: Vec<(u64, u64, u64)> = Vec::new();
    let mut humidity_to_location: Vec<(u64, u64, u64)> = Vec::new();

    let mut current_map = &mut seed_to_soil;

    for line in input.into_iter().skip(2) {
        if line.chars().next().unwrap().is_numeric() {
            let nums: Vec<u64> = line
                .split_ascii_whitespace()
                .map(|s| s.trim().parse().unwrap())
                .collect();
            let dest_start = nums[0];
            let source_start = nums[1];
            let range = nums[2];
            current_map.push((dest_start, source_start, range))
        } else {
            match line {
                _ if line.contains("soil-to-fertilizer") => current_map = &mut soil_to_fertilizer,
                _ if line.contains("fertilizer-to-water") => current_map = &mut fertilizer_to_water,
                _ if line.contains("water-to-light") => current_map = &mut water_to_light,
                _ if line.contains("light-to-temperature") => {
                    current_map = &mut light_to_temperature
                }
                _ if line.contains("temperature-to-humidity") => {
                    current_map = &mut temperature_to_humidity
                }
                _ if line.contains("humidity-to-location") => {
                    current_map = &mut humidity_to_location
                }
                _ => panic!(""),
            }
        }
    }

    (seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location)

}

fn map_number(num: u64, maps: &Vec<(u64, u64, u64)>) -> u64 {
    let mut mapped_num = num;
    for map in maps {
        let (dest_start, source_start, range) = map;
        let source_diff = (num as i64) - (*source_start as i64);
        if source_diff >= 0 && source_diff < (*range as i64) {
            mapped_num = dest_start + (source_diff as u64);
            break;
        }
    }
    mapped_num
}

fn map_ranges(num_ranges: Vec<(u64, u64)>, maps: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let mut mapped_ranges: Vec<(u64, u64)> = Vec::new();
    let mut maps = maps.to_owned();
    maps.sort_by(|a, b| a.1.cmp(&b.1));
    for num_range in num_ranges {
        let (mut num_start, num_range) = num_range;
        let num_end = num_start + num_range;
        for map in maps.iter() {

            let (dest_start, source_start, map_range) = map;
            let source_end = source_start + map_range;
            if (*source_start > num_end) || (num_start > source_end) {
                continue;
            }

            let intersection_start = num_start.max(*source_start);
            let intersection_end = num_end.min(source_end);
            let intersection_range = intersection_end - intersection_start;
            let intersection_offset = intersection_start - source_start;
            mapped_ranges.push((dest_start + intersection_offset, intersection_range));

            if num_start < intersection_start {
                mapped_ranges.push((num_start, intersection_start - num_start));
            }

            num_start = intersection_end;

        }

        if num_start < num_end {
            mapped_ranges.push((num_start, num_end - num_start));
        }

    }
    mapped_ranges
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4";
        let value = Day.part_one(input);
        assert_eq!(value, "35");
    }

    #[test]
    fn test_part_two() {
        let input = "
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4";
        let value = Day.part_two(input);
        assert_eq!(value, "46");
    }
}
