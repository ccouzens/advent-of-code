use nom::{
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::{separated_list0, separated_list1},
    IResult,
};
type Num = u64;
type Report = Vec<Num>;
type Reports = Vec<Report>;

fn parse(input: &str) -> Reports {
    let raw: IResult<&str, Reports> = separated_list0(
        newline,
        separated_list1(space1, map_res(digit1, str::parse)),
    )(input);

    raw.unwrap().1
}

fn safe_report(r: &[Num]) -> bool {
    (r.iter().zip(r.iter().skip(1)).all(|(a, b)| a > b)
        || r.iter().zip(r.iter().skip(1)).all(|(a, b)| a < b))
        && r.iter()
            .zip(r.iter().skip(1))
            .all(|(&a, &b)| Num::abs_diff(a, b) <= 3)
}

pub fn part_1(input: &str) -> usize {
    let reports = parse(input);
    reports.iter().filter(|r| safe_report(r)).count()
}

pub fn part_2(input: &str) -> usize {
    let reports = parse(input);
    reports
        .iter()
        .filter(|r| {
            (0..r.len()).any(move |i| {
                let mut r_prime = (*r).clone();
                r_prime.remove(i);
                safe_report(&r_prime)
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 2);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 326);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(include_str!("../example_1.txt")), 4);
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 381);
    }
}
