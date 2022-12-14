pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|l| scan_fmt::scan_fmt!(l, "{} {}", char, char))
            .map(|(a, b)| match (a, b) {
                ('A', 'X') => 1 + 3, // 4
                ('B', 'X') => 1,     // 1
                ('C', 'X') => 1 + 6, // 7

                ('A', 'Y') => 2 + 6, // 8
                ('B', 'Y') => 2 + 3, // 5
                ('C', 'Y') => 2,     // 2
                //
                ('A', 'Z') => 3,     // 3
                ('B', 'Z') => 3 + 6, // 9
                ('C', 'Z') => 3 + 3, // 6
                (_, _) => panic!("wtf"),
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|l| scan_fmt::scan_fmt!(l, "{} {}", char, char))
            .map(|(a, b)| match a {
                'A' => match b {
                    'Y' => 3 + 1,
                    'X' => 3,
                    'Z' => 6 + 2,
                    _ => panic!("?"),
                },
                'B' => match b {
                    'Y' => 3 + 2,
                    'X' => 1,
                    'Z' => 6 + 3,
                    _ => panic!("?"),
                },
                'C' => match b {
                    'Y' => 3 + 3,
                    'X' => 2,
                    'Z' => 6 + 1,
                    _ => panic!("?"),
                },
                _ => panic!("?"),
            })
            .sum(),
    )
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
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
