use std::{
    collections::{hash_map::Entry, HashMap},
    ops::Add,
};

type Num = i32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Coord {
    x: Num,
    y: Num,
}

impl Add for &Coord {
    type Output = Coord;

    fn add(self, other: &Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum DirectionalKeypadButton {
    Up,
    A,
    Left,
    Down,
    Right,
}

impl DirectionalKeypadButton {
    fn at_coord(coord: &Coord) -> Option<Self> {
        match coord {
            Coord { x: 0, y: 0 } => Some(Self::A),
            Coord { x: -1, y: 0 } => Some(Self::Up),
            Coord { x: -2, y: -1 } => Some(Self::Left),
            Coord { x: -1, y: -1 } => Some(Self::Down),
            Coord { x: 0, y: -1 } => Some(Self::Right),
            _ => None,
        }
    }

    fn direction(&self) -> Coord {
        match self {
            DirectionalKeypadButton::Up => Coord { x: 0, y: 1 },
            DirectionalKeypadButton::A => Coord { x: 0, y: 0 },
            DirectionalKeypadButton::Left => Coord { x: -1, y: 0 },
            DirectionalKeypadButton::Down => Coord { x: 0, y: -1 },
            DirectionalKeypadButton::Right => Coord { x: 1, y: 0 },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum NumberPadButton {
    Digit(u8),
    A,
}

impl NumberPadButton {
    fn at_coord(coord: &Coord) -> Option<Self> {
        match coord {
            Coord { x: 0, y: 0 } => Some(Self::A),
            Coord { x: -1, y: 0 } => Some(Self::Digit(0)),
            Coord { x: -2, y: 1 } => Some(Self::Digit(1)),
            Coord { x: -1, y: 1 } => Some(Self::Digit(2)),
            Coord { x: 0, y: 1 } => Some(Self::Digit(3)),
            Coord { x: -2, y: 2 } => Some(Self::Digit(4)),
            Coord { x: -1, y: 2 } => Some(Self::Digit(5)),
            Coord { x: 0, y: 2 } => Some(Self::Digit(6)),
            Coord { x: -2, y: 3 } => Some(Self::Digit(7)),
            Coord { x: -1, y: 3 } => Some(Self::Digit(8)),
            Coord { x: 0, y: 3 } => Some(Self::Digit(9)),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
struct RobotsState {
    depressurized_robot: Coord,
    freezing_robot: Coord,
    radiation_robot: Coord,
    presses: usize,
}

impl RobotsState {
    fn push_button(
        self,
        button: DirectionalKeypadButton,
        desired_presses: &[NumberPadButton],
    ) -> Option<Self> {
        fn control_depressurized_robot(
            button: DirectionalKeypadButton,
            robots_state: RobotsState,
            desired_presses: &[NumberPadButton],
        ) -> Option<RobotsState> {
            let coord = &robots_state.depressurized_robot + &button.direction();
            let button_under_pointer = NumberPadButton::at_coord(&coord)?;
            let mut state = robots_state;
            state.depressurized_robot = coord;
            if button == DirectionalKeypadButton::A {
                if button_under_pointer != desired_presses[state.presses] {
                    return None;
                }
                state.presses += 1;
            }
            Some(state)
        }
        fn control_freezing_robot(
            button: DirectionalKeypadButton,
            arm_states: RobotsState,
            desired_presses: &[NumberPadButton],
        ) -> Option<RobotsState> {
            let coord = &arm_states.freezing_robot + &button.direction();
            let button_under_pointer = DirectionalKeypadButton::at_coord(&coord)?;
            let state = RobotsState {
                freezing_robot: coord,
                ..arm_states
            };
            if button == DirectionalKeypadButton::A {
                control_depressurized_robot(button_under_pointer, state, desired_presses)
            } else {
                Some(state)
            }
        }
        let coord = &self.radiation_robot + &button.direction();
        let button_under_pointer = DirectionalKeypadButton::at_coord(&coord)?;
        let state = RobotsState {
            radiation_robot: coord,
            ..self
        };
        if button == DirectionalKeypadButton::A {
            control_freezing_robot(button_under_pointer, state, desired_presses)
        } else {
            Some(state)
        }
    }
}

fn input_code(goal: &[NumberPadButton]) -> usize {
    let mut time_to_state: HashMap<RobotsState, usize> = HashMap::new();
    let mut explore_next: Vec<RobotsState> = vec![RobotsState::default()];
    for time in 0.. {
        for &s in std::mem::take(&mut explore_next).iter() {
            if s.presses == goal.len() {
                return time;
            }
            if let Entry::Vacant(v) = time_to_state.entry(s) {
                v.insert(time);
                for b in [
                    DirectionalKeypadButton::Up,
                    DirectionalKeypadButton::Down,
                    DirectionalKeypadButton::Left,
                    DirectionalKeypadButton::Right,
                    DirectionalKeypadButton::A,
                ] {
                    if let Some(next_state) = s.push_button(b, goal) {
                        explore_next.push(next_state);
                    }
                }
            }
        }
        if explore_next.is_empty() {
            panic!();
        }
    }
    unreachable!();
}

fn complexity(code: &str) -> usize {
    let encoded: Vec<NumberPadButton> = code
        .bytes()
        .map(|b| {
            if b == b'A' {
                NumberPadButton::A
            } else {
                NumberPadButton::Digit(b - b'0')
            }
        })
        .collect();
    let shortest_sequence = input_code(&encoded);
    let numeric_part: usize = code[0..3].parse().unwrap();
    numeric_part * shortest_sequence
}

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(complexity)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 126384);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 138764);
    }
}
