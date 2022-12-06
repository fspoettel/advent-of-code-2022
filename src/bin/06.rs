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

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), Some(7));
        assert_eq!(part_one(parse("bvwbjplbgvbhsrlpgdmjqwftvncz")), Some(5));
        assert_eq!(part_one(parse("nppdvjthqldpwncqszvftbrmjlhg")), Some(6));
        assert_eq!(
            part_one(parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            Some(10)
        );
        assert_eq!(
            part_one(parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
            Some(11)
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), Some(19));
    }
}
