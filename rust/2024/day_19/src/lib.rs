use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
};

#[derive(Debug, PartialEq)]
enum Colour {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Colour {
    fn parser<'a>(
    ) -> impl FnMut(&'a str) -> Result<(&'a str, Self), nom::Err<nom::error::Error<&'a str>>> {
        alt((
            map(char('w'), |_| Self::White),
            map(char('u'), |_| Self::Blue),
            map(char('b'), |_| Self::Black),
            map(char('r'), |_| Self::Red),
            map(char('g'), |_| Self::Green),
        ))
    }
}

#[derive(Debug)]
struct TowelPattern(Vec<Colour>);

impl TowelPattern {
    fn parser<'a>(
    ) -> impl FnMut(&'a str) -> Result<(&'a str, Self), nom::Err<nom::error::Error<&'a str>>> {
        map(many1(Colour::parser()), Self)
    }
}

#[derive(Debug)]
struct DesiredDesign(Vec<Colour>);

impl DesiredDesign {
    fn parser<'a>(
    ) -> impl FnMut(&'a str) -> Result<(&'a str, Self), nom::Err<nom::error::Error<&'a str>>> {
        map(many1(Colour::parser()), Self)
    }

    fn can_build_from_patterns(&self, patterns: &[TowelPattern]) -> bool {
        let mut stack: Vec<Vec<&TowelPattern>> = vec![Vec::new()];
        while let Some(patterns_in_design) = stack.pop() {
            let len = patterns_in_design.iter().map(|p| p.0.len()).sum();
            let remaining_design = &self.0[len..];
            for pattern in patterns.iter() {
                if remaining_design == pattern.0 {
                    return true;
                }
                if remaining_design.starts_with(&pattern.0) {
                    let mut new_patterns = patterns_in_design.clone();
                    new_patterns.push(pattern);

                    stack.push(new_patterns);
                }
            }
        }

        false
    }
}

pub fn part_1(input: &str) -> usize {
    let onsen = Onsen::parse(input);
    onsen
        .desired_designs
        .iter()
        .filter(|d| d.can_build_from_patterns(&onsen.towel_patterns))
        .count()
}

#[derive(Debug)]
struct Onsen {
    towel_patterns: Vec<TowelPattern>,
    desired_designs: Vec<DesiredDesign>,
}

impl Onsen {
    fn parser<'a>(
    ) -> impl FnMut(&'a str) -> Result<(&'a str, Self), nom::Err<nom::error::Error<&'a str>>> {
        map(
            separated_pair(
                separated_list1(tag(", "), TowelPattern::parser()),
                tag("\n\n"),
                separated_list1(newline, DesiredDesign::parser()),
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
}
