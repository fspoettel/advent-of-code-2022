use indextree::{Arena, NodeId};

#[derive(Clone)]
pub struct Entry<'a> {
    name: &'a str,
    size: u32,
}

type Input<'a> = Arena<Entry<'a>>;

fn parse(input: &str) -> Input {
    let mut arena = Arena::new();
    let mut current_id = arena.new_node(Entry { name: "/", size: 0 });

    input
        .split("$ ")
        .skip(2)
        .map(|chunk| {
            let (cmd, rest) = chunk.split_at(2);
            (cmd, rest.trim())
        })
        .try_for_each(|cmd| {
            match cmd {
                ("cd", "..") => {
                    current_id = arena.get(current_id)?.parent()?;
                }
                ("cd", dir) => {
                    current_id = current_id
                        .children(&arena)
                        .find(|id| arena.get(*id).unwrap().get().name == dir)?;
                }
                ("ls", rest) => {
                    rest.lines().try_for_each(|l| {
                        let (size, name) = l.split_once(' ')?;
                        if size == "dir" {
                            let id = arena.new_node(Entry { name, size: 0 });
                            current_id.append(id, &mut arena);
                        } else {
                            let size = size.parse::<u32>().ok()?;
                            current_id
                                .ancestors(&arena)
                                .collect::<Vec<NodeId>>()
                                .into_iter()
                                .for_each(|id| {
                                    arena.get_mut(id).unwrap().get_mut().size += size;
                                })
                        }
                        Some(())
                    });
                }
                _ => unreachable!(),
            }

            Some(())
        });

    arena
}

pub fn part_one(arena: Input) -> Option<u32> {
    Some(
        arena
            .iter()
            .map(|entry| entry.get().size)
            .filter(|size| *size < 100000)
            .sum(),
    )
}

pub fn part_two(arena: Input) -> Option<u32> {
    let mut values = arena.iter().map(|entry| entry.get().size);
    let total_size = values.next()?;
    let needed = total_size + 30000000 - 70000000;
    values.filter(|x| *x >= needed).min()
}

advent_of_code::main!(7);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::read_file("examples", 7)));
        assert_eq!(result, Some(95437));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::read_file("examples", 7)));
        assert_eq!(result, Some(24933642));
    }
}
