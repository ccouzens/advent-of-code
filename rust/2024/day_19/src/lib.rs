use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};

use rayon::prelude::*;

fn count_permutations(design: &[char], patterns: &[Vec<char>]) -> usize {
    let mut perms = vec![0; design.len() + 1];
    perms[0] = 1;
    for i in 0..design.len() {
        let remaining_design = &design[i..];
        for pattern in patterns.iter() {
            if remaining_design.starts_with(pattern) {
                perms[i + pattern.len()] += perms[i];
            }
        }
    }
    perms[design.len()]
}

pub fn part_1(input: &str) -> usize {
    let onsen = Onsen::parse(input);
    onsen
        .desired_designs
        .par_iter()
        .filter(|d| count_permutations(d, &onsen.towel_patterns) > 0)
        .count()
}

pub fn part_2(input: &str) -> usize {
    let onsen = Onsen::parse(input);
    onsen
        .desired_designs
        .par_iter()
        .map(|d| count_permutations(d, &onsen.towel_patterns))
        .sum()
}

#[derive(Debug)]
struct Onsen {
    towel_patterns: Vec<Vec<char>>,
    desired_designs: Vec<Vec<char>>,
}

impl Onsen {
    fn parser<'a>(
    ) -> impl FnMut(&'a str) -> Result<(&'a str, Self), nom::Err<nom::error::Error<&'a str>>> {
        map(
            separated_pair(
                separated_list1(tag(", "), map(alpha1, |c: &str| c.chars().collect())),
                tag("\n\n"),
                separated_list1(newline, map(alpha1, |c: &str| c.chars().collect())),
            ),
            |(towel_patterns, desired_designs)| Self {
                towel_patterns,
                desired_designs,
            },
        )
    }

    fn parse(input: &str) -> Self {
        Self::parser()(input).unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 6);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 340);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(include_str!("../example_1.txt")), 16);
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 717561822679428);
    }
}
