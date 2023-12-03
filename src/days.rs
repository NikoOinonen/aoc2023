
pub mod day_01;
pub mod day_02;
pub mod day_03;
mod day_template;

pub trait Problem {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}
