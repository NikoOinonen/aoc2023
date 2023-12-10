
pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
mod day_template;

pub trait Problem {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}
