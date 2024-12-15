use std::{
    collections::BTreeSet,
    ops::{Add, AddAssign, Mul},
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    character::complete::{char, newline},
    combinator::{map, opt},
    multi::many1,
    sequence::{separated_pair, terminated},
};

type Num = isize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord([Num; 2]);

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self([self.0[0] + other.0[0], self.0[1] + other.0[1]])
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        *self = Self([self.0[0] + other.0[0], self.0[1] + other.0[1]])
    }
}

impl Mul<Num> for Coord {
    type Output = Self;

    fn mul(self, other: Num) -> Self {
        Self([self.0[0] * other, self.0[1] * other])
    }
}

#[derive(Debug)]
struct Warehouse {
    robot: Coord,
    walls: BTreeSet<Coord>,
    boxes: BTreeSet<Coord>,
}

impl Warehouse {
    fn parse<'a>(
    ) -> impl FnMut(&'a str) -> Result<(&'a str, Self), nom::Err<nom::error::Error<&'a str>>> {
        map(take_until1("\n\n"), |input: &'a str| {
            let mut warehouse = Warehouse {
                robot: Coord([0, 0]),
                walls: Default::default(),
                boxes: Default::default(),
            };

            for (line, y) in input.lines().zip(0..) {
                for (c, x) in line.chars().zip(0..) {
                    let coord = Coord([x, y]);
                    match c {
                        '#' => {
                            warehouse.walls.insert(coord);
                        }
                        'O' => {
                            warehouse.boxes.insert(coord);
                        }
                        '@' => {
                            warehouse.robot = coord;
                        }
                        _ => {}
                    }
                }
            }

            warehouse
        })
    }

    fn move_robot(&mut self, direction: Coord) {
        let mut hops = 1;
        loop {
            let c = self.robot + direction * hops;
            if self.walls.contains(&c) {
                return;
            }
            if !self.boxes.contains(&c) {
                if hops > 1 {
                    self.boxes.remove(&(self.robot + direction));
                    self.boxes.insert(self.robot + direction * hops);
                }
                self.robot += direction;
                return;
            }
            hops += 1;
        }
    }

    fn gps_sum(&self) -> Num {
        self.boxes.iter().map(|b| b.0[0] + 100 * b.0[1]).sum()
    }
}

#[derive(Debug)]
struct Moves(Vec<Coord>);

impl Moves {
    fn parse<'a>(
    ) -> impl FnMut(&'a str) -> Result<(&'a str, Self), nom::Err<nom::error::Error<&'a str>>> {
        map(
            many1(terminated(
                alt((
                    map(char('<'), |_| Coord([-1, 0])),
                    map(char('>'), |_| Coord([1, 0])),
                    map(char('^'), |_| Coord([0, -1])),
                    map(char('v'), |_| Coord([0, 1])),
                )),
                opt(newline),
            )),
            Self,
        )
    }
}

#[derive(Debug)]
struct Puzzle {
    moves: Moves,
    warehouse: Warehouse,
}

impl Puzzle {
    fn parse<'a>(
    ) -> impl FnMut(&'a str) -> Result<(&'a str, Self), nom::Err<nom::error::Error<&'a str>>> {
        map(
            separated_pair(Warehouse::parse(), tag("\n\n"), Moves::parse()),
            |(warehouse, moves)| Self { warehouse, moves },
        )
    }

    fn new_from_input(input: &str) -> Self {
        Self::parse()(input).unwrap().1
    }
}

pub fn part_1(input: &str) -> Num {
    let mut puzzle = Puzzle::new_from_input(input);
    for &m in puzzle.moves.0.iter() {
        puzzle.warehouse.move_robot(m);
    }
    puzzle.warehouse.gps_sum()
}
