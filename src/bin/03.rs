use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;
use once_cell::sync::Lazy;

static MAP: Lazy<HashMap<char, u32>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for (i, c) in ('a'..='z').enumerate() {
        map.insert(c, (i + 1) as u32);
    }

    for (i, c) in ('A'..='Z').enumerate() {
        map.insert(c, (i + 27) as u32);
    }

    map
});

pub fn parse_input(input: &str) -> Vec<(HashSet<u32>, HashSet<u32>)> {
    input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(c1, c2)| {
            let a: HashSet<u32> = c1.chars().map(|c| MAP[&c]).collect();
            let b: HashSet<u32> = c2.chars().map(|c| MAP[&c]).collect();
            (a, b)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .iter()
            .map(|elf| elf.0.intersection(&elf.1).sum::<u32>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .iter()
            .map(|(c1, c2)| c1 | c2)
            .chunks(3)
            .into_iter()
            .map(|mut elf_group| {
                elf_group
                    .next()
                    .map(|elf| elf_group.fold(elf, |elf1, elf2| &elf1 & &elf2))
                    .unwrap()
                    .into_iter()
                    .next()
                    .unwrap()
            })
            .sum(),
    )
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
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
