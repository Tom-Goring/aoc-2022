pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|l| scan_fmt::scan_fmt!(l, "{}-{},{}-{}", i32, i32, i32, i32))
            .map(|(a, b, c, d)| (a - c) * (b - d))
            .filter(|&b| b <= 0)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|l| scan_fmt::scan_fmt!(l, "{}-{},{}-{}", i32, i32, i32, i32))
            .filter(|(a, b, c, d)| a.max(c) <= b.min(d))
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
