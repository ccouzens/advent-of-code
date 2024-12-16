use std::{
    cmp::{Ordering, Reverse},
    collections::{BTreeSet, BinaryHeap},
    ops::Add,
};

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn rotate_right(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    fn rotate_left(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Maze {
    start: Coord,
    end: Coord,
    walls: BTreeSet<Coord>,
}

impl Maze {
    fn parse(input: &str) -> Self {
        let mut start = Coord::default();
        let mut end = Coord::default();
        let mut walls = BTreeSet::new();
        for (line, y) in input.lines().zip(0..) {
            for (c, x) in line.chars().zip(0..) {
                let coord = Coord { x, y };
                match c {
                    '#' => {
                        walls.insert(coord);
                    }
                    'E' => {
                        end = coord;
                    }
                    'S' => {
                        start = coord;
                    }
                    _ => {}
                }
            }
        }
        Self { start, end, walls }
    }

    fn solve(&self) -> isize {
        #[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
        struct Node {
            position: Coord,
            direction: Coord,
        }
        #[derive(Debug, PartialEq, Eq)]
        struct HeapData {
            score: isize,
            node: Node,
        }

        impl PartialOrd for HeapData {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for HeapData {
            fn cmp(&self, other: &Self) -> Ordering {
                Reverse(self.score)
                    .cmp(&Reverse(other.score))
                    .then(self.node.cmp(&other.node))
            }
        }

        let mut visited = BTreeSet::new();

        let mut unvisited = BinaryHeap::<HeapData>::new();
        unvisited.push(HeapData {
            score: 0,
            node: Node {
                position: self.start,
                direction: Coord { x: 1, y: 0 },
            },
        });

        while let Some(candidate) = unvisited.pop() {
            if candidate.node.position == self.end {
                return candidate.score;
            }

            if !visited.insert(candidate.node) {
                continue;
            }

            unvisited.push(HeapData {
                score: 1000 + candidate.score,
                node: Node {
                    position: candidate.node.position,
                    direction: candidate.node.direction.rotate_right(),
                },
            });

            unvisited.push(HeapData {
                score: 1000 + candidate.score,
                node: Node {
                    position: candidate.node.position,
                    direction: candidate.node.direction.rotate_left(),
                },
            });

            let forward = candidate.node.position + candidate.node.direction;
            if !self.walls.contains(&forward) {
                unvisited.push(HeapData {
                    score: 1 + candidate.score,
                    node: Node {
                        position: candidate.node.position + candidate.node.direction,
                        direction: candidate.node.direction,
                    },
                });
            }
        }

        0
    }
}

pub fn part_1(input: &str) -> isize {
    let maze = Maze::parse(input);
    maze.solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 7036);
        assert_eq!(part_1(include_str!("../example_2.txt")), 11048);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 114476);
    }
}
