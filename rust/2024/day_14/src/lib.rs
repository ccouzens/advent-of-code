use std::iter::repeat_n;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, newline},
    combinator::{map, map_res, opt},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

type Num = i64;
type Coord = [Num; 2];

#[derive(Debug)]
struct Robot {
    position: Coord,
    velocity: Coord,
}

#[derive(Debug)]
struct World {
    size: Coord,
    robots: Vec<Robot>,
}

impl Robot {
    fn parse<'a>(
    ) -> impl FnMut(&'a str) -> Result<(&'a str, Self), nom::Err<nom::error::Error<&'a str>>> {
        map(
            tuple((tag("p="), parse_coord(), tag(" v="), parse_coord())),
            |(_, position, _, velocity)| Robot { position, velocity },
        )
    }

    fn step(&mut self, world_size: Coord) {
        self.position = [
            Num::rem_euclid(self.position[0] + self.velocity[0], world_size[0]),
            Num::rem_euclid(self.position[1] + self.velocity[1], world_size[1]),
        ];
    }
}

fn parse_num<'a>(
) -> impl FnMut(&'a str) -> Result<(&'a str, Num), nom::Err<nom::error::Error<&'a str>>> {
    map(
        tuple((opt(char('-')), map_res(digit1, str::parse))),
        |(sign, num)| if sign.is_some() { 0 - num } else { num },
    )
}

fn parse_coord<'a>(
) -> impl FnMut(&'a str) -> Result<(&'a str, Coord), nom::Err<nom::error::Error<&'a str>>> {
    map(
        separated_pair(parse_num(), char(','), parse_num()),
        |(a, b)| [a, b],
    )
}

impl World {
    fn parse(size: Coord, input: &str) -> Self {
        let raw: IResult<_, _> = separated_list1(newline, Robot::parse())(input);

        Self {
            size,
            robots: raw.unwrap().1,
        }
    }

    fn step(&mut self) {
        for robot in self.robots.iter_mut() {
            robot.step(self.size);
        }
    }

    fn safety_factor(&self) -> usize {
        let mut product = 1;
        for x_range in [0..self.size[0] / 2, self.size[0] / 2 + 1..self.size[0]] {
            for y_range in [0..self.size[1] / 2, self.size[1] / 2 + 1..self.size[1]] {
                let mut count = 0;

                for robot in self.robots.iter() {
                    if x_range.contains(&robot.position[0]) && y_range.contains(&robot.position[1])
                    {
                        count += 1;
                    }
                }

                product *= count;
            }
        }

        product
    }

    fn draw(&self) {
        let width = self.size[0];
        let height = self.size[1];
        let mut bitmap: Vec<usize> = repeat_n(0, (width * height) as usize).collect();
        for robot in self.robots.iter() {
            bitmap[(robot.position[0] + robot.position[1] * width) as usize] += 1
        }

        for (&b, i) in bitmap.iter().zip(0..) {
            if b == 0 {
                print!(" ");
            } else {
                print!("{}", b);
            }

            if i % width == width - 1 {
                println!();
            }
        }
    }

    fn has_line_of_robots(&self) -> bool {
        let width = self.size[0];
        let height = self.size[1];
        let mut bitmap: Vec<usize> = repeat_n(0, (width * height) as usize).collect();
        for robot in self.robots.iter() {
            bitmap[(robot.position[0] + robot.position[1] * width) as usize] += 1
        }

        let mut uninterupted_line = 0;
        for &b in bitmap.iter() {
            if b == 0 {
                uninterupted_line = 0;
            } else {
                uninterupted_line += 1
            }

            if uninterupted_line >= 10 {
                return true;
            }
        }
        false
    }
}

pub fn part_1(size: Coord, input: &str) -> usize {
    let mut world = World::parse(size, input);

    for _ in 0..100 {
        world.step();
    }

    world.safety_factor()
}

pub fn part_2(size: Coord, input: &str) -> usize {
    let mut world = World::parse(size, input);

    for i in 0.. {
        if world.has_line_of_robots() {
            world.draw();
            return i;
        }
        world.step();
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1([11, 7], include_str!("../example_1.txt")), 12);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1([101, 103], include_str!("../input.txt")), 215476074);
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(part_2([101, 103], include_str!("../input.txt")), 6285);
    }
}
