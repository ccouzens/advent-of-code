use std::collections::HashSet;

#[derive(Debug)]
struct Lab {
    walls: Vec<bool>,
    width: isize,
    height: isize,
}

#[derive(Debug)]
struct Guard {
    direction: (isize, isize),
    position: (isize, isize),
    visited: HashSet<(isize, isize)>,
}

#[derive(Debug)]
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
        for (line, y) in input.lines().filter(|l| !l.is_empty()).zip(0..) {
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
                visited: [position].iter().copied().collect(),
            },
        }
    }

    fn guard_in_lab(&self) -> bool {
        self.guard.in_lab(&self.lab)
    }

    fn advance_guard(&mut self) {
        self.guard.advance(&self.lab);
    }
}

impl Lab {
    fn is_wall(&self, x: isize, y: isize) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return false;
        }
        let index = x + y * self.width;
        match usize::try_from(index) {
            Ok(i) => self.walls.get(i).copied().unwrap_or(false),
            Err(_) => false,
        }
    }
}

impl Guard {
    fn rotate_right(&mut self) {
        self.direction = (-self.direction.1, self.direction.0);
    }

    fn position_in_front(&self) -> (isize, isize) {
        (
            self.position.0 + self.direction.0,
            self.position.1 + self.direction.1,
        )
    }

    fn in_lab(&self, lab: &Lab) -> bool {
        self.position.0 >= 0
            && self.position.0 < lab.width
            && self.position.1 >= 0
            && self.position.1 < lab.height
    }

    fn record_visit(&mut self, lab: &Lab) {
        if self.in_lab(lab) {
            self.visited.insert(self.position);
        }
    }

    fn advance(&mut self, lab: &Lab) {
        let position_in_font = self.position_in_front();
        if lab.is_wall(position_in_font.0, position_in_font.1) {
            self.rotate_right();
        }

        self.position.0 += self.direction.0;
        self.position.1 += self.direction.1;

        self.record_visit(lab);
    }
}

pub fn part_1(input: &str) -> usize {
    let mut world = World::parse(input);
    while world.guard_in_lab() {
        world.advance_guard();
    }
    world.guard.visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 41);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 4580);
    }
}
