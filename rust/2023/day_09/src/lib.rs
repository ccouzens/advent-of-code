fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|v| v.parse().ok())
                .collect()
        })
        .collect()
}

fn next_sequence_value(input: &[i64]) -> i64 {
    if input.iter().all(|v| *v == 0) {
        return 0;
    }
    let differences: Vec<i64> = input
        .iter()
        .zip(input.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();

    next_sequence_value(&differences) + input.last().unwrap()
}

pub fn part_one(input: &str) -> i64 {
    parse(input).iter().map(|l| next_sequence_value(l)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(include_str!("../example.txt")), 114)
    }

    #[test]
    #[cfg(feature = "challenge")]
    fn part_one_challenge() {
        assert_eq!(part_one(include_str!("../input.txt")), 1898776583)
    }
}
