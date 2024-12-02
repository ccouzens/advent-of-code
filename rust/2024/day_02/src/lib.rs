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

fn safe_report(report: impl DoubleEndedIterator<Item = Num> + Clone) -> bool {
    (report.clone().is_sorted() || report.clone().rev().is_sorted())
        && report
            .clone()
            .zip(report.clone().skip(1))
            .all(|(a, b)| (1..=3).contains(&Num::abs_diff(a, b)))
}

pub fn part_1(input: &str) -> usize {
    let reports = parse(input);
    reports
        .iter()
        .filter(|r| safe_report(r.iter().copied()))
        .count()
}

pub fn part_2(input: &str) -> usize {
    let reports = parse(input);
    reports
        .iter()
        .filter(|r| {
            (0..r.len()).any(move |i| {
                safe_report(
                    r.iter()
                        .enumerate()
                        .filter_map(|(j, &n)| (i != j).then_some(n)),
                )
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
