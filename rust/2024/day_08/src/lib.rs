use std::collections::{BTreeMap, BTreeSet};

type Coord = (isize, isize);

struct Map {
    width: isize,
    height: isize,
    frequencies: BTreeMap<char, Vec<Coord>>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let width = input.lines().next().unwrap().chars().count() as isize;
        let mut height = 0;
        let mut frequencies: BTreeMap<char, Vec<Coord>> = BTreeMap::new();
        for (line, y) in input.lines().filter(|l| !l.is_empty()).zip(0..) {
            height = y + 1;
            for (c, x) in line.chars().zip(0..) {
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

    fn iterate_frequency_pairs(&self) -> impl Iterator<Item = (char, Coord, Coord)> + '_ {
        self.frequencies.iter().flat_map(|(&frequency, antennas)| {
            antennas
                .iter()
                .enumerate()
                .flat_map(move |(i, &a)| antennas[i + 1..].iter().map(move |&b| (frequency, a, b)))
        })
    }
}

fn antinodes_of_antenna_pairs((ax, ay): Coord, (bx, by): Coord) -> [Coord; 2] {
    [(2 * ax - bx, 2 * ay - by), (2 * bx - ax, 2 * by - ay)]
}

pub fn part_1(input: &str) -> usize {
    let map = Map::parse(input);
    let mut antinodes = BTreeSet::<Coord>::new();
    for (_frequency, antenna_a, antenna_b) in map.iterate_frequency_pairs() {
        for &(antinode_x, antinode_y) in antinodes_of_antenna_pairs(antenna_a, antenna_b).iter() {
            if antinode_x >= 0
                && antinode_x < map.width
                && antinode_y >= 0
                && antinode_y < map.height
            {
                antinodes.insert((antinode_x, antinode_y));
            }
        }
    }
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
}
