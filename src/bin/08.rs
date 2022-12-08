use advent_of_code::helpers::{Direction, Grid};

type Input<'a> = Grid<u32>;

fn parse(input: &str) -> Input {
    Grid::from_str(input, &|x| x.to_digit(10).unwrap(), None)
}

pub fn part_one(grid: Input) -> Option<usize> {
    Some(
        grid.points
            .iter()
            .filter(|(a, size)| {
                grid.is_edge(a)
                    || Direction::iter()
                        .any(|dir| grid.walk(a, dir).all(|b| grid.get(&b).unwrap() < size))
            })
            .count(),
    )
}

pub fn part_two(grid: Input) -> Option<isize> {
    grid.points
        .iter()
        .filter(|(p, _)| !grid.is_edge(p))
        .map(|(point, size)| {
            Direction::iter()
                .map(|dir| {
                    let mut points = 0;
                    for point in grid.walk(point, dir) {
                        points += 1;
                        if grid.get(&point).unwrap() >= size {
                            break;
                        }
                    }
                    points
                })
                .product()
        })
        .max()
}

advent_of_code::main!(8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::read_file("examples", 8)));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::read_file("examples", 8)));
        assert_eq!(result, Some(8));
    }
}
