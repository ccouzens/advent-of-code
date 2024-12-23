use std::{
    collections::{BTreeMap, BTreeSet},
    iter::once,
};

fn parse_connections(input: &str) -> BTreeMap<&str, BTreeSet<&str>> {
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
    connections
}

pub fn part_1(input: &str) -> usize {
    let connections = parse_connections(input);

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

pub fn part_2(input: &str) -> String {
    let connections = parse_connections(input);
    let mut clusters: BTreeSet<BTreeSet<&str>> = BTreeSet::new();
    clusters.insert(BTreeSet::new());
    loop {
        let prev_clusters = std::mem::take(&mut clusters);
        for prev_cluster in prev_clusters.iter() {
            for (&other, other_connections) in connections.iter() {
                if prev_cluster.iter().all(|p| other_connections.contains(p)) {
                    clusters.insert(prev_cluster.iter().copied().chain(once(other)).collect());
                }
            }
        }
        if clusters.is_empty() {
            return prev_clusters
                .first()
                .unwrap()
                .iter()
                .copied()
                .collect::<Vec<_>>()
                .join(",");
        }
    }
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

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(include_str!("../example_1.txt")), "co,de,ka,ta");
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(
            part_2(include_str!("../input.txt")),
            "ej,hm,ks,ms,ns,rb,rq,sc,so,un,vb,vd,wd"
        );
    }
}
