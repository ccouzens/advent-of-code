use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Cell(u8);

impl Cell {
    fn goes_direction(&self, direction: Direction) -> bool {
        matches!(
            (self.0, direction),
            (b'|', Direction::North | Direction::South)
                | (b'-', Direction::East | Direction::West)
                | (b'L', Direction::North | Direction::East)
                | (b'J', Direction::North | Direction::West)
                | (b'7', Direction::South | Direction::West)
                | (b'F', Direction::South | Direction::East)
        )
    }
}

pub struct Grid(Vec<Vec<Cell>>);

impl Grid {
    pub fn new(input: &str) -> Self {
        Self(
            input
                .trim()
                .lines()
                .map(|l| l.trim().bytes().map(Cell).collect())
                .collect(),
        )
    }

    fn find_start(&self) -> (usize, usize) {
        for (y, line) in self.0.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if cell.0 == b'S' {
                    return (x, y);
                }
            }
        }
        panic!("No start token");
    }

    fn normalise_start(&mut self) -> (usize, usize) {
        let (start_x, start_y) = self.find_start();
        let connects_north = start_y
            .checked_sub(1)
            .and_then(|y| self.0.get(y)?.get(start_x))
            .map(|c| c.goes_direction(Direction::South))
            .unwrap_or(false);
        let connects_east = self
            .0
            .get(start_y)
            .and_then(|line| line.get(start_x + 1))
            .map(|c| c.goes_direction(Direction::West))
            .unwrap_or(false);
        let connects_south = self
            .0
            .get(start_y + 1)
            .and_then(|line| line.get(start_x))
            .map(|c| c.goes_direction(Direction::North))
            .unwrap_or(false);
        let connects_west = start_x
            .checked_sub(1)
            .and_then(|x| self.0.get(start_y)?.get(x))
            .map(|c| c.goes_direction(Direction::East))
            .unwrap_or(false);
        self.0[start_y][start_x] = Cell(
            match (connects_north, connects_east, connects_south, connects_west) {
                (true, false, true, false) => b'|',
                (false, true, false, true) => b'-',
                (true, true, false, false) => b'L',
                (true, false, false, true) => b'J',
                (false, false, true, true) => b'7',
                (false, true, true, false) => b'F',
                _ => b'.',
            },
        );
        (start_x, start_y)
    }

    fn extract_loop(mut self) -> BTreeMap<(usize, usize), Cell> {
        let start = self.normalise_start();

        let mut candidates = vec![start];
        let mut visited = candidates
            .iter()
            .cloned()
            .map(|(x, y)| ((x, y), self.0[y][x]))
            .collect::<BTreeMap<(usize, usize), Cell>>();

        while !candidates.is_empty() {
            let mut old_candidates = std::mem::take(&mut candidates);
            while let Some(old_candidate) = old_candidates.pop() {
                for direction in [
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ] {
                    if let Some(new_candidate) = self.make_move(direction, old_candidate) {
                        if visited
                            .insert(new_candidate, self.0[new_candidate.1][new_candidate.0])
                            .is_none()
                        {
                            candidates.push(new_candidate);
                        }
                    }
                }
            }
        }
        visited
    }

    pub fn make_move(
        &self,
        direction: Direction,
        (initial_x, initial_y): (usize, usize),
    ) -> Option<(usize, usize)> {
        let old_pipe = self.0.get(initial_y)?.get(initial_x)?;
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

        if old_pipe.goes_direction(direction) {
            Some((new_x, new_y))
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> u64 {
    let grid = Grid::new(input);

    let pipe_loop = grid.extract_loop();

    for y in 0..150 {
        for x in 0..150 {
            print!(
                "{}",
                char::from(pipe_loop.get(&(x, y)).map(|c| c.0).unwrap_or(b' '))
            );
        }
        println!();
    }
    (pipe_loop.len() / 2).try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(include_str!("../example-1.txt")), 8)
    }

    #[test]
    #[cfg(feature = "challenge")]
    fn part_one_challenge() {
        assert_eq!(part_one(include_str!("../input.txt")), 6778)
    }
}
