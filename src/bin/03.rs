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

advent_of_code::main!(3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&parse(&advent_of_code::read_file("examples", 3)));
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&parse(&advent_of_code::read_file("examples", 3)));
        assert_eq!(result, Some(70));
    }
}
