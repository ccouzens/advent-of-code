struct Grid(Vec<Vec<char>>);

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Forward,
    Still,
    Backward,
}

const DIRECTIONS: &[(Direction, Direction)] = &[
    (Direction::Forward, Direction::Forward),
    (Direction::Forward, Direction::Still),
    (Direction::Forward, Direction::Backward),
    (Direction::Still, Direction::Forward),
    (Direction::Still, Direction::Backward),
    (Direction::Backward, Direction::Forward),
    (Direction::Backward, Direction::Still),
    (Direction::Backward, Direction::Backward),
];

impl Direction {
    fn follow(&self, original: usize, offset: usize) -> Option<usize> {
        match self {
            Direction::Forward => original.checked_add(offset),
            Direction::Still => Some(original),
            Direction::Backward => original.checked_sub(offset),
        }
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        Grid(input.lines().map(|line| line.chars().collect()).collect())
    }

    fn get(
        &self,
        x: usize,
        y: usize,
        offset: usize,
        horizontal_direction: Direction,
        vertical_direction: Direction,
    ) -> Option<char> {
        let adjusted_x = horizontal_direction.follow(x, offset);
        let adjusted_y = vertical_direction.follow(y, offset);
        if let (Some(adjusted_x), Some(adjusted_y)) = (adjusted_x, adjusted_y) {
            self.0
                .get(adjusted_y)
                .and_then(|line| line.get(adjusted_x))
                .copied()
        } else {
            None
        }
    }

    fn iterate_positions(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, line)| (0..line.len()).map(move |x| (x, y)))
    }
}

pub fn part_1(input: &str) -> usize {
    let grid = Grid::new(input);
    grid.iterate_positions()
        .map(|(x, y)| {
            DIRECTIONS
                .iter()
                .filter(|(horizontal_direction, vertical_direction)| {
                    "XMAS".chars().enumerate().all(|(offset, letter)| {
                        grid.get(x, y, offset, *horizontal_direction, *vertical_direction)
                            == Some(letter)
                    })
                })
                .count()
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let grid = Grid::new(input);
    grid.iterate_positions()
        .filter(|&(x, y)| {
            matches!(
                (
                    grid.get(x, y, 0, Direction::Still, Direction::Still),
                    (
                        grid.get(x, y, 1, Direction::Forward, Direction::Forward),
                        grid.get(x, y, 1, Direction::Backward, Direction::Backward),
                    ),
                    (
                        grid.get(x, y, 1, Direction::Forward, Direction::Backward),
                        grid.get(x, y, 1, Direction::Backward, Direction::Forward),
                    ),
                ),
                (
                    Some('A'),
                    (Some('M'), Some('S')) | (Some('S'), Some('M')),
                    (Some('M'), Some('S')) | (Some('S'), Some('M')),
                )
            )
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 18);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 2549);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(include_str!("../example_1.txt")), 9);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 2003);
    }
}
