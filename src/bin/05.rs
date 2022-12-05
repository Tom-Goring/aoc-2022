use std::str::FromStr;

use nom::{IResult, bytes::complete::tag, character::complete::{digit1, space1, line_ending}, sequence::{preceded, tuple, terminated, delimited}, combinator::{map, eof, opt}, multi::{separated_list0, many1}, branch::alt};

enum Parsed {
    Movement { idx: usize, src: usize, dst: usize },
    Err
}

fn parse_movement(input: &str) -> IResult<&str, Parsed> {
    let parser = 
    terminated(
            
        alt(line_ending, eof),
    );

    // let data_parser = map(parser, |(_, idx, _, src, _, dst): (&str, usize, &str, usize, &str, usize)| Parsed::Movement { idx, src, dst });

    // data_parser(input)
    Ok(("hello", Parsed::Err))
}

fn parse_input(input: &str) -> IResult<&str, Data> {


    Ok(("hello", Data {}))
}

struct Data {
    // stacks: Vec<Vec<char>>,
}

impl FromStr for Data {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Data {})
    }
}

fn _parse_input(_input: &str) {

}

pub fn part_one(_input: &str) -> Option<u32> {
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        let _ = Data::from_str("move 1 from 2 to 3");
        let _ = Data::from_str("1   2   3   4");
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
