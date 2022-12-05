use std::{cmp, ops::Range};

type Input = Vec<(Range<u8>, Range<u8>)>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|l| {
            let (a, b) = l.split_once(',')?;
            let (a_start, a_end) = a.split_once('-')?;
            let (b_start, b_end) = b.split_once('-')?;
            Some((
                (a_start.parse().ok()?)..(a_end.parse().ok()?),
                (b_start.parse().ok()?)..(b_end.parse().ok()?),
            ))
        })
        .collect()
}

fn envelops(a: &Range<u8>, b: &Range<u8>) -> bool {
    b.start >= a.start && b.end <= a.end
}

pub fn part_one(input: Input) -> Option<usize> {
    Some(
        input
            .iter()
            .filter(|(a, b)| envelops(a, b) || envelops(b, a))
            .count(),
    )
}

fn overlaps(a: &Range<u8>, b: &Range<u8>) -> bool {
    cmp::max(a.start, b.start) <= cmp::min(a.end, b.end)
}

pub fn part_two(input: Input) -> Option<usize> {
    Some(input.iter().filter(|(a, b)| overlaps(a, b)).count())
}

advent_of_code::main!(4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::read_file("examples", 4)));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::read_file("examples", 4)));
        assert_eq!(result, Some(4));
    }
}
