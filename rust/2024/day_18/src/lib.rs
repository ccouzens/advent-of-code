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
            && !self.walls.contains(location)
    }

    fn tick(&mut self) -> bool {
        if let Some(&wall) = self.maze_input.walls.get(self.time) {
            self.walls.insert(wall);
            self.time += 1;
            true
        } else {
            false
        }
    }

    fn time_to_end(&self) -> Option<usize> {
        let mut visited = BTreeSet::new();
        let mut to_visit_next = BTreeSet::new();
        to_visit_next.insert(Coord { x: 0, y: 0 });

        let mut step_count = 0;

        loop {
            if to_visit_next.is_empty() {
                return None;
            }
            let to_visit = std::mem::take(&mut to_visit_next);
            for &c in to_visit.iter() {
                if c.x == self.size.x - 1 && c.y == self.size.y - 1 {
                    return Some(step_count);
                }
                if visited.insert(c) {
                    to_visit_next.extend(c.neighbours().iter().filter(|&n| self.is_passable(n)));
                }
            }
            step_count += 1;
        }
    }
}

pub fn part_1(input: &str, size: Coord, time: usize) -> Option<usize> {
    let maze = Maze::new(input, size, time);
    maze.time_to_end()
}

pub fn part_2(input: &str, size: Coord, time: usize) -> Option<Coord> {
    let mut maze = Maze::new(input, size, time);
    while maze.time_to_end().is_some() {
        if !maze.tick() {
            return None;
        }
    }
    maze.maze_input.walls.get(maze.time - 1).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(
            part_1(include_str!("../example_1.txt"), Coord { x: 7, y: 7 }, 12),
            Some(22)
        );
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(
            part_1(include_str!("../input.txt"), Coord { x: 71, y: 71 }, 1024),
            Some(380)
        );
    }

    #[test]
    fn example_part_2() {
        assert_eq!(
            part_2(include_str!("../example_1.txt"), Coord { x: 7, y: 7 }, 0),
            Some(Coord { x: 6, y: 1 })
        );
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(
            part_2(include_str!("../input.txt"), Coord { x: 71, y: 71 }, 0),
            Some(Coord { x: 26, y: 50 })
        );
    }
}
