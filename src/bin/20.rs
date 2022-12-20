use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Item(usize, i64);

type Input<'a> = VecDeque<Item>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .enumerate()
        .filter_map(|(i, l)| Some(Item(i, l.parse().ok()?)))
        .collect()
}

fn solve(input: &mut VecDeque<Item>, rounds: usize, key: i64) -> Option<i64> {
    let len = input.len();

    for item in input.iter_mut() {
        item.1 *= key;
    }

    for _ in 0..rounds {
        for i in 0..len {
            let idx = input
                .iter()
                .enumerate()
                .find_map(
                    |(idx, &Item(original_idx, _))| {
                        if i == original_idx {
                            Some(idx)
                        } else {
                            None
                        }
                    },
                )
                .unwrap();

            input.rotate_left(idx);
            let item = input.pop_front().unwrap();
            input.rotate_left(item.1.rem_euclid(len as i64 - 1) as usize);
            input.push_front(item);
        }
    }

    input.iter().enumerate().find_map(|(i, Item(_, val))| {
        if *val == 0 {
            Some(input[(i + 1000) % len].1 + input[(i + 2000) % len].1 + input[(i + 3000) % len].1)
        } else {
            None
        }
    })
}

pub fn part_one(mut input: Input) -> Option<i64> {
    solve(&mut input, 1, 1)
}

pub fn part_two(mut input: Input) -> Option<i64> {
    solve(&mut input, 10, 811589153)
}

advent_of_code::main!(20);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 20)));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 20)));
        assert_eq!(result, Some(1623178306));
    }
}
