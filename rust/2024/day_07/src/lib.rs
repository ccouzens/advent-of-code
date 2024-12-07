use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

type Num = i64;

#[derive(Debug)]
struct Equation {
    test_value: Num,
    remaining_numbers: Vec<Num>,
}

impl Equation {
    fn calculate(&self, operators: Num) -> Option<Num> {
        let mut sum: Num = *self.remaining_numbers.first()?;
        for (&num, i) in self.remaining_numbers.iter().skip(1).zip(0..) {
            sum = if Num::pow(2, i) & operators == 0 {
                sum.checked_add(num)?
            } else {
                sum.checked_mul(num)?
            };
        }
        Some(sum)
    }

    fn is_valid(&self) -> bool {
        (0..Num::pow(2, self.remaining_numbers.len() as u32 - 1))
            .any(|operators| self.calculate(operators) == Some(self.test_value))
    }
}

fn parse(input: &str) -> Vec<Equation> {
    let raw: IResult<_, _> = separated_list1(
        newline,
        map(
            separated_pair(
                map_res(digit1, str::parse),
                tag(": "),
                separated_list1(char(' '), map_res(digit1, str::parse)),
            ),
            |(test_value, remaining_numbers)| Equation {
                test_value,
                remaining_numbers,
            },
        ),
    )(input);

    raw.unwrap().1
}

pub fn part_1(input: &str) -> Num {
    let equations = parse(input);
    equations
        .iter()
        .filter(|e| e.is_valid())
        .map(|e| e.test_value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 3749);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 7885693428401);
    }
}
