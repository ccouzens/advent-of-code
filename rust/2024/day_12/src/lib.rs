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
        self.0
            .iter()
            .map(|&c| {
                DIRECTIONS
                    .iter()
                    .filter(|&&d| !self.0.contains(&(c.0 + d.0, c.1 + d.1)))
                    .count()
            })
            .sum()
    }

    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }
}

pub fn part_1(input: &str) -> usize {
    let garden = GardenMap::parse(input);
    let plots = garden.plots();
    plots.iter().map(|p| p.price()).sum()
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
}
