use std::collections::BTreeSet;

#[derive(Debug)]
struct Map {
    width: isize,
    height: isize,
    topology: Vec<i8>,
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl Map {
    fn parse(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut topology = Vec::new();
        for (line, y) in input.lines().zip(0..) {
            height = y + 1;
            for (c, x) in line.bytes().zip(0..) {
                width = x + 1;
                topology.push((c - b'0') as i8);
            }
        }
        Self {
            width,
            height,
            topology,
        }
    }

    fn get(&self, (x, y): (isize, isize)) -> Option<i8> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.topology.get((y * self.width + x) as usize).copied()
        } else {
            None
        }
    }

    fn find_trailheads(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        self.topology
            .iter()
            .zip(0..)
            .filter(|&(&t, _i)| (t == 0))
            .map(|(_t, i)| (i % self.width, i / self.width))
    }

    fn trailhead_score(&self, trailhead: (isize, isize)) -> usize {
        let mut peaks = BTreeSet::new();
        let mut stack = vec![trailhead];
        while let Some(v) = stack.pop() {
            let height = self.get(v).unwrap();
            for &d in DIRECTIONS.iter() {
                let neighbour = (v.0 + d.0, v.1 + d.1);
                if self.get(neighbour) == Some(height + 1) {
                    stack.push(neighbour)
                }
            }
            if height == 9 {
                peaks.insert(v);
            }
        }
        peaks.len()
    }
}

pub fn part_1(input: &str) -> usize {
    let map = Map::parse(input);
    map.find_trailheads()
        .map(|trailhead| map.trailhead_score(trailhead))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 1);
        assert_eq!(part_1(include_str!("../example_2.txt")), 36);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 472);
    }
}
