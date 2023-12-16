use super::Problem;

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let hash_sum: usize = input.trim().split(',').map(|s| get_hash(s)).sum();
        println!("{hash_sum}");
        format!("{hash_sum}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut boxes: Vec<Vec<(&str, u32)>> = (0..256).map(|_| Vec::new()).collect();
        for instruction in input.trim().split(',') {
            let op_ind = instruction.chars().position(|c| c == '-' || c == '=').unwrap();
            let label = &instruction[..op_ind];
            let hash = get_hash(label);
            let operation = instruction.chars().nth(op_ind).unwrap();
            match operation {
                '-' => {
                    let bx = &mut boxes[hash];
                    if let Some(ind) = bx.iter().position(|(l, _)| *l == label) {
                        bx.remove(ind);
                    }
                }
                '=' => {
                    let focal_length: u32 = instruction[(op_ind + 1)..].parse().unwrap();
                    let new_lens = (label, focal_length);
                    let bx = &mut boxes[hash];
                    if let Some(ind) = bx.iter().position(|(l, _)| *l == label) {
                        bx[ind] = new_lens;
                    } else {
                        bx.push(new_lens);
                    }
                }
                _ => panic!(),
            }
        }
        let total_focusing_power: u32 = boxes
            .iter()
            .enumerate()
            .map(|(i, bx)| {
                let box_num = i as u32 + 1;
                bx.iter()
                    .enumerate()
                    .map(|(slot, (_, focal_lenght))| box_num * (slot as u32 + 1) * focal_lenght)
                    .sum::<u32>()
            })
            .sum();
        println!("{total_focusing_power}");
        format!("{total_focusing_power}")
    }
}

fn get_hash(s: &str) -> usize {
    let mut hash: u32 = 0;
    for c in s.chars() {
        hash += c as u32;
        hash *= 17;
        hash %= 256;
    }
    hash as usize
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let value = Day.part_one(input);
        assert_eq!(value, "1320");
    }

    #[test]
    fn test_part_two() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let value = Day.part_two(input);
        assert_eq!(value, "145");
    }
}
