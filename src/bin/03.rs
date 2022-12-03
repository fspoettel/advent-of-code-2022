use itertools::Itertools;
use std::collections::HashSet;

fn to_byte_set(s: &str) -> HashSet<&u8> {
    HashSet::from_iter(s.as_bytes())
}

fn score_char(code: u8) -> u8 {
    code % 32 + (26 * (code <= 90) as u8)
}

pub fn part_one(input: &str) -> Option<u32> {
    let prios = input
        .lines()
        .filter_map(|l| {
            let parts = l.split_at(l.len() / 2);
            let a = to_byte_set(parts.0);
            let b = to_byte_set(parts.1);
            Some(score_char(**a.intersection(&b).next()?) as u32)
        })
        .sum();
    Some(prios)
}

pub fn part_two(input: &str) -> Option<u32> {
    let prios = input
        .lines()
        .chunks(3)
        .into_iter()
        .filter_map(|chunks| {
            let matches = chunks
                .map(to_byte_set)
                .reduce(|acc, chunk| acc.intersection(&chunk).copied().collect())?;
            Some(score_char(**matches.iter().next()?) as u32)
        })
        .sum();
    Some(prios)
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
