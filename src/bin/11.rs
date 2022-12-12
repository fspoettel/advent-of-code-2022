type Input = Vec<Monkey>;

#[derive(Clone)]
enum Operand {
    OldValue,
    Number(u64),
}

impl Operand {
    fn try_from_str(s: &str) -> Option<Self> {
        match s {
            "old" => Some(Operand::OldValue),
            x => Some(Operand::Number(x.parse().ok()?)),
        }
    }

    fn apply(&self, x: u64) -> u64 {
        match self {
            Operand::OldValue => x,
            Operand::Number(y) => *y,
        }
    }
}

#[derive(Clone)]
enum Operation {
    Add(Operand, Operand),
    Multiply(Operand, Operand),
}

impl Operation {
    fn apply(&self, x: u64) -> u64 {
        match self {
            Self::Add(a, b) => a.apply(x) + b.apply(x),
            Self::Multiply(a, b) => a.apply(x) * b.apply(x),
        }
    }
}

#[derive(Clone)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divide_by: u64,
    next_falsy: usize,
    next_truthy: usize,
}

impl Monkey {
    fn try_from_str(str: &str) -> Option<Self> {
        let mut lines = str.lines().skip(1);

        let items = lines
            .next()?
            .split_once(':')?
            .1
            .split(',')
            .filter_map(|x| x.trim().parse().ok())
            .collect();

        let mut operation_line = lines.next()?.split_once("= ")?.1.split_ascii_whitespace();
        let operand_a = operation_line.next().map(Operand::try_from_str)??;
        let operator = operation_line.next()?;
        let operand_b = operation_line.next().map(Operand::try_from_str)??;
        let operation = match operator {
            "+" => Operation::Add(operand_a, operand_b),
            "*" => Operation::Multiply(operand_a, operand_b),
            _ => unreachable!(),
        };

        let divide_by = lines
            .next()?
            .split_ascii_whitespace()
            .last()?
            .parse()
            .ok()?;

        let next_truthy = lines
            .next()?
            .split_ascii_whitespace()
            .last()?
            .parse()
            .ok()?;

        let next_falsy = lines
            .next()?
            .split_ascii_whitespace()
            .last()?
            .parse()
            .ok()?;

        Some(Self {
            items,
            operation,
            divide_by,
            next_falsy,
            next_truthy,
        })
    }
}

fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .into_iter()
        .filter_map(Monkey::try_from_str)
        .collect()
}

fn simulate(mut monkeys: Input, rounds: usize, func: impl Fn(u64) -> u64) -> usize {
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..(monkeys.len()) {
            let monkey = &mut monkeys[i];

            let items: Vec<u64> = monkey
                .items
                .drain(..)
                .map(|item| func(monkey.operation.apply(item)))
                .collect();

            inspections[i] += items.len();

            let divide_by = monkey.divide_by;
            let next_truthy = monkey.next_truthy;
            let next_falsy = monkey.next_falsy;

            for item in items {
                let target = if item % divide_by == 0 {
                    next_truthy
                } else {
                    next_falsy
                };
                monkeys[target].items.push(item);
            }
        }
    }

    inspections.sort_unstable();
    inspections.iter().rev().take(2).product()
}

pub fn part_one(monkeys: Input) -> Option<usize> {
    Some(simulate(monkeys, 20, |x| x / 3))
}

pub fn part_two(monkeys: Input) -> Option<usize> {
    let base: u64 = monkeys.iter().map(|x| x.divide_by).product();
    Some(simulate(monkeys, 10000, |x| x % base))
}

advent_of_code::main!(11);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn divide_by_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 11)));
        assert_eq!(result, Some(10605));
    }

    #[test]
    fn divide_by_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 11)));
        assert_eq!(result, Some(2713310158));
    }
}
