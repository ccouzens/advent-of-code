use nom::{
    character::complete::{char, digit1, newline},
    combinator::{map, map_res},
    multi::{count, separated_list0, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::cmp::Ordering;

type Num = u16;

#[derive(Debug)]
struct Rule(Num, Num);
#[derive(Debug)]
struct Rules(Vec<Rule>);

#[derive(Debug)]
struct Update(Vec<Num>);
#[derive(Debug)]
struct Updates(Vec<Update>);

#[derive(Debug)]
struct World {
    rules: Rules,
    updates: Updates,
}

impl World {
    fn parse(input: &str) -> Self {
        let raw: IResult<_, _> = map(
            separated_pair(
                map(
                    separated_list0(
                        newline,
                        map(
                            separated_pair(
                                map_res(digit1, str::parse),
                                char('|'),
                                map_res(digit1, str::parse),
                            ),
                            |(a, b)| Rule(a, b),
                        ),
                    ),
                    Rules,
                ),
                count(newline, 2),
                map(
                    separated_list0(
                        newline,
                        map(
                            separated_list1(char(','), map_res(digit1, str::parse)),
                            Update,
                        ),
                    ),
                    Updates,
                ),
            ),
            |(rules, updates)| World { rules, updates },
        )(input);

        raw.unwrap().1
    }
}

impl Update {
    fn middle_page_number(&self) -> Num {
        self.0[self.0.len() / 2]
    }

    fn sorted_middle_page_number(&mut self, rules: &Rules) -> Num {
        self.0.sort_by(|&a, &b| {
            if rules.0.iter().any(|r| r.0 == a && r.1 == b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        self.middle_page_number()
    }
}

impl Rule {
    fn update_is_valid(&self, update: &Update) -> bool {
        if let (Some(a), Some(b)) = (
            update.0.iter().position(|&n| n == self.0),
            update.0.iter().position(|&n| n == self.1),
        ) {
            a < b
        } else {
            true
        }
    }
}

impl Rules {
    fn update_is_valid(&self, update: &Update) -> bool {
        self.0.iter().all(|rule| rule.update_is_valid(update))
    }
}

pub fn part_1(input: &str) -> Num {
    let world = World::parse(input);
    world
        .updates
        .0
        .iter()
        .filter(|update| world.rules.update_is_valid(update))
        .map(|update| update.middle_page_number())
        .sum()
}

pub fn part_2(input: &str) -> Num {
    let mut world = World::parse(input);
    world
        .updates
        .0
        .iter_mut()
        .filter(|update| !world.rules.update_is_valid(update))
        .map(|update| update.sorted_middle_page_number(&world.rules))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 143);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 6242);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(include_str!("../example_1.txt")), 123);
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 123);
    }
}
