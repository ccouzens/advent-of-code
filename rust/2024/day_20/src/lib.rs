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
    fn neighbours(&self) -> [Self; 4] {
        [self + [0, 1], self + [-1, 0], self + [1, 0], self + [0, -1]]
    }

    fn jump_1_neighbours(&self) -> [Self; 8] {
        [
            self + [0, -2],
            self + [1, -1],
            self + [2, 0],
            self + [1, 1],
            self + [0, 2],
            self + [-1, 1],
            self + [-2, 0],
            self + [-1, -1],
        ]
    }

    fn jump_2_neighbours(&self) -> [Self; 12] {
        [
            self + [0, -3],
            self + [1, -2],
            self + [2, -1],
            self + [3, 0],
            self + [2, 1],
            self + [1, 2],
            self + [0, 3],
            self + [-1, 2],
            self + [-2, 1],
            self + [-3, 0],
            self + [-2, -1],
            self + [-1, -2],
        ]
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
                    for n in c.neighbours() {
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

pub fn part_1(input: &str, required_saving: Num) -> Num {
    let maze = Maze::parse(input);
    let distances_from_start = maze.distances_from_point(maze.start);
    let distances_from_end = maze.distances_from_point(maze.end);
    let regular_distance = distances_from_start[&maze.end];
    let mut cheat_counts = 0;

    for (c, d) in distances_from_end.iter() {
        for n in c.jump_1_neighbours().iter() {
            if let Some(&sd) = distances_from_start.get(n) {
                let jump_1_improvement = regular_distance - d - sd - 2;
                if jump_1_improvement >= required_saving {
                    cheat_counts += 1;
                }
            }
        }
    }
    cheat_counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt"), 12), 8);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt"), 100), 1395);
    }
}
