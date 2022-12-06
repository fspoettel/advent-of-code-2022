use itertools::Itertools;

type Input<'a> = Vec<char>;

fn parse(input: &str) -> Input {
    input.chars().collect()
}

fn find_marker(chars: &[char], window_size: usize) -> Option<usize> {
    chars
        .windows(window_size)
        .find_position(|chars| {
            for (i, x) in chars.iter().enumerate() {
                for (j, y) in chars.iter().enumerate() {
                    if i != j && x == y {
                        return false;
                    }
                }
            }

            true
        })
        .map(|(pos, _)| pos + window_size)
}

pub fn part_one(input: Input) -> Option<usize> {
    // using simple compares here is ~5x faster than this looped solution.
    find_marker(&input, 4)
}

pub fn part_two(input: Input) -> Option<usize> {
    find_marker(&input, 14)
}

advent_of_code::main!(6);

#[cfg(test)]
mod tests {
    use super::*;

    fn test_answer(func: &dyn Fn(Input) -> Option<usize>, answers: &[usize]) {
        advent_of_code::read_file("examples", 6)
            .lines()
            .enumerate()
            .for_each(|(i, l)| {
                assert_eq!(func(parse(l)), Some(answers[i]));
            })
    }

    #[test]
    fn test_part_one() {
        test_answer(&part_one, &[7, 5, 6, 10, 11]);
    }

    #[test]
    fn test_part_two() {
        test_answer(&part_two, &[19, 23, 23, 29, 26]);
    }
}
