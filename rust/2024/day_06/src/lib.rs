use std::collections::HashSet;

struct Lab {
    walls: Vec<bool>,
    width: isize,
    height: isize,
}

struct Guard {
    direction: (isize, isize),
    position: (isize, isize),
    visited: HashSet<(isize, isize)>,
}
struct World {
    lab: Lab,
    guard: Guard,
}

impl World {
    fn parse(input: &str) -> Self {
        let width = input.lines().next().unwrap().chars().count() as isize;
        let mut height = 0;
        let mut position = (0, 0);
        let mut walls = Vec::new();
        for (line, y) in input.lines().filter(|l| l.is_empty()).zip(0..) {
            height += 1;
            for (c, x) in line.chars().zip(0..) {
                match c {
                    '#' => {
                        walls.push(true);
                    }
                    '^' => {
                        walls.push(false);
                        position = (x, y)
                    }
                    _ => walls.push(false),
                }
            }
        }

        Self {
            lab: Lab {
                walls,
                width,
                height,
            },
            guard: Guard {
                direction: (0, -1),
                position,
                visited: HashSet::new(),
            },
        }
    }

    fn guard_in_lab(&self) -> bool {
        self.guard.position.0 >= 0 && self.guard.position.0 <= self.lab.width &&
        self.guard.position.1 >= 0 && self.guard.position.1 <= self.lab.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
