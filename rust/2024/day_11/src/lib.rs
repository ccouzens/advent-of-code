use std::collections::BTreeMap;

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

    fn sequence_length_calculator(&self, iterations: usize) -> usize {
        let mut memo = BTreeMap::<(Num, usize), usize>::new();
        let mut count = 0;

        for &stone in self.0.iter() {
            count += sequence_length_calculator_internal(stone, iterations, &mut memo);
        }

        count
    }
}

fn sequence_length_calculator_internal(
    num: Num,
    iterations: usize,
    memo: &mut BTreeMap<(Num, usize), usize>,
) -> usize {
    if iterations == 0 {
        return 1;
    }

    if let Some(&count) = memo.get(&(num, iterations)) {
        return count;
    }

    let digits = Num::checked_ilog10(num).map(|i| i + 1);
    let count = match digits {
        None => sequence_length_calculator_internal(1, iterations - 1, memo),
        Some(digits) if digits % 2 == 0 => {
            let divider = Num::pow(10, digits / 2);

            sequence_length_calculator_internal(num / divider, iterations - 1, memo)
                + sequence_length_calculator_internal(num % divider, iterations - 1, memo)
        }
        _ => sequence_length_calculator_internal(num * 2024, iterations - 1, memo),
    };

    memo.insert((num, iterations), count);
    count
}

pub fn part_1(input: &str) -> usize {
    let pebbles = Pebbles::parse(input);
    pebbles.sequence_length_calculator(25)
}

pub fn part_2(input: &str) -> usize {
    let pebbles = Pebbles::parse(input);
    pebbles.sequence_length_calculator(75)
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

    #[test]
    fn challenge_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 220377651399268);
    }
}
