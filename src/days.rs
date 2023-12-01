
pub mod day_01;
pub mod day_02;

pub trait Problem {
    fn part_one(&self, input: &str);
    fn part_two(&self, input: &str);
}
