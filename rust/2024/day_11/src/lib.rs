type Num = u64;
struct Pebbles(Vec<Num>);

impl Pebbles {
    fn parse(input: &str) -> Self {
        Self(
            input
                .split_ascii_whitespace()
                .map(|c| c.parse().unwrap())
                .collect(),
        )
    }

    fn blink(&mut self) {
        let mut following = Vec::with_capacity(self.0.len());

        for &stone in self.0.iter() {
            let digits = Num::checked_ilog10(stone).map(|i| i + 1);
            match digits {
                None => following.push(1),
                Some(digits) if digits % 2 == 0 => {
                    let divider = Num::pow(10, digits / 2);
                    following.push(stone / divider);
                    following.push(stone % divider);
                }
                _ => following.push(stone * 2024),
            }
        }

        self.0 = following;
    }

    fn count(&self) -> usize {
        self.0.len()
    }
}

pub fn part_1(input: &str) -> usize {
    let mut pebbles = Pebbles::parse(input);
    for _ in 0..25 {
        pebbles.blink();
    }
    pebbles.count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 55312);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 183620);
    }
}
