use std::fmt::Write;
use std::ops::Range;

#[derive(Debug)]
struct Hailstone {
    loc: [f64; 3],
    velocity: [f64; 3],
    two_d_m: f64,
    two_d_c: f64,
}

impl Hailstone {
    fn new_from_loc_and_velocity(loc: [f64; 3], velocity: [f64; 3]) -> Self {
        Self {
            loc,
            velocity,
            two_d_m: velocity[1] / velocity[0],
            two_d_c: -loc[0] * velocity[1] / velocity[0] + loc[1],
        }
    }

    fn new(input: &str) -> Option<Self> {
        let mut split = input
            .split(|c: char| !c.is_ascii_digit() && c != '-')
            .filter_map(|n| n.parse().ok());
        let loc = [split.next()?, split.next()?, split.next()?];
        let velocity = [split.next()?, split.next()?, split.next()?];
        Some(Self::new_from_loc_and_velocity(loc, velocity))
    }

    fn two_d_paths_intersect(&self, other: &Self) -> Option<(f64, f64)> {
        if self.two_d_m == other.two_d_m {
            return None;
        }
        let x = (other.two_d_c - self.two_d_c) / (self.two_d_m - other.two_d_m);
        let y = self.two_d_m * x + self.two_d_c;
        if (x >= self.loc[0]) != self.velocity[0].is_sign_positive() {
            return None;
        }
        if (x >= other.loc[0]) != other.velocity[0].is_sign_positive() {
            return None;
        }
        Some((x, y))
    }

    fn position_at_time(&self, time: f64, dimension: usize) -> f64 {
        self.loc[dimension] + time * self.velocity[dimension]
    }

    /**
    Use least squares linear regression to find a hailstone that would
    come closest to hitting the other hailstones at the provided times.

    Each dimension (x, y, z) is worked out independently.
    */
    fn new_from_intersecting_paths(others: &[(&Self, f64)]) -> Self {
        let mut loc = [0.0; 3];
        let mut velocity = [0.0; 3];
        let n = others.len() as f64;
        let sum_x: f64 = others.iter().map(|(_h, t)| t).sum();
        let sum_x2: f64 = others.iter().map(|(_h, t)| t * t).sum();
        for (d, (l, v)) in loc.iter_mut().zip(velocity.iter_mut()).enumerate() {
            let sum_x_y: f64 = others
                .iter()
                .map(|(h, t)| h.position_at_time(*t, d) * t)
                .sum();
            let sum_y: f64 = others.iter().map(|(h, t)| h.position_at_time(*t, d)).sum();
            let m = (n * sum_x_y - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
            let b = (sum_y - m * sum_x) / n;
            *v = m;
            *l = b;
        }
        Self::new_from_loc_and_velocity(loc, velocity)
    }
}

fn parse(input: &str) -> Vec<Hailstone> {
    input
        .trim()
        .lines()
        .map(|line| {
            Hailstone::new(line).unwrap_or_else(|| panic!("failed to parse hailstorm {:?}", line))
        })
        .collect()
}

pub fn part_one(input: &str, boundary: Range<f64>) -> u64 {
    let hailstones = parse(input);
    let mut count = 0;
    for (i, hailstone_a) in hailstones.iter().enumerate() {
        for hailstone_b in hailstones.iter().skip(i + 1) {
            if let Some(intersect) = hailstone_a.two_d_paths_intersect(hailstone_b) {
                if boundary.contains(&intersect.0) && boundary.contains(&intersect.1) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn score_part_two_solution(hailstones: &[(&Hailstone, f64)]) -> f64 {
    let ideal = Hailstone::new_from_intersecting_paths(hailstones);
    hailstones
        .iter()
        .map(|(other, t)| {
            (0..3)
                .map(|d| {
                    let diff = other.position_at_time(*t, d) - ideal.position_at_time(*t, d);
                    diff.abs()
                })
                .sum::<f64>()
        })
        .sum::<f64>()
}

fn iterate_to_part_two_solution(hailstones: &[Hailstone]) -> f64 {
    let mut working_set = hailstones.iter().zip(0..).map(|(h, s)| (h, s as f64)).collect::<Vec<_>>();
    let mut index = 0;
    let mut score = score_part_two_solution(&working_set);

    'outer: loop {
        for direction in [-1.0, 1.0] {
            for magnitude in (0..).map(|m| f64::powi(2.0, m)/16.0) {
                println!("{index} {direction} {magnitude}");
                println!("{:?} {score}", working_set.iter().map(|w| w.1).collect::<Vec<_>>());
    let ideal = Hailstone::new_from_intersecting_paths(&working_set);
                println!("{ideal:?}");
                let mut candidate_set = working_set.clone();
                candidate_set[index].1 += direction * magnitude;
                if candidate_set[index].1 < 0.0 {
                    println!("negative breaking");
                    break;
                }
                let new_score = score_part_two_solution(&candidate_set);
                if new_score.is_nan() {
                    println!("nan continue");
                    continue;
                }
                if new_score > score && magnitude > 1024.0 {
                    break;
                }
                if new_score > score {
                    continue;
                }
                working_set = candidate_set;
                score = new_score;
                if score == 0.0 {
                    break 'outer;
                }
            }
        }
        index = (index + 1) % hailstones.len();
    }
    let ideal = Hailstone::new_from_intersecting_paths(&working_set);
    ideal.loc.iter().sum()
}

pub fn part_two(input: &str) -> String {
    let mut output = String::new();
    let hailstones = parse(input);
    let time_variables = ['t', 'u', 'w'];
    let location_variables = ['x', 'y', 'z'];
    let velocity_variables = ['a', 'b', 'c'];

    for (hailstone, time_variable) in hailstones.iter().zip(time_variables.iter()) {
        writeln!(
            &mut output,
            "({} {} {}) + {time_variable}({} {} {})",
            hailstone.loc[0],
            hailstone.loc[1],
            hailstone.loc[2],
            hailstone.velocity[0],
            hailstone.velocity[1],
            hailstone.velocity[2]
        )
        .unwrap();
    }
    writeln!(&mut output).unwrap();
    for (hailstone, time_variable) in hailstones.iter().zip(time_variables.iter()) {
        for (loc, (velocity, (loc_var, velocity_var))) in hailstone.loc.iter().zip(
            hailstone
                .velocity
                .iter()
                .zip(location_variables.iter().zip(velocity_variables.iter())),
        ) {
            writeln!(
                &mut output,
                "{loc_var} + {time_variable}*{velocity_var} = {loc}{velocity:+}{time_variable},"
            )
            .unwrap();
        }
    }
    writeln!(&mut output).unwrap();

    for (loc_i, _loc_var) in location_variables.iter().enumerate() {
        writeln!(
            &mut output,
            "{}{:+}{}{:+}{}-{}*{}+{}*{}=0,",
            hailstones[0].loc[loc_i] - hailstones[1].loc[loc_i],
            hailstones[0].velocity[loc_i],
            time_variables[0],
            -hailstones[1].velocity[loc_i],
            time_variables[1],
            time_variables[0],
            velocity_variables[loc_i],
            time_variables[1],
            velocity_variables[loc_i],
        )
        .unwrap();
        writeln!(
            &mut output,
            "{}{:+}{}{:+}{}-{}*{}+{}*{}=0,",
            hailstones[0].loc[loc_i] - hailstones[2].loc[loc_i],
            hailstones[0].velocity[loc_i],
            time_variables[0],
            -hailstones[2].velocity[loc_i],
            time_variables[2],
            time_variables[0],
            velocity_variables[loc_i],
            time_variables[2],
            velocity_variables[loc_i],
        )
        .unwrap();
    }

    writeln!(&mut output).unwrap();

    println!("{:?}", iterate_to_part_two_solution(&hailstones));

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(include_str!("../example.txt"), 7.0..27.0), 2)
    }

    #[test]
    #[cfg(feature = "challenge")]
    fn part_one_challenge() {
        assert_eq!(
            part_one(
                include_str!("../input.txt"),
                200000000000000.0..400000000000000.0
            ),
            15262
        )
    }

    #[test]
    fn part_two_example() {
        println!("{}", part_two(include_str!("../example.txt")));
        assert_eq!(
            part_two(include_str!("../example.txt")),
            include_str!("../part-two-example-output.txt")
        )
    }

    #[test]
    #[cfg(feature = "challenge")]
    fn part_two_challenge() {
        println!("{}", part_two(include_str!("../input.txt")));
        assert_eq!(
            part_two(include_str!("../example.txt")),
            include_str!("../part-two-input-output.txt")
        )
    }
}
