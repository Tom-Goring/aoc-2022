use itertools::Itertools;

// 5.17µs if compiled with RUSTFLAGS="-C target-cpu=native"!
pub fn solve(input: &str, size: usize) -> Option<u32> {
    let masks = input
        .chars()
        .map(|c| 1u32 << (c as u32 - 'a' as u32)) // this turns a lowercase char into its index into the alphabet
        .collect_vec();

    let mut accum = 0u32;
    for (i, mask) in masks.iter().enumerate() {
        accum ^= mask;
        if i >= size {
            accum ^= masks[i - size];
            if accum.count_ones() as usize == size {
                // and then here we can xor the bitmask into an accum to keep track of which letters
                // we've seen, and if we see 'size' number of them they're all unique!
                return Some((i + 1) as u32);
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
