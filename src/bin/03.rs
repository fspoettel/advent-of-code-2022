type Input<'a> = Vec<&'a str>;

fn parse(input: &str) -> Input {
    input.lines().collect()
}

fn score_char(code: u8) -> u8 {
    code % 32 + (26 * (code <= 90) as u8)
}

pub fn part_one(input: &Input) -> Option<u32> {
    let prios = input
        .iter()
        .filter_map(|l| {
            let parts = l.split_at(l.len() / 2);
            let a = parts.0.as_bytes();
            let b = parts.1.as_bytes();
            a.iter()
                .find(|byte| b.contains(byte))
                .map(|&byte| score_char(byte) as u32)
        })
        .sum();
    Some(prios)
}

pub fn part_two(input: &Input) -> Option<u32> {
    let prios = input
        .chunks(3)
        .filter_map(|chunks| {
            let mut chunks = chunks.iter();
            let a = chunks.next()?.as_bytes();
            let b = chunks.next()?.as_bytes();
            let c = chunks.next()?.as_bytes();
            a.iter()
                .find(|byte| b.contains(byte) && c.contains(byte))
                .map(|&byte| score_char(byte) as u32)
        })
        .sum();
    Some(prios)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    let parsed = advent_of_code::parse!(parse, input);
    advent_of_code::solve!(1, part_one, &parsed);
    advent_of_code::solve!(2, part_two, &parsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&parse(&input)), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&parse(&input)), Some(70));
    }
}
