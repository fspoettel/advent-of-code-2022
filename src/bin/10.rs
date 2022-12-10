use itertools::Itertools;
use std::collections::VecDeque;

type Input = VecDeque<Op>;

#[derive(Clone)]
pub enum Op {
    Noop,
    Add(i32),
}

#[derive(Clone)]
pub struct Task {
    op: Op,
    after: i32,
}

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|l| {
            if l.starts_with("noop") {
                Some(Op::Noop)
            } else {
                let (_, value) = l.split_once(' ')?;
                Some(Op::Add(value.parse().ok()?))
            }
        })
        .collect()
}

pub fn part_one(mut stack: Input) -> Option<i32> {
    let mut signal_strengths = vec![];

    work_loop(&mut stack, |cycle, value| {
        if cycle == 20 || cycle % 40 == 20 {
            signal_strengths.push(cycle * value);
        }
    });

    Some(signal_strengths.iter().sum())
}

pub fn part_two(mut stack: Input) -> Option<String> {
    let mut pixels = vec![];

    work_loop(&mut stack, |cycle, value| {
        let col = (cycle - 1) % 40;
        if col >= value - 1 && col <= value + 1 {
            pixels.push('#');
        } else {
            pixels.push('.')
        };
    });

    Some(pixels.chunks(40).map(String::from_iter).join("\n"))
}

fn work_loop(stack: &mut Input, mut callback: impl FnMut(i32, i32)) {
    let mut value = 1;
    let mut cycle = 1;
    let mut task: Option<Task> = None;

    while !stack.is_empty() {
        // assign task if nothing is processing.
        if task.is_none() {
            let op = stack.pop_front().unwrap();
            let after = match op {
                Op::Noop => cycle,
                Op::Add(_) => cycle + 1,
            };
            task = Some(Task { op, after })
        }

        // pass value to callback once each cycle.
        callback(cycle as i32, value);

        let current_task = task.as_ref().unwrap();
        // apply current task if due.
        if current_task.after == cycle {
            if let Op::Add(x) = current_task.op {
                value += x
            };
            task = None;
        }

        cycle += 1;
    }
}

advent_of_code::main!(10);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::read_file("examples", 10)));
        assert_eq!(result, Some(13140));
    }

    #[test]
    fn test_part_two() {
        let s = [
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ];
        let result = part_two(parse(&advent_of_code::read_file("examples", 10)));
        assert_eq!(result, Some(s.join("\n").to_string()));
    }
}
