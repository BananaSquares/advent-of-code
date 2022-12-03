#[derive(PartialEq, Debug)]
enum Types {
    Rock,
    Paper,
    Scissors,
}
#[derive(PartialEq, Debug)]
enum Result {
    Win,
    Lose,
    Draw,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut points = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let type1: Types;
        let type2: Types;
        let mut rpoints = 0;
        match parts[1] {
            "Z" => {
                rpoints = rpoints + 3;
                type2 = Types::Scissors
            }
            "X" => {
                rpoints = rpoints + 1;
                type2 = Types::Rock;
            }
            "Y" => {
                rpoints = rpoints + 2;
                type2 = Types::Paper;
            }
            _ => {
                panic!("{}", parts[1]);
            }
        }
        match parts[0] {
            "A" => {
                type1 = Types::Rock;
            }
            "B" => {
                type1 = Types::Paper;
            }
            "C" => {
                type1 = Types::Scissors;
            }
            _ => {
                panic!("{}", parts[0]);
            }
        }
        if type1 == type2 {
            rpoints = rpoints + 3;
        } else if type1 == Types::Rock {
            if type2 == Types::Paper {
                rpoints = rpoints + 6;
            }
        } else if type1 == Types::Paper {
            if type2 == Types::Scissors {
                rpoints = rpoints + 6;
            }
        } else if type1 == Types::Scissors {
            if type2 == Types::Rock {
                rpoints = rpoints + 6;
            }
        }
        points = points + rpoints;
    }
    Some(points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut points = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let type1: Types;
        let type2: Result;
        let mut rpoints = 0;
        match parts[1] {
            "Z" => {
                rpoints = rpoints + 6;
                type2 = Result::Win;
            }
            "X" => {
                type2 = Result::Lose;
            }
            "Y" => {
                rpoints = rpoints + 3;
                type2 = Result::Draw;
            }
            _ => {
                panic!("{}", parts[1]);
            }
        }
        match parts[0] {
            "A" => {
                type1 = Types::Rock;
            }
            "B" => {
                type1 = Types::Paper;
            }
            "C" => {
                type1 = Types::Scissors;
            }
            _ => {
                panic!("{}", parts[0]);
            }
        }
        if type1 == Types::Rock {
            if type2 == Result::Win {
                rpoints = rpoints + 2;
            } else if type2 == Result::Draw {
                rpoints = rpoints + 1
            } else {
                rpoints = rpoints + 3;
            }
        } else if type1 == Types::Paper {
            if type2 == Result::Win {
                rpoints = rpoints + 3;
            } else if type2 == Result::Draw {
                rpoints = rpoints + 2
            } else {
                rpoints = rpoints + 1;
            }
        } else if type1 == Types::Scissors {
            if type2 == Result::Win {
                rpoints = rpoints + 1;
            } else if type2 == Result::Draw {
                rpoints = rpoints + 3
            } else {
                rpoints = rpoints + 2;
            }
        }
        points = points + rpoints;
    }
    Some(points)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
