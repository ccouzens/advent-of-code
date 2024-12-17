type Num = i64;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::tuple,
};

#[derive(Debug)]
struct Computer {
    registers: [Num; 3],
    code: Vec<Num>,
    instruction_pointer: usize,
    output: Vec<Num>,
}

fn parse_num<'a>(
) -> impl FnMut(&'a str) -> Result<(&'a str, Num), nom::Err<nom::error::Error<&'a str>>> {
    map_res(digit1, str::parse)
}

impl Computer {
    fn parser<'a>(
    ) -> impl FnMut(&'a str) -> Result<(&'a str, Self), nom::Err<nom::error::Error<&'a str>>> {
        map(
            tuple((
                tag("Register A: "),
                parse_num(),
                tag("\nRegister B: "),
                parse_num(),
                tag("\nRegister C: "),
                parse_num(),
                tag("\n\nProgram: "),
                separated_list0(char(','), parse_num()),
            )),
            |(_, a, _, b, _, c, _, code)| Self {
                registers: [a, b, c],
                code,
                instruction_pointer: 0,
                output: Vec::new(),
            },
        )
    }

    fn parse(input: &str) -> Self {
        Self::parser()(input).unwrap().1
    }

    fn combo_operand(&self, operand: Num) -> Option<Num> {
        match operand {
            0..=3 => Some(operand),
            4..=6 => self
                .registers
                .get(usize::try_from(operand - 4).unwrap())
                .copied(),
            7 => None,
            _ => None,
        }
    }

    fn compute(&mut self) -> Option<()> {
        let &opcode = self.code.get(self.instruction_pointer)?;
        let &operand = self.code.get(self.instruction_pointer + 1)?;

        match opcode {
            0 => {
                self.registers[0] /= Num::pow(2, self.combo_operand(operand).unwrap() as u32);
            }
            1 => {
                self.registers[1] ^= operand;
            }
            2 => {
                self.registers[1] = self.combo_operand(operand).unwrap() % 8;
            }
            3 => {
                if self.registers[0] != 0 {
                    self.instruction_pointer = usize::try_from(operand).unwrap();
                    return Some(());
                }
            }
            4 => {
                self.registers[1] ^= self.registers[2];
            }
            5 => {
                self.output.push(self.combo_operand(operand).unwrap() % 8);
            }
            6 => {
                self.registers[1] =
                    self.registers[0] / Num::pow(2, self.combo_operand(operand).unwrap() as u32);
            }
            7 => {
                self.registers[2] =
                    self.registers[0] / Num::pow(2, self.combo_operand(operand).unwrap() as u32);
            }
            _ => {
                panic!("unexected opcode");
            }
        }
        self.instruction_pointer += 2;
        Some(())
    }
}

pub fn part_1(input: &str) -> String {
    let mut computer = Computer::parse(input);
    while let Some(()) = computer.compute() {}

    computer
        .output
        .iter()
        .enumerate()
        .map(|(i, &o)| {
            if i == 0 {
                format!("{}", o)
            } else {
                format!(",{}", o)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(
            part_1(include_str!("../example_1.txt")),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), "5,1,4,0,5,1,0,2,6");
    }
}
