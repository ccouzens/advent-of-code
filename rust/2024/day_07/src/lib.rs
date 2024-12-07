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
    fn calculate(&self, operators: Num, concatenation: bool) -> Option<Num> {
        let mut sum: Num = *self.remaining_numbers.first()?;
        for (&num, i) in self.remaining_numbers.iter().skip(1).zip(0..) {
            let operator = (operators / Num::pow(3, i)) % 3;
            sum = match operator {
                0 => sum.checked_add(num)?,
                1 => sum.checked_mul(num)?,
                2 => {
                    if concatenation {
                        let multiplier = Num::pow(10, num.ilog10() + 1);
                        sum.checked_mul(multiplier)?.checked_add(num)?
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        }
        Some(sum)
    }

    fn is_valid(&self, concatenation: bool) -> bool {
        (0..Num::pow(3, self.remaining_numbers.len() as u32 - 1))
            .any(|operators| self.calculate(operators, concatenation) == Some(self.test_value))
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
        .filter(|e| e.is_valid(false))
        .map(|e| e.test_value)
        .sum()
}

pub fn part_2(input: &str) -> Num {
    let equations = parse(input);
    equations
        .iter()
        .filter(|e| e.is_valid(true))
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

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(include_str!("../example_1.txt")), 11387);
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 348360680516005);
    }
}
