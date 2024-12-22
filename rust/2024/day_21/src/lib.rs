use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    ops::Add,
};

type Num = i64;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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

    fn to_coord(&self) -> Coord {
        match self {
            DirectionalKeypadButton::Up => Coord { x: -1, y: 0 },
            DirectionalKeypadButton::A => Coord { x: 0, y: 0 },
            DirectionalKeypadButton::Left => Coord { x: -2, y: -1 },
            DirectionalKeypadButton::Down => Coord { x: -1, y: -1 },
            DirectionalKeypadButton::Right => Coord { x: 0, y: -1 },
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct MoveTime {
    level: usize,
    start: DirectionalKeypadButton,
    end: DirectionalKeypadButton,
}

const DIRECTION_KEYPAD_BUTTONS: [DirectionalKeypadButton; 5] = [
    DirectionalKeypadButton::Up,
    DirectionalKeypadButton::A,
    DirectionalKeypadButton::Left,
    DirectionalKeypadButton::Down,
    DirectionalKeypadButton::Right,
];

const DIRECTIONAL_KEYPAD_BUTTONS: [DirectionalKeypadButton; 4] = [
    DirectionalKeypadButton::Up,
    DirectionalKeypadButton::Left,
    DirectionalKeypadButton::Down,
    DirectionalKeypadButton::Right,
];

fn populate_move_times(levels: usize) -> HashMap<MoveTime, usize> {
    let mut move_times = HashMap::new();
    for start in DIRECTION_KEYPAD_BUTTONS {
        for end in DIRECTION_KEYPAD_BUTTONS {
            let start_pos = start.to_coord();
            let end_pos = end.to_coord();
            move_times.insert(
                MoveTime {
                    level: 0,
                    start,
                    end,
                },
                (start_pos.x.abs_diff(end_pos.x) + start_pos.y.abs_diff(end_pos.y) + 1) as usize,
            );
        }
    }
    for level in 1..levels {
        for start in DIRECTION_KEYPAD_BUTTONS {
            for end in DIRECTION_KEYPAD_BUTTONS {
                let mut visited =
                    HashSet::<(DirectionalKeypadButton, bool, DirectionalKeypadButton)>::new();
                let mut unvisited = BinaryHeap::<(
                    Reverse<usize>,
                    DirectionalKeypadButton,
                    bool,
                    DirectionalKeypadButton,
                )>::new();
                unvisited.push((Reverse(0), DirectionalKeypadButton::A, false, start));
                while let Some((Reverse(time), parent, pressed, button)) = unvisited.pop() {
                    if pressed && button == end {
                        move_times.insert(MoveTime { level, start, end }, time);
                        break;
                    }

                    if !visited.insert((parent, pressed, button)) {
                        continue;
                    }

                    if button == end {
                        unvisited.push((
                            Reverse(
                                time + move_times[&MoveTime {
                                    level: level - 1,
                                    start: parent,
                                    end: DirectionalKeypadButton::A,
                                }],
                            ),
                            DirectionalKeypadButton::A,
                            true,
                            button,
                        ));
                    }

                    for direction in DIRECTIONAL_KEYPAD_BUTTONS {
                        if let Some(resulting_button) = DirectionalKeypadButton::at_coord(
                            &(&button.to_coord() + &direction.direction()),
                        ) {
                            unvisited.push((
                                Reverse(
                                    time + move_times[&MoveTime {
                                        level: level - 1,
                                        start: parent,
                                        end: direction,
                                    }],
                                ),
                                direction,
                                false,
                                resulting_button,
                            ));
                        }
                    }
                }
            }
        }
    }
    move_times
}

fn input_code(
    sequence: &[NumberPadButton],
    depth: usize,
    move_times: &HashMap<MoveTime, usize>,
) -> usize {
    let mut visited: HashSet<(usize, DirectionalKeypadButton, Coord)> = HashSet::new();
    let mut unvisited =
        BinaryHeap::<(Reverse<usize>, usize, DirectionalKeypadButton, Coord)>::new();
    unvisited.push((Reverse(0), 0, DirectionalKeypadButton::A, Coord::default()));
    while let Some((Reverse(time), inputted, parent, arm)) = unvisited.pop() {
        if inputted == sequence.len() {
            return time;
        }

        if !visited.insert((inputted, parent, arm)) {
            continue;
        }

        if let Some(button) = NumberPadButton::at_coord(&arm) {
            if button == sequence[inputted] {
                unvisited.push((
                    Reverse(
                        time + move_times[&MoveTime {
                            level: depth - 1,
                            start: parent,
                            end: DirectionalKeypadButton::A,
                        }],
                    ),
                    inputted + 1,
                    DirectionalKeypadButton::A,
                    arm,
                ));
            }

            for direction in DIRECTIONAL_KEYPAD_BUTTONS {
                unvisited.push((
                    Reverse(
                        time + move_times[&MoveTime {
                            level: depth - 1,
                            start: parent,
                            end: direction,
                        }],
                    ),
                    inputted,
                    direction,
                    &arm + &direction.direction(),
                ));
            }
        }
    }
    panic!();
}

pub fn complexity(input: &str, depth: usize) -> usize {
    let move_times = populate_move_times(depth);
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|code| {
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
            let shortest_sequence = input_code(&encoded, depth, &move_times);
            let numeric_part: usize = code[0..3].parse().unwrap();
            numeric_part * shortest_sequence
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(complexity(include_str!("../example_1.txt"), 2), 126384);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(complexity(include_str!("../input.txt"), 2), 138764);
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(
            complexity(include_str!("../input.txt"), 25),
            169137886514152
        );
    }
}
