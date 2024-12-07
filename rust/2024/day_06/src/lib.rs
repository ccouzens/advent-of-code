use std::collections::BTreeSet;

#[derive(Debug, Clone)]
struct Lab {
    walls: Vec<bool>,
    width: isize,
    height: isize,
}

#[derive(Debug, Clone)]
struct Guard {
    direction: (isize, isize),
    position: (isize, isize),
    visited: BTreeSet<(isize, isize)>,
    loop_record: BTreeSet<((isize, isize), (isize, isize))>,
}

#[derive(Debug, Clone)]
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
        let direction = (0, -1);
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
                direction,
                position,
                visited: [position].iter().copied().collect(),
                loop_record: [(position, direction)].iter().copied().collect(),
            },
        }
    }

    fn guard_in_lab(&self) -> bool {
        self.guard.in_lab(&self.lab)
    }

    fn advance_guard(&mut self) -> bool {
        self.guard.advance(&self.lab)
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

    fn record_visit(&mut self, lab: &Lab) -> bool {
        if self.in_lab(lab) {
            self.visited.insert(self.position);
        }
        self.loop_record.insert((self.position, self.direction))
    }

    fn advance(&mut self, lab: &Lab) -> bool {
        for _ in 0..2 {
            let position_in_front = self.position_in_front();
            if lab.is_wall(position_in_front.0, position_in_front.1) {
                self.rotate_right();
            }
        }

        self.position.0 += self.direction.0;
        self.position.1 += self.direction.1;

        self.record_visit(lab)
    }
}

pub fn part_1(input: &str) -> usize {
    let mut world = World::parse(input);
    while world.guard_in_lab() {
        world.advance_guard();
    }
    world.guard.visited.len()
}

pub fn part_2(input: &str) -> usize {
    let world = World::parse(input);
    (0..world.lab.height)
        .flat_map(|y| (0..world.lab.height).map(move |x| (x, y)))
        .filter(|&(x, y)| {
            if world.guard.position == (x, y) {
                return false;
            }
            let mut world = world.clone();
            world.lab.walls[(x + y * world.lab.height) as usize] = true;
            while world.guard_in_lab() {
                if !world.advance_guard() {
                    return true;
                }
            }
            false
        })
        .count()
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

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(include_str!("../example_1.txt")), 6);
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 1480);
    }
}
