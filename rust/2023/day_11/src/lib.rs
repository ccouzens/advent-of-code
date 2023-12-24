use std::collections::HashSet;

struct Image {
    galaxies: HashSet<(i64, i64)>,
    non_empty_horizontal_space: HashSet<i64>,
    non_empty_vertical_space: HashSet<i64>,
}

impl Image {
    fn new(input: &str) -> Self {
        let mut galaxies = HashSet::new();
        let mut non_empty_horizontal_space = HashSet::new();
        let mut non_empty_vertical_space = HashSet::new();
        for (line, y) in input.trim().lines().zip(0..) {
            for (c, x) in line.chars().zip(0..) {
                if c == '#' {
                    galaxies.insert((x, y));
                    non_empty_horizontal_space.insert(y);
                    non_empty_vertical_space.insert(x);
                }
            }
        }
        Self {
            galaxies,
            non_empty_horizontal_space,
            non_empty_vertical_space,
        }
    }
}
pub fn solve(input: &str, empty_space_size: i64) -> i64 {
    let image = Image::new(input);
    let mut sum = 0;
    for galaxy_a in image.galaxies.iter() {
        for galaxy_b in image.galaxies.iter() {
            if galaxy_a > galaxy_b {
                for x in i64::min(galaxy_a.0, galaxy_b.0)..i64::max(galaxy_a.0, galaxy_b.0) {
                    if image.non_empty_vertical_space.contains(&x) {
                        sum += 1;
                    } else {
                        sum += empty_space_size;
                    }
                }

                for y in i64::min(galaxy_a.1, galaxy_b.1)..i64::max(galaxy_a.1, galaxy_b.1) {
                    if image.non_empty_horizontal_space.contains(&y) {
                        sum += 1;
                    } else {
                        sum += empty_space_size;
                    }
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!(solve(include_str!("../example.txt"), 2), 374)
    }

    #[test]
    #[cfg(feature = "challenge")]
    fn part_one_challenge() {
        assert_eq!(solve(include_str!("../input.txt"), 2), 9214785)
    }

    #[test]
    fn part_two_example() {
        assert_eq!(solve(include_str!("../example.txt"), 10), 1030)
    }

    #[test]
    #[cfg(feature = "challenge")]
    fn part_two_challenge() {
        assert_eq!(solve(include_str!("../input.txt"), 1000000), 613686987427)
    }
}
