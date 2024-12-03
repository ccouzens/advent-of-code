use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{char, digit1},
    combinator::{map, map_res},
    multi::many0,
    sequence::{delimited, separated_pair},
    IResult,
};
type Num = u64;
enum Instruction {
    Junk,
    Mul(Num, Num),
    Do,
    DoNot,
}

fn parse(input: &str) -> Vec<Instruction> {
    let raw: IResult<_, _> = many0(alt((
        map(
            delimited(
                tag("mul("),
                separated_pair(
                    map_res(digit1, str::parse),
                    char(','),
                    map_res(digit1, str::parse),
                ),
                char(')'),
            ),
            |(a, b)| Instruction::Mul(a, b),
        ),
        map(tag("do()"), |_| Instruction::Do),
        map(tag("don't()"), |_| Instruction::DoNot),
        map(take(1usize), |_| Instruction::Junk),
    )))(input);

    raw.unwrap().1
}

pub fn part_1(input: &str) -> Num {
    parse(input)
        .iter()
        .filter_map(|instruction| {
            if let Instruction::Mul(a, b) = instruction {
                Some(a * b)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_2(input: &str) -> Num {
    let mut sum = 0;
    let mut enabled = true;
    for instruction in parse(input).iter() {
        match instruction {
            Instruction::Do => enabled = true,
            Instruction::DoNot => enabled = false,
            Instruction::Mul(a, b) => {
                if enabled {
                    sum += a * b
                }
            }
            Instruction::Junk => {}
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 161);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 180233229);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(include_str!("../example_2.txt")), 48);
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 95411583);
    }
}
