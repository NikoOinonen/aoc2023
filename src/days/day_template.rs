
use super::Problem;


pub struct Day;

impl Problem for Day {

    fn part_one(&self, input: &str) -> String {
        println!("{input}");
        format!("{input}")
    }

    fn part_two(&self, input: &str) -> String {
        println!("{input}");
        format!("{input}")
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "Part 1";
        let value = Day.part_one(input);
        assert_eq!(value, "Part 1");
    }

    #[test]
    fn test_part_two() {
        let input = "Part 2";
        let value = Day.part_two(input);
        assert_eq!(value, "Part 2");
    }

}
