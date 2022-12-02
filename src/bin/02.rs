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
    let score = match (3 + theirs - ours) % 3 {
        0 => 3,
        1 => 0,
        2 => 6,
        _ => unreachable!(),
    };

    score + ours + 1
}

pub fn part_one(input: &Input) -> Option<u32> {
    Some(
        input
            .iter()
            .map(|&(theirs, ours)| score_distance(theirs, ours))
            .sum(),
    )
}

pub fn part_two(input: &Input) -> Option<u32> {
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

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    let parsed = advent_of_code::parse!(parse, input);
    advent_of_code::solve!(1, part_one, &parsed);
    advent_of_code::solve!(2, part_two, &parsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&parse(&input)), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&parse(&input)), Some(12));
    }
}