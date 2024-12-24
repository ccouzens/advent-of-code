use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
};

#[derive(Debug, Clone, Copy)]
enum GateInput {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, Copy)]
struct Gate<'a> {
    inputs: [&'a str; 2],
    gate: GateInput,
}

impl<'a> Gate<'a> {
    fn parser(
    ) -> impl FnMut(&'a str) -> Result<(&'a str, Self), nom::Err<nom::error::Error<&'a str>>> {
        map(
            tuple((
                alphanumeric1,
                alt((
                    map(tag(" AND "), |_| GateInput::And),
                    map(tag(" OR "), |_| GateInput::Or),
                    map(tag(" XOR "), |_| GateInput::Xor),
                )),
                alphanumeric1,
            )),
            |(a, g, b)| Self {
                inputs: [a, b],
                gate: g,
            },
        )
    }
}

#[derive(Debug, Clone)]
struct PuzzleInput<'a> {
    inputs: BTreeMap<&'a str, bool>,
    gates: BTreeMap<&'a str, Gate<'a>>,
}

impl<'a> PuzzleInput<'a> {
    fn parser(
    ) -> impl FnMut(&'a str) -> Result<(&'a str, Self), nom::Err<nom::error::Error<&'a str>>> {
        map(
            separated_pair(
                separated_list1(
                    newline,
                    separated_pair(
                        alphanumeric1,
                        tag(": "),
                        alt((map(char('0'), |_| false), map(char('1'), |_| true))),
                    ),
                ),
                tag("\n\n"),
                separated_list1(
                    newline,
                    separated_pair(Gate::parser(), tag(" -> "), alphanumeric1),
                ),
            ),
            |(inputs, gates)| Self {
                inputs: inputs.iter().copied().collect(),
                gates: gates.iter().copied().map(|(a, b)| (b, a)).collect(),
            },
        )
    }

    fn parse(input: &'a str) -> Self {
        Self::parser()(input).unwrap().1
    }

    fn value_of_wire<'b>(&self, wire: &'a str, memo: &'b mut BTreeMap<&'a str, bool>) -> bool {
        if let Some(&v) = memo.get(wire) {
            return v;
        }
        if let Some(&v) = self.inputs.get(wire) {
            return v;
        }
        let gate_input = self.gates[wire];
        let input_a = self.value_of_wire(gate_input.inputs[0], memo);
        let input_b = self.value_of_wire(gate_input.inputs[1], memo);
        let v = match gate_input.gate {
            GateInput::And => input_a && input_b,
            GateInput::Or => input_a || input_b,
            GateInput::Xor => input_a ^ input_b,
        };
        memo.insert(wire, v);
        v
    }
}

pub fn part_1(input: &str) -> u64 {
    let puzzle = PuzzleInput::parse(input);
    let mut result = 0;
    let mut memo = BTreeMap::new();
    for &wire in puzzle.gates.keys().rev() {
        if wire.starts_with('z') {
            let value = puzzle.value_of_wire(wire, &mut memo);
            result = result << 1 | if value { 1 } else { 0 };
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1_a() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 4);
    }

    #[test]
    fn example_part_1_b() {
        assert_eq!(part_1(include_str!("../example_2.txt")), 0b0011111101000);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 51837135476040);
    }
}
