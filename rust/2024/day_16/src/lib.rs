use std::{
    cmp::{Ordering, Reverse},
    collections::{btree_map::Entry, BTreeMap, BTreeSet, BinaryHeap},
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

    fn lowest_score(&self) -> isize {
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

            for n in candidate.edges(self) {
                unvisited.push(n);
            }
        }

        0
    }

    fn seats(&self) -> usize {
        let mut shortest_path_from_start = BTreeMap::<Node, isize>::new();

        let mut unvisited = BinaryHeap::<HeapData>::new();
        unvisited.push(HeapData {
            score: 0,
            node: Node {
                position: self.start,
                direction: Coord { x: 1, y: 0 },
            },
        });

        while let Some(candidate) = unvisited.pop() {
            match shortest_path_from_start.entry(candidate.node) {
                Entry::Vacant(vacant_entry) => vacant_entry.insert(candidate.score),
                Entry::Occupied(_) => {
                    continue;
                }
            };
            if candidate.node.position == self.end {
                break;
            }

            for n in candidate.edges(self) {
                unvisited.push(n);
            }
        }
        let mut shortest_path_from_end = BTreeMap::<Node, isize>::new();
        {
            let mut direction = Coord { x: 1, y: 0 };
            for _ in 0..4 {
                unvisited.push(HeapData {
                    score: 0,
                    node: Node {
                        position: self.end,
                        direction,
                    },
                });
                direction = direction.rotate_right();
            }
        }

        while let Some(candidate) = unvisited.pop() {
            match shortest_path_from_end.entry(candidate.node) {
                Entry::Vacant(vacant_entry) => vacant_entry.insert(candidate.score),
                Entry::Occupied(_) => {
                    continue;
                }
            };
            if candidate.node.position == self.start {
                break;
            }

            for n in candidate.edges(self) {
                unvisited.push(n);
            }
        }

        let mut on_shortest_path = BTreeSet::new();

        let &shortest_path = shortest_path_from_start
            .iter()
            .find_map(|(node, d)| (node.position == self.end).then_some(d))
            .unwrap();
        for (node, &d) in shortest_path_from_start.iter() {
            match shortest_path_from_end.get(&Node {
                position: node.position,
                direction: node.direction.rotate_right().rotate_right(),
            }) {
                None => {}
                Some(od) => {
                    if od + d == shortest_path {
                        on_shortest_path.insert(node.position);
                    }
                }
            }
        }
        on_shortest_path.len()
    }
}

impl HeapData {
    fn edges<'a>(&'a self, maze: &'a Maze) -> impl Iterator<Item = HeapData> + 'a {
        (0..3).filter_map(|i| match i {
            0 => Some(HeapData {
                score: 1000 + self.score,
                node: Node {
                    position: self.node.position,
                    direction: self.node.direction.rotate_right(),
                },
            }),
            1 => Some(HeapData {
                score: 1000 + self.score,
                node: Node {
                    position: self.node.position,
                    direction: self.node.direction.rotate_left(),
                },
            }),
            2 => {
                let forward = self.node.position + self.node.direction;
                (!maze.walls.contains(&forward)).then(|| HeapData {
                    score: 1 + self.score,
                    node: Node {
                        position: forward,
                        direction: self.node.direction,
                    },
                })
            }
            _ => None,
        })
    }
}

pub fn part_1(input: &str) -> isize {
    let maze = Maze::parse(input);
    maze.lowest_score()
}

pub fn part_2(input: &str) -> usize {
    let maze = Maze::parse(input);
    maze.seats()
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

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(include_str!("../example_1.txt")), 45);
        assert_eq!(part_2(include_str!("../example_2.txt")), 64);
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 508);
    }
}
