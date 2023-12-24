use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Grid(Vec<Vec<u8>>);

impl Grid {
    pub fn new(input: &str) -> Self {
        Self(
            input
                .trim()
                .lines()
                .map(|l| l.trim().bytes().collect())
                .collect(),
        )
    }

    fn find_start(&self) -> (usize, usize) {
        for (y, line) in self.0.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                if *char == b'S' {
                    return (x, y);
                }
            }
        }
        panic!("No start token");
    }

    pub fn make_move(
        &self,
        direction: Direction,
        (initial_x, initial_y): (usize, usize),
    ) -> Option<(usize, usize)> {
        let mut new_x = initial_x;
        let mut new_y = initial_y;
        match direction {
            Direction::North => {
                new_y = new_y.checked_sub(1)?;
            }
            Direction::East => {
                new_x += 1;
            }
            Direction::South => {
                new_y += 1;
            }
            Direction::West => {
                new_x = new_x.checked_sub(1)?;
            }
        }

        let old_pipe = self.0.get(initial_y)?.get(initial_x)?;
        let new_pipe = self.0.get(new_y)?.get(new_x)?;

        if !matches!(
            (new_pipe, direction),
            (b'S', _)
                | (b'|', Direction::North | Direction::South)
                | (b'-', Direction::East | Direction::West)
                | (b'L', Direction::South | Direction::West)
                | (b'J', Direction::East | Direction::South)
                | (b'7', Direction::North | Direction::East)
                | (b'F', Direction::North | Direction::West)
        ) {
            return None;
        }

        if !matches!(
            (old_pipe, direction),
            (b'S', _)
                | (b'|', Direction::North | Direction::South)
                | (b'-', Direction::East | Direction::West)
                | (b'L', Direction::North | Direction::East)
                | (b'J', Direction::North | Direction::West)
                | (b'7', Direction::South | Direction::West)
                | (b'F', Direction::East | Direction::South)
        ) {
            return None;
        }
        Some((new_x, new_y))
    }
}

pub fn part_one(input: &str) -> u64 {
    let grid = Grid::new(input);
    let mut candidates = vec![grid.find_start()];
    let mut visited = candidates
        .iter()
        .cloned()
        .collect::<HashSet<(usize, usize)>>();
    let mut steps = 0;

    while !candidates.is_empty() {
        steps += 1;
        let mut old_candidates = std::mem::take(&mut candidates);
        while let Some(old_candidate) = old_candidates.pop() {
            for direction in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ] {
                if let Some(new_candidate) = grid.make_move(direction, old_candidate) {
                    if visited.insert(new_candidate) {
                        candidates.push(new_candidate);
                    }
                }
            }
        }
    }
    steps - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(include_str!("../example.txt")), 8)
    }

    #[test]
    #[cfg(feature = "challenge")]
    fn part_one_challenge() {
        assert_eq!(part_one(include_str!("../input.txt")), 6778)
    }
}
