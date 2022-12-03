use std::collections::{HashMap, HashSet};
fn sharedchar(b:String,a:String) -> char {
    let (shorter, longer) = if a.len() > b.len() {
        (b, a)
    }  else {
        (a, b)
    };
    let set: HashSet<char> = shorter.chars().collect();
    for i in longer.chars() {
        if set.contains(&i) { return i; }
        }
        panic!();
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut codes = HashMap::new();
    for (i,letter) in ('a'..='z').enumerate() {
        codes.insert(letter, i+1);
    }for (i,letter) in ('A'..='Z').enumerate() {
        codes.insert(letter, i+27);
    }
    let mut result = 0;
    for line in input.lines() {
    let e =  line.chars().enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % (line.len() / 2) == 0 {
                Some(' ')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>();
        let compartments: Vec<&str> = e.split(" ").collect();
        let b = sharedchar(compartments[0].to_string(), compartments[1].to_string());
        if !codes.contains_key(&b) { panic!()}
        result+= (*codes.entry(b).or_default()) as u32
    }
    Some(result)
}
fn sharedchar2(c:String,b:String,a:String) -> char {
    let (shorter, longer) = if a.len() > b.len() {
        (b, a)
    }  else {
        (a, b)
    };
    let set: HashSet<char> = shorter.chars().collect();
    let mut secondset: HashSet<char> = HashSet::new();
    for i in longer.chars() {
        if set.contains(&i) { secondset.insert(i); }
    }
    for i in c.chars() {
        if secondset.contains(&i) {
            return i;
        }
    }
        panic!();

}
pub fn part_two(input: &str) -> Option<u32> {
    let mut codes = HashMap::new();
    for (i,letter) in ('a'..='z').enumerate() {
        codes.insert(letter, i+1);
    }for (i,letter) in ('A'..='Z').enumerate() {
        codes.insert(letter, i+27);
    }
    let mut result = 0;
    let mut a: &str = "";
    let mut b: &str = "";
    for (i,line )in input.lines().enumerate() {
        let z = i % 3;
        if z == 0 {a = line}
        else if z == 1 {b = line}
        else if z == 2 {
            let b = sharedchar2(a.to_string(), b.to_string(),line.to_string());
            if !codes.contains_key(&b) { panic!()}
            result+= (*codes.entry(b).or_default()) as u32
        }else {
            panic!();
        };
    }
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
