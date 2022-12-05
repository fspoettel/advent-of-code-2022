use itertools::Itertools;
use std::cmp::Reverse;

type Input = Vec<u32>;

fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|c| c.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
        .collect()
}

pub fn part_one(calorie_counts: Input) -> Option<u32> {
    calorie_counts.iter().max().copied()
}

pub fn part_two(calorie_counts: Input) -> Option<u32> {
    Some(
        calorie_counts
            .iter()
            .sorted_by_key(|x| Reverse(*x))
            .take(3)
            .sum(),
    )
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::read_file("examples", 1)));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::read_file("examples", 1)));
        assert_eq!(result, Some(45000));
    }
}
