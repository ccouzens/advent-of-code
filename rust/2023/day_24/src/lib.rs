use std::ops::Range;

#[derive(Debug)]
struct Hailstone {
    loc: [f64; 3],
    velocity: [f64; 3],
    two_d_m: f64,
    two_d_c: f64,
}

impl Hailstone {
    fn new(input: &str) -> Option<Self> {
        let mut split = input
            .split(|c: char| !c.is_ascii_digit() && c != '-')
            .filter_map(|n| n.parse().ok());
        let loc = [split.next()?, split.next()?, split.next()?];
        let velocity = [split.next()?, split.next()?, split.next()?];
        Some(Self {
            loc,
            velocity,
            two_d_m: velocity[1] / velocity[0],
            two_d_c: -loc[0] * velocity[1] / velocity[0] + loc[1],
        })
    }

    fn two_d_paths_intersect(&self, other: &Self) -> Option<(f64, f64)> {
        if self.two_d_m == other.two_d_m {
            return None;
        }
        let x = (other.two_d_c - self.two_d_c) / (self.two_d_m - other.two_d_m);
        let y = self.two_d_m * x + self.two_d_c;
        if (x >= self.loc[0]) != self.velocity[0].is_sign_positive() {
            return None;
        }
        if (x >= other.loc[0]) != other.velocity[0].is_sign_positive() {
            return None;
        }
        Some((x, y))
    }
}

fn parse(input: &str) -> Vec<Hailstone> {
    input
        .trim()
        .lines()
        .map(|line| {
            Hailstone::new(line).unwrap_or_else(|| panic!("failed to parse hailstorm {:?}", line))
        })
        .collect()
}

pub fn part_one(input: &str, boundary: Range<f64>) -> u64 {
    let hailstones = parse(input);
    let mut count = 0;
    for (i, hailstone_a) in hailstones.iter().enumerate() {
        for hailstone_b in hailstones.iter().skip(i + 1) {
            if let Some(intersect) = hailstone_a.two_d_paths_intersect(hailstone_b) {
                if boundary.contains(&intersect.0) && boundary.contains(&intersect.1) {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(include_str!("../example.txt"), 7.0..27.0), 2)
    }

    #[test]
    #[cfg(feature = "challenge")]
    fn part_one_challenge() {
        assert_eq!(
            part_one(
                include_str!("../input.txt"),
                200000000000000.0..400000000000000.0
            ),
            15262
        )
    }
}
