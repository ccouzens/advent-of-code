#[derive(Debug)]
struct LockKey {
    switch_points: [u8; 5],
    lock: bool,
}

impl LockKey {
    fn parse(input: &[u8]) -> Self {
        let lock = &input[0..5] == b"#####";
        let switch_point = |x: usize| {
            for y in 0..=5 {
                let i = input[6 + x + y * 6];
                if (i == b'.') == lock {
                    return y as u8;
                }
            }
            panic!();
        };

        Self {
            lock,
            switch_points: [
                switch_point(0),
                switch_point(1),
                switch_point(2),
                switch_point(3),
                switch_point(4),
            ],
        }
    }

    fn fit(&self, other: &Self) -> bool {
        if self.lock == other.lock {
            return false;
        }
        if other.lock {
            return other.fit(self);
        }
        (0..5).all(|x| self.switch_points[x] <= other.switch_points[x])
    }

    fn parse_all(inputs: &[u8]) -> Vec<Self> {
        let mut lock_keys = vec![];
        for i in 0.. {
            if let Some(input) = inputs.get(i * 43..i * 43 + 41) {
                lock_keys.push(Self::parse(input));
            } else {
                break;
            }
        }
        lock_keys
    }
}

pub fn part_1(input: &str) -> usize {
    let lock_keys = LockKey::parse_all(input.as_bytes());
    let locks: Vec<&LockKey> = lock_keys.iter().filter(|lk| lk.lock).collect();
    let keys: Vec<&LockKey> = lock_keys.iter().filter(|lk| !lk.lock).collect();
    let mut count = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if lock.fit(key) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 3);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 3136);
    }
}
