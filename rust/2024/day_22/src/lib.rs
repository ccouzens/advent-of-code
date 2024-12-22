type Num = i64;

fn parse(input: &str) -> impl Iterator<Item = Num> + '_ {
    input.lines().filter_map(|l| l.parse().ok())
}

fn evolve(secret: Num) -> Num {
    let secret = ((secret * 64) ^ secret) % 16777216;
    let secret = ((secret / 32) ^ secret) % 16777216;
    ((secret * 2048) ^ secret) % 16777216
}

pub fn part_1(input: &str) -> Num {
    parse(input)
        .map(|mut secret| {
            for _ in 0..2000 {
                secret = evolve(secret);
            }
            secret
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 37327623);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 19847565303);
    }
}
