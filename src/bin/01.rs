pub fn part_one(input: &str) -> Option<u32> {
    let mut cals: Vec<u32> = Vec::new();
    let mut elf: u32 = 0;
    for line in input.lines() {
        let num = line.parse::<u32>();
        if num.is_ok() {
            elf += num.unwrap();
        } else {
            cals.push(elf);
            elf = 0;
        }
    }
    Some(cals.iter().fold(std::u32::MIN, |a, b| a.max(*b)))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cals: Vec<u32> = Vec::new();
    let mut elf: u32 = 0;
    for line in input.lines() {
        let num = line.parse::<u32>();
        if num.is_ok() {
            elf += num.unwrap();
        } else {
            cals.push(elf);
            elf = 0;
        }
    }
    cals.sort();
    cals.reverse();
    let total = cals[0] + cals[1] + cals[2];
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
