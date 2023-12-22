use core::panic;
use std::collections::HashMap;

use regex::Regex;

use super::Problem;

#[derive(Debug)]
enum Filter {
    Map((char, char, u32, String)),
    Destination(String),
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn get_val(&self, var: char) -> u32 {
        match var {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!(),
        }
    }
    fn replace(&self, var: char, val: u32) -> Part {
        let mut new_part = self.clone();
        match var {
            'x' => new_part.x = val,
            'm' => new_part.m = val,
            'a' => new_part.a = val,
            's' => new_part.s = val,
            _ => panic!(),
        }
        new_part
    }
}

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let (workflows, parts) = get_workflows_parts(input);

        let accepted_parts: Vec<Part> = parts
            .into_iter()
            .filter_map(|part| {
                let mut filters = workflows.get("in").unwrap();
                loop {
                    for filter in filters.iter() {
                        let destination = match filter {
                            Filter::Destination(dest) => dest,
                            Filter::Map((var, op, val, dest)) => {
                                let part_val = part.get_val(*var);
                                let res = match op {
                                    '>' => part_val > *val,
                                    '<' => part_val < *val,
                                    _ => panic!(),
                                };
                                if res {
                                    dest
                                } else {
                                    continue;
                                }
                            }
                        };
                        match destination.as_str() {
                            "A" => return Some(part),
                            "R" => return None,
                            dest => {
                                filters = workflows.get(dest).unwrap();
                                break;
                            }
                        }
                    }
                }
            })
            .collect();

        let total_value: u32 = accepted_parts.iter().map(|part| part.x + part.m + part.a + part.s).sum();

        println!("{total_value}");
        format!("{total_value}")
    }

    fn part_two(&self, input: &str) -> String {
        let (workflows, _) = get_workflows_parts(input);

        let init_range = (
            Part { x: 1, m: 1, a: 1, s: 1 },
            Part {
                x: 4000,
                m: 4000,
                a: 4000,
                s: 4000,
            },
            "in",
        );
        let mut ranges = vec![init_range];
        let mut final_ranges = Vec::new();

        while let Some((mut start, mut end, workflow_name)) = ranges.pop() {
            let filters = workflows.get(workflow_name).unwrap();
            for filter in filters {
                let (destination, new_range) = match filter {
                    Filter::Destination(dest) => (Some((start, end, dest.as_str())), None),
                    Filter::Map((var, op, val, dest)) => {
                        let start_val = start.get_val(*var);
                        let end_val = end.get_val(*var);
                        match op {
                            '>' => {
                                if start_val > *val {
                                    (Some((start.clone(), end.clone(), dest.as_str())), None)
                                } else if end_val > *val {
                                    (
                                        Some((start.replace(*var, val + 1), end.clone(), dest.as_str())),
                                        Some((start, end.replace(*var, *val))),
                                    )
                                } else {
                                    (None, Some((start, end)))
                                }
                            }
                            '<' => {
                                if end_val < *val {
                                    (Some((start.clone(), end.clone(), dest.as_str())), None)
                                } else if start_val < *val {
                                    (
                                        Some((start.clone(), end.replace(*var, val - 1), dest.as_str())),
                                        Some((start.replace(*var, *val), end)),
                                    )
                                } else {
                                    (None, Some((start, end)))
                                }
                            }
                            _ => panic!(),
                        }
                    }
                };
                if let Some((s, e, dest)) = destination {
                    if dest == "A" {
                        final_ranges.push((s, e));
                    } else if dest != "R" {
                        ranges.push((s, e, dest));
                    }
                }
                match new_range {
                    Some((new_start, new_end)) => {
                        start = new_start;
                        end = new_end;
                    }
                    None => {
                        break;
                    }
                }
            }
        }

        let total_combinations: u64 = final_ranges
            .into_iter()
            .map(|(start, end)| {
                ((end.x - start.x + 1) as u64)
                    * ((end.m - start.m + 1) as u64)
                    * ((end.a - start.a + 1) as u64)
                    * ((end.s - start.s + 1) as u64)
            })
            .sum();

        println!("{total_combinations}");
        format!("{total_combinations}")
    }
}

fn get_workflows_parts(input: &str) -> (HashMap<String, Vec<Filter>>, Vec<Part>) {
    let re_empty_line = Regex::new(r"\n[ ]*\n").unwrap();
    let input: Vec<Vec<&str>> = re_empty_line.split(input).map(|s| s.split_ascii_whitespace().collect()).collect();

    let re_workflow = Regex::new(r"(?<name>.+)\{(?<filters>.+)\}").unwrap();
    let workflows: HashMap<String, Vec<Filter>> = input[0]
        .iter()
        .map(|line| {
            let cap = re_workflow.captures(line).unwrap();
            let name = cap["name"].to_string();
            let filters = cap["filters"]
                .split(',')
                .map(|s| match s.split_once(':') {
                    Some((a, dest)) => {
                        let var = a.chars().nth(0).unwrap();
                        let op = a.chars().nth(1).unwrap();
                        let val = a[2..].parse().unwrap();
                        Filter::Map((var, op, val, dest.to_string()))
                    }
                    None => Filter::Destination(s.to_string()),
                })
                .collect();
            (name, filters)
        })
        .collect();
    let parts: Vec<Part> = input[1]
        .iter()
        .map(|line| {
            let line = &line[1..(line.len() - 1)];
            let values: Vec<u32> = line
                .split(',')
                .map(|s| {
                    let (_, val) = s.split_once('=').unwrap();
                    val.parse().unwrap()
                })
                .collect();
            Part {
                x: values[0],
                m: values[1],
                a: values[2],
                s: values[3],
            }
        })
        .collect();

    (workflows, parts)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}
        
        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}";
        let value = Day.part_one(input);
        assert_eq!(value, "19114");
    }

    #[test]
    fn test_part_two() {
        let input = "
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}
        
        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}";
        let value = Day.part_two(input);
        assert_eq!(value, "167409079868000");
    }
}
