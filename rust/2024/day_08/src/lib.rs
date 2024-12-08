use std::collections::{BTreeMap, BTreeSet};

type Coord = (isize, isize);

struct Map {
    width: isize,
    height: isize,
    frequencies: BTreeMap<char, Vec<Coord>>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut frequencies = BTreeMap::<char, Vec<Coord>>::new();
        for (line, y) in input.lines().filter(|l| !l.is_empty()).zip(0..) {
            height = y + 1;
            for (c, x) in line.chars().zip(0..) {
                width = x + 1;
                if c.is_ascii_alphanumeric() {
                    frequencies.entry(c).or_default().push((x, y));
                }
            }
        }
        Self {
            width,
            height,
            frequencies,
        }
    }

    fn frequency_pairs(&self) -> impl Iterator<Item = (Coord, Coord)> + '_ {
        self.frequencies.values().flat_map(|antennas| {
            antennas
                .iter()
                .enumerate()
                .flat_map(|(i, &a)| antennas[i + 1..].iter().map(move |&b| (a, b)))
        })
    }

    fn coord_inside(&self, (x, y): Coord) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn antinodes_of_antenna_pairs_with_distance(
        &self,
        (ax, ay): Coord,
        (bx, by): Coord,
    ) -> impl Iterator<Item = Coord> + '_ {
        [(2 * ax - bx, 2 * ay - by), (2 * bx - ax, 2 * by - ay)]
            .into_iter()
            .filter(|&c| self.coord_inside(c))
    }

    fn antinodes_of_antenna_pairs_in_line(
        &self,
        (ax, ay): Coord,
        (bx, by): Coord,
    ) -> impl Iterator<Item = Coord> + '_ {
        Iterator::chain(
            (0..)
                .map(move |m| (ax + m * (ax - bx), ay + m * (ay - by)))
                .take_while(|&c| self.coord_inside(c)),
            (1..)
                .map(move |m| (ax - m * (ax - bx), ay - m * (ay - by)))
                .take_while(|&c| self.coord_inside(c)),
        )
    }
}

pub fn part_1(input: &str) -> usize {
    let map = Map::parse(input);
    let antinodes = BTreeSet::from_iter(
        map.frequency_pairs()
            .flat_map(|(a, b)| map.antinodes_of_antenna_pairs_with_distance(a, b)),
    );
    antinodes.len()
}

pub fn part_2(input: &str) -> usize {
    let map = Map::parse(input);
    let antinodes = BTreeSet::from_iter(
        map.frequency_pairs()
            .flat_map(|(a, b)| map.antinodes_of_antenna_pairs_in_line(a, b)),
    );
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_2.txt")), 2);
        assert_eq!(part_1(include_str!("../example_3.txt")), 4);
        assert_eq!(part_1(include_str!("../example_1.txt")), 14);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 214);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(include_str!("../example_4.txt")), 9);
        assert_eq!(part_2(include_str!("../example_1.txt")), 34);
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 809);
    }
}
