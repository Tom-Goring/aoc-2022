use std::str::FromStr;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult, Parser,
};

#[derive(Debug)]
enum Parsed {
    Movement(usize, usize, usize),
    CrateRow(Vec<Option<char>>),
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse())(input)
}

fn parse_line(input: &str) -> IResult<&str, Parsed> {
    alt((parse_crate_row, parse_movement))(input)
}

fn parse_movement(input: &str) -> IResult<&str, Parsed> {
    map(
        tuple((
            tag("move "),
            parse_number,
            tag(" from "),
            parse_number,
            tag(" to "),
            parse_number,
        )),
        |(_, idx, _, src, _, dst)| Parsed::Movement(idx, src, dst),
    )(input)
}

fn parse_crate_row(input: &str) -> IResult<&str, Parsed> {
    let parser = separated_list1(
        tag(" "),
        map(delimited(tag("["), anychar, tag("]")), Some).or(map(tag("   "), |_t: &str| None)),
    );

    map(parser, Parsed::CrateRow)(input)
}

#[derive(Debug)]
struct Data {
    stacks: Vec<Vec<char>>,
    movements: Vec<(usize, usize, usize)>,
}

impl FromStr for Data {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut crates = Vec::new();
        let mut movements = Vec::new();

        for line in s
            .lines()
            .filter(|l| !l.is_empty())
            .flat_map(parse_line)
            .map(|(_, x)| x)
        {
            match line {
                Parsed::Movement(idx, src, dst) => movements.push((idx, src - 1, dst - 1)),
                Parsed::CrateRow(row) => crates.push(row),
            }
        }

        let stacks = (0..crates[0].len())
            .map(|i| crates.iter().rev().filter_map(|c| c[i]).collect_vec())
            .collect_vec();

        Ok(Data { stacks, movements })
    }
}

impl Data {
    pub fn solve1(&mut self) {
        for (idx, src, dst) in self.movements.iter() {
            for _ in 0..*idx {
                let crt = self.stacks[*src]
                    .pop()
                    .expect("Tried to move a crate where there were none to move!");
                self.stacks[*dst].push(crt);
            }
        }
    }

    pub fn solve2(&mut self) {
        for (idx, src, dst) in self.movements.iter() {
            let mut crates = Vec::new();
            for _ in 0..*idx {
                crates.push(self.stacks[*src].pop().unwrap());
            }
            for c in crates.iter().rev() {
                self.stacks[*dst].push(*c);
            }
        }
    }

    pub fn stack_tops(&self) -> String {
        self.stacks.iter().flat_map(|s| s.last()).collect()
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut data = Data::from_str(input).unwrap();
    data.solve1();
    Some(data.stack_tops())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut data = Data::from_str(input).unwrap();
    data.solve2();
    Some(data.stack_tops())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_owned()));
    }
}
