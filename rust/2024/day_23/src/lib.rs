use std::collections::{BTreeMap, BTreeSet};

pub fn part_1(input: &str) -> usize {
    let mut connections: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let a = &line[0..2];
        let b = &line[3..5];
        connections.entry(a).or_default().insert(b);
        connections.entry(b).or_default().insert(a);
    }

    let mut clusters: BTreeSet<BTreeSet<&str>> = BTreeSet::new();
    for (&a, a_connections) in connections.iter() {
        for &b in a_connections.iter() {
            for &c in a_connections.intersection(&connections[b]) {
                clusters.insert([a, b, c].iter().copied().collect());
            }
        }
    }
    clusters
        .iter()
        .filter(|cluster| cluster.iter().any(|computer| computer.starts_with('t')))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_1.txt")), 7);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 1110);
    }
}
