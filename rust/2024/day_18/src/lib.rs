use std::{collections::BTreeSet, ops::Add};

use nom::{
    character::complete::{char, digit1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
};

type Num = i64;

fn parser_num<'a>(
) -> impl FnMut(&'a str) -> Result<(&'a str, Num), nom::Err<nom::error::Error<&'a str>>> {
    map_res(digit1, str::parse)
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Coord {
    x: Num,
    y: Num,
}
impl Add<[Num; 2]> for &Coord {
    type Output = Coord;

    fn add(self, other: [Num; 2]) -> Coord {
        Coord {
            x: self.x + other[0],
            y: self.y + other[1],
        }
    }
}

impl Coord {
    fn parser<'a>(
    ) -> impl FnMut(&'a str) -> Result<(&'a str, Self), nom::Err<nom::error::Error<&'a str>>> {
        map(
            separated_pair(parser_num(), char(','), parser_num()),
            |(x, y)| Self { x, y },
        )
    }

    fn neighbours(&self) -> [Self; 4] {
        [self + [0, 1], self + [-1, 0], self + [1, 0], self + [0, -1]]
    }
}

#[derive(Debug)]
struct MazeInput {
    walls: Vec<Coord>,
}

impl MazeInput {
    fn parser<'a>(
    ) -> impl FnMut(&'a str) -> Result<(&'a str, Self), nom::Err<nom::error::Error<&'a str>>> {
        map(separated_list1(newline, Coord::parser()), |walls| Self {
            walls,
        })
    }

    fn parse(input: &str) -> Self {
        Self::parser()(input).unwrap().1
    }
}

struct Maze {
    maze_input: MazeInput,
    time: usize,
    walls: BTreeSet<Coord>,
    size: Coord,
}

impl Maze {
    fn new(input: &str, size: Coord, time: usize) -> Self {
        let maze_input = MazeInput::parse(input);
        Self {
            walls: maze_input.walls.iter().copied().take(time).collect(),
            maze_input,
            time,
            size,
        }
    }

    fn is_passable(&self, location: &Coord) -> bool {
        location.x >= 0
            && location.y >= 0
            && location.x < self.size.x
            && location.y < self.size.y
            && !self.walls.contains(&location)
    }
}

pub fn part_1(input: &str, size: Coord, time: usize) -> usize {
    let maze = Maze::new(input, size, time);
    let mut visited = BTreeSet::new();
    let mut to_visit_next = BTreeSet::new();
    to_visit_next.insert(Coord { x: 0, y: 0 });

    let mut step_count = 0;

    loop {
        let to_visit = std::mem::take(&mut to_visit_next);
        for &c in to_visit.iter() {
            if c.x == maze.size.x - 1 && c.y == maze.size.y - 1 {
                return step_count;
            }
            if visited.insert(c) {
                to_visit_next.extend(c.neighbours().iter().filter(|&n| maze.is_passable(n)));
            }
        }
        step_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(
            part_1(include_str!("../example_1.txt"), Coord { x: 7, y: 7 }, 12),
            22
        );
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(
            part_1(include_str!("../input.txt"), Coord { x: 71, y: 71 }, 1024),
            380
        );
    }
}
