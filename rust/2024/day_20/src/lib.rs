use rayon::prelude::*;
use std::{
    collections::{hash_map::Entry, HashMap},
    ops::Add,
};

type Num = i32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: Num,
    y: Num,
}

impl Coord {
    fn neighbours_within_distance(self, distance: Num) -> impl Iterator<Item = (Num, Self)> {
        (-distance..=distance).flat_map(move |dy| {
            (-distance + dy.abs()..=distance - dy.abs())
                .map(move |dx| (dy.abs() + dx.abs(), &self + [dx, dy]))
        })
    }
}

#[derive(Debug)]
struct Maze {
    size: Coord,
    walls: Vec<bool>,
    start: Coord,
    end: Coord,
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

impl Maze {
    fn parse(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut start = Coord { x: 0, y: 0 };
        let mut end = Coord { x: 0, y: 0 };
        let mut walls = Vec::new();

        for (line, y) in input.lines().filter(|l| !l.is_empty()).zip(0..) {
            height = y + 1;
            for (c, x) in line.chars().zip(0..) {
                width = Num::max(width, x + 1);
                walls.push(c == '#');
                if c == 'S' {
                    start = Coord { x, y };
                }
                if c == 'E' {
                    end = Coord { x, y };
                }
            }
        }

        Self {
            start,
            end,
            walls,
            size: Coord {
                x: width,
                y: height,
            },
        }
    }

    fn is_wall(&self, coord: &Coord) -> bool {
        coord.x < 0
            || coord.y < 0
            || coord.x >= self.size.x
            || coord.y >= self.size.y
            || self.walls[(coord.x + self.size.x * coord.y) as usize]
    }

    fn distances_from_point(&self, point: Coord) -> HashMap<Coord, Num> {
        let mut distances: HashMap<Coord, Num> = HashMap::new();
        let mut explore_next = vec![point];
        for distance in 0.. {
            for &c in std::mem::take(&mut explore_next).iter() {
                if let Entry::Vacant(v) = distances.entry(c) {
                    v.insert(distance);
                    for (_, n) in c.neighbours_within_distance(1) {
                        if !self.is_wall(&n) {
                            explore_next.push(n);
                        }
                    }
                }
            }
            if explore_next.is_empty() {
                break;
            }
        }
        distances
    }
}

pub fn puzzle(input: &str, required_saving: Num, skip_distance: Num) -> Num {
    let maze = Maze::parse(input);
    let distances_from_start = maze.distances_from_point(maze.start);
    let distances_from_end = maze.distances_from_point(maze.end);
    let regular_distance = distances_from_start[&maze.end];

    distances_from_end
        .par_iter()
        .map(|(c, &d)| {
            let mut cheat_counts = 0;
            for (skipped_distance, n) in c.neighbours_within_distance(skip_distance) {
                if let Some(&sd) = distances_from_start.get(&n) {
                    if regular_distance - d - sd - skipped_distance - required_saving >= 0 {
                        cheat_counts += 1;
                    }
                }
            }
            cheat_counts
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(puzzle(include_str!("../example_1.txt"), 12, 2), 8);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(puzzle(include_str!("../input.txt"), 100, 2), 1395);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(puzzle(include_str!("../example_1.txt"), 72, 20), 29);
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(puzzle(include_str!("../input.txt"), 100, 20), 993178);
    }
}
