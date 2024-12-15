use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

type Num = i64;

#[derive(Debug)]
struct Coord([Num; 2]);

#[derive(Debug)]
struct ClawMachine {
    button_a: Coord,
    button_b: Coord,
    prize: Coord,
}

impl ClawMachine {
    fn tokens_for_prize(&self) -> Option<Num> {
        let b_presses = Num::checked_div(
            self.button_a.0[0] * self.prize.0[1] - self.button_a.0[1] * self.prize.0[0],
            self.button_a.0[0] * self.button_b.0[1] - self.button_a.0[1] * self.button_b.0[0],
        )?;

        let a_presses = Num::checked_div(
            self.prize.0[0] - b_presses * self.button_b.0[0],
            self.button_a.0[0],
        )?;

        if a_presses * self.button_a.0[0] + b_presses * self.button_b.0[0] != self.prize.0[0]
            || a_presses * self.button_a.0[1] + b_presses * self.button_b.0[1] != self.prize.0[1]
        {
            return None;
        }

        Some(3 * a_presses + b_presses)
    }
}

fn parse_claw_machines(input: &str) -> Vec<ClawMachine> {
    let raw: IResult<&str, Vec<ClawMachine>> = separated_list1(
        tag("\n\n"),
        map(
            tuple((
                tag("Button A: X+"),
                map_res(digit1, str::parse),
                tag(", Y+"),
                map_res(digit1, str::parse),
                tag("\nButton B: X+"),
                map_res(digit1, str::parse),
                tag(", Y+"),
                map_res(digit1, str::parse),
                tag("\nPrize: X="),
                map_res(digit1, str::parse),
                tag(", Y="),
                map_res(digit1, str::parse),
            )),
            |(_, ax, _, ay, _, bx, _, by, _, px, _, py)| ClawMachine {
                button_a: Coord([ax, ay]),
                button_b: Coord([bx, by]),
                prize: Coord([px, py]),
            },
        ),
    )(input);

    raw.unwrap().1
}

pub fn part_1(input: &str) -> Num {
    let claw_machines = parse_claw_machines(input);
    claw_machines
        .iter()
        .filter_map(|m| m.tokens_for_prize())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 480);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 28753);
    }
}
