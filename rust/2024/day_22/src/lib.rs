use std::{collections::BTreeMap, iter::once};

type Num = i64;

fn parse(input: &str) -> impl Iterator<Item = Num> + '_ {
    input.lines().filter_map(|l| l.parse().ok())
}

fn evolve(secret: Num) -> Num {
    let secret = ((secret * 64) ^ secret) % 16777216;
    let secret = ((secret / 32) ^ secret) % 16777216;
    ((secret * 2048) ^ secret) % 16777216
}

fn secret_evolution(mut secret: Num) -> impl Iterator<Item = Num> {
    once(secret).chain((0..2000).map(move |_| {
        secret = evolve(secret);
        secret
    }))
}

pub fn part_1(input: &str) -> Num {
    parse(input)
        .map(|secret| secret_evolution(secret).last().unwrap())
        .sum()
}

pub fn part_2(input: &str) -> Num {
    let mut overall_totals: BTreeMap<[Num; 4], Num> = BTreeMap::new();
    for hiding_spot in parse(input) {
        let mut local_totals: BTreeMap<[Num; 4], Num> = BTreeMap::new();
        let prices: Vec<_> = secret_evolution(hiding_spot).map(|i| i % 10).collect();
        for window in prices.windows(5) {
            local_totals
                .entry([
                    window[1] - window[0],
                    window[2] - window[1],
                    window[3] - window[2],
                    window[4] - window[3],
                ])
                .or_insert(window[4]);
        }
        for (&seq, &bananas) in local_totals.iter() {
            *overall_totals.entry(seq).or_default() += bananas;
        }
    }
    overall_totals.values().copied().max().unwrap()
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

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(include_str!("../example_2.txt")), 23);
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 2250);
    }
}
