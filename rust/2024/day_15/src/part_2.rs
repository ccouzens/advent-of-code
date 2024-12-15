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
                    let coord = Coord([x * 2, y]);
                    match c {
                        '#' => {
                            warehouse.walls.insert(coord);
                            warehouse.walls.insert(coord + Coord([1, 0]));
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
        let mut boxes_to_move = BTreeSet::<Coord>::new();

        #[derive(PartialEq)]
        enum MoveInto {
            Wall,
            Gap,
        }

        fn move_robot_internal(
            warehouse: &Warehouse,
            start: Coord,
            direction: Coord,
            boxes_to_move: &mut BTreeSet<Coord>,
        ) -> MoveInto {
            let next_c = start + direction;
            if warehouse.walls.contains(&next_c) {
                return MoveInto::Wall;
            }

            let next_c_left_offset = next_c + Coord([-1, 0]);
            let next_c_right_offset = next_c + Coord([1, 0]);

            if direction == Coord([1, 0]) && warehouse.boxes.contains(&next_c) {
                boxes_to_move.insert(next_c);
                return move_robot_internal(
                    warehouse,
                    next_c_right_offset,
                    direction,
                    boxes_to_move,
                );
            }
            if direction == Coord([-1, 0]) && warehouse.boxes.contains(&next_c_left_offset) {
                boxes_to_move.insert(next_c_left_offset);
                return move_robot_internal(
                    warehouse,
                    next_c_left_offset,
                    direction,
                    boxes_to_move,
                );
            }

            if warehouse.boxes.contains(&next_c) {
                boxes_to_move.insert(next_c);
                if move_robot_internal(warehouse, next_c, direction, boxes_to_move)
                    == MoveInto::Wall
                {
                    return MoveInto::Wall;
                }
                return move_robot_internal(
                    warehouse,
                    next_c_right_offset,
                    direction,
                    boxes_to_move,
                );
            }

            if warehouse.boxes.contains(&next_c_left_offset) {
                boxes_to_move.insert(next_c_left_offset);
                if move_robot_internal(warehouse, next_c_left_offset, direction, boxes_to_move)
                    == MoveInto::Wall
                {
                    return MoveInto::Wall;
                }
                return move_robot_internal(warehouse, next_c, direction, boxes_to_move);
            }

            MoveInto::Gap
        }

        if move_robot_internal(self, self.robot, direction, &mut boxes_to_move) == MoveInto::Wall {
            return;
        }

        for b in boxes_to_move.iter() {
            self.boxes.remove(b);
        }
        for &b in boxes_to_move.iter() {
            self.boxes.insert(b + direction);
        }

        self.robot += direction;
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

pub fn part_2(input: &str) -> Num {
    let mut puzzle = Puzzle::new_from_input(input);
    for &m in puzzle.moves.0.iter() {
        puzzle.warehouse.move_robot(m);
    }
    puzzle.warehouse.gps_sum()
}
