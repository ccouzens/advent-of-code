use std::collections::BTreeSet;

#[derive(Debug)]
struct GardenMap(Vec<Vec<char>>);

type Coord = (isize, isize);

const DIRECTIONS: [Coord; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Debug)]
struct Plot(BTreeSet<Coord>);

impl GardenMap {
    fn parse(input: &str) -> Self {
        Self(input.lines().map(|line| line.chars().collect()).collect())
    }

    fn get(&self, c: Coord) -> Option<char> {
        self.0
            .get(c.1 as usize)
            .and_then(|l| l.get(c.0 as usize).copied())
    }

    fn plots(&self) -> Vec<Plot> {
        let mut allocated = BTreeSet::<Coord>::new();
        let mut plots = Vec::new();
        for (line, y) in self.0.iter().zip(0..) {
            for (&t, x) in line.iter().zip(0..) {
                if allocated.insert((x, y)) {
                    let mut plot = BTreeSet::new();
                    let mut stack = vec![(x, y)];
                    while let Some(c) = stack.pop() {
                        plot.insert(c);
                        for &d in DIRECTIONS.iter() {
                            let new_location = (c.0 + d.0, c.1 + d.1);
                            if self.get(new_location) == Some(t) && allocated.insert(new_location) {
                                stack.push(new_location);
                            }
                        }
                    }

                    plots.push(Plot(plot));
                }
            }
        }
        plots
    }
}

impl Plot {
    fn area(&self) -> usize {
        self.0.len()
    }

    fn perimeter(&self) -> usize {
        DIRECTIONS
            .iter()
            .map(|&d| {
                self.0
                    .iter()
                    .filter(|&&c| !self.0.contains(&(c.0 + d.0, c.1 + d.1)))
                    .count()
            })
            .sum()
    }

    fn number_of_sides(&self) -> usize {
        let mut count = 0;
        for &direction in DIRECTIONS.iter() {
            let mut considered = BTreeSet::new();
            let turn_a = (-direction.1, direction.0);
            let turn_b = (direction.1, -direction.0);
            for &c in self.0.iter() {
                if considered.insert(c) && !self.0.contains(&(c.0 + direction.0, c.1 + direction.1))
                {
                    for &turn in &[turn_a, turn_b] {
                        for i in 0.. {
                            let neighbour = (c.0 + turn.0 * i, c.1 + turn.1 * i);
                            if !self.0.contains(&neighbour) {
                                break;
                            }
                            if self
                                .0
                                .contains(&(neighbour.0 + direction.0, neighbour.1 + direction.1))
                            {
                                break;
                            }
                            considered.insert(neighbour);
                        }
                    }
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn part_1(input: &str) -> usize {
    let garden = GardenMap::parse(input);
    let plots = garden.plots();
    plots.iter().map(|p| p.area() * p.perimeter()).sum()
}

pub fn part_2(input: &str) -> usize {
    let garden = GardenMap::parse(input);
    let plots = garden.plots();
    plots.iter().map(|p| p.area() * p.number_of_sides()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 140);
        assert_eq!(part_1(include_str!("../example_2.txt")), 772);
        assert_eq!(part_1(include_str!("../example_3.txt")), 1930);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 1485656);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(include_str!("../example_1.txt")), 80);
        assert_eq!(part_2(include_str!("../example_2.txt")), 436);
        assert_eq!(part_2(include_str!("../example_4.txt")), 236);
        assert_eq!(part_2(include_str!("../example_5.txt")), 368);
        assert_eq!(part_2(include_str!("../example_3.txt")), 1206);
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 899196);
    }
}
