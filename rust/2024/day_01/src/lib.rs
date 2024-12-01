use nom::{
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

type Num = u32;
type Pair = (Num, Num);
type List = Vec<Pair>;

fn parse(input: &str) -> List {
    let raw: IResult<&str, List> = separated_list0(
        newline,
        separated_pair(
            map_res(digit1, str::parse),
            space1,
            map_res(digit1, str::parse),
        ),
    )(input);

    raw.unwrap().1
}

pub fn part_1(input: &str) -> Num {
    let list = parse(input);
    let mut list_a: Vec<_> = list.iter().map(|p| p.0).collect();
    let mut list_b: Vec<_> = list.iter().map(|p| p.1).collect();
    list_a.sort();
    list_b.sort();
    Iterator::zip(list_a.iter(), list_b.iter())
        .map(|(&a, &b)| Num::abs_diff(a, b))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 11);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 1603498);
    }
}
