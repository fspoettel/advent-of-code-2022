use itertools::Itertools;

fn score_char(code: u8) -> u8 {
    code % 32 + (26 * (code <= 90) as u8)
}

pub fn part_one(input: &str) -> Option<u32> {
    let prios = input
        .lines()
        .filter_map(|l| {
            let parts = l.split_at(l.len() / 2);
            let a = parts.0.as_bytes();
            let b = parts.1.as_bytes();
            a.iter()
                .find(|x| b.contains(x))
                .map(|x| score_char(*x) as u32)
        })
        .sum();

    Some(prios)
}

pub fn part_two(input: &str) -> Option<u32> {
    let prios = input
        .lines()
        .chunks(3)
        .into_iter()
        .filter_map(|mut chunks| {
            let a = chunks.next()?.as_bytes();
            let b = chunks.next()?.as_bytes();
            let c = chunks.next()?.as_bytes();
            a.iter()
                .find(|x| b.contains(x) && c.contains(x))
                .map(|x| score_char(*x) as u32)
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
