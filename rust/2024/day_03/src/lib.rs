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
type Mul = (Num, Num);

fn parse(input: &str) -> Vec<Option<Mul>> {
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
            Some,
        ),
        map(take(1usize), |_| None),
    )))(input);

    raw.unwrap().1
}

pub fn part_1(input: &str) -> Num {
    parse(input)
        .iter()
        .filter_map(|mul| mul.map(|(a, b)| a * b))
        .sum()
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
}
