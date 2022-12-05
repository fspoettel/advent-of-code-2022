type Input = Vec<(u32, u32)>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|l| {
            // normalize to index 0 with ASCII table. (A -> 65 | X -> 88)
            let bytes = l.as_bytes();
            Some(((bytes.first()? - 65) as u32, (bytes.last()? - 88) as u32))
        })
        .collect()
}

fn score_distance(theirs: u32, ours: u32) -> u32 {
    // game result can be determined by calculating wrapped distance.
    // use modulo to map indexes (0, 1, 2) to (loss, draw, win) for multiplication with 3.
    let score = (3 - (2 + theirs - ours) % 3) % 3 * 3;
    score + ours + 1
}

pub fn part_one(input: Input) -> Option<u32> {
    Some(
        input
            .iter()
            .map(|&(theirs, ours)| score_distance(theirs, ours))
            .sum(),
    )
}

pub fn part_two(input: Input) -> Option<u32> {
    let score = input
        .iter()
        .map(|&(theirs, result)| {
            // reverse scoring method to find our strategy for round.
            let ours = match result {
                0 => (theirs + 2) % 3,
                1 => theirs,
                2 => (theirs + 1) % 3,
                _ => unreachable!(),
            };

            score_distance(theirs, ours)
        })
        .sum();

    Some(score)
}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::read_file("examples", 2)));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::read_file("examples", 2)));
        assert_eq!(result, Some(12));
    }
}
