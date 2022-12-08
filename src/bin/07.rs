#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, digit1, multispace0, tab},
    combinator::{map, map_res, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};

struct IndentationTracker {
    current: usize,
}

#[derive(Clone, Debug)]
pub enum Value {
    File {
        parent: Option<Weak<RefCell<Value>>>,
        ident: Rc<char>,
        ext: Option<Rc<str>>,
        size: u32,
    },
    Dir {
        parent: Option<Weak<RefCell<Value>>>,
        ident: Rc<char>,
        children: Vec<Rc<RefCell<Value>>>,
    },
}

impl Value {
    pub fn add_child(&mut self, value: Rc<RefCell<Value>>) {
        match self {
            Value::File { .. } => panic!("Oh god"),
            Value::Dir { children, .. } => children.push(value),
        }
    }

    pub fn parent(&self) -> Option<Weak<RefCell<Value>>> {
        match self {
            Value::File {
                parent,
                ident,
                ext,
                size,
            } => parent.clone(),
            Value::Dir {
                parent,
                ident,
                children,
            } => parent.clone(),
        }
    }
}

fn parse_line(input: &str, parent: Option<Rc<RefCell<Value>>>) -> IResult<&str, Value> {
    let (input, _) = tag("- ")(input)?;
    let (input, ident) = anychar(input)?;
    let (input, ext) = map(opt(tuple((tag("."), alpha1))), |r| {
        r.map(|(_, ext)| Rc::from(ext))
    })(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("(")(input)?;
    let (input, kind) = alt((tag("dir"), tag("file")))(input)?;

    let value = match kind {
        "dir" => {
            let (input, _) = tag(")")(input)?;
            Value::Dir {
                parent: parent.map(|rc| Rc::downgrade(&rc)),
                ident: Rc::from(ident),
                children: Vec::new(),
            }
        }
        "file" => {
            let (input, _) = tag(", size=")(input)?;
            let (input, size) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
            Value::File {
                parent: parent.map(|rc| Rc::downgrade(&rc)),
                ident: Rc::from(ident),
                ext,
                size,
            }
        }
        _ => panic!("Got a weird kind"),
    };

    Ok((input, value))
}

fn parse_input<'a>(
    input: &'a str,
    tracker: &'a mut IndentationTracker,
) -> IResult<&'a str, &'a str> {
    let mut lines = input.lines();
    let mut current_dir: Rc<RefCell<Value>> =
        parse_line(lines.next().unwrap(), None).map(|(_, v)| Rc::new(RefCell::new(v)))?;

    for line in lines {
        let (line, tabs) = many0(tab)(line)?;
        let value =
            parse_line(line, Some(current_dir.clone())).map(|(_, v)| Rc::new(RefCell::new(v)))?;

        match tabs.len().cmp(&tracker.current) {
            std::cmp::Ordering::Greater => {
                current_dir = value.clone();
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Less => {
                let parent = current_dir.borrow().parent().unwrap().upgrade().unwrap();
                current_dir = parent;
            }
        }
        tracker.current = tabs.len();

        if let Value::File { .. } = value {
            current_dir
                .borrow_mut()
                .deref_mut()
                .add_child(value.clone());
        };
    }

    println!("{current_dir:?}");
    Ok(("hello", "hello"))
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut tracker = IndentationTracker { current: 0 };
    let input = parse_input(input, &mut tracker);
    println!("{input:?}");
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7).replace("  ", "\t");
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7).replace("  ", "\t");
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7).replace("  ", "\t");
        assert_eq!(part_two(&input), None);
    }
}
