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

pub fn part_two(input: &str) -> String {
    let mut output = String::new();
    let hailstones = parse(input);
    let time_variables = ['t', 'u', 'w'];

    for (hailstone, time_variable) in hailstones.iter().zip(time_variables.iter()) {
        writeln!(
            &mut output,
            "(x y z) + {time_variable}(a b c) - ({} {} {}) - {time_variable}({} {} {}) = (0 0 0)",
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
        writeln!(
            &mut output,
            "(x y z) -  ({} {} {}) = {time_variable}({} {} {}) - {time_variable}(a b c)",
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
        writeln!(
            &mut output,
            "(x - {loc_x}, y - {loc_y}, z - {loc_z}) = {time_variable}({vel_x} - a, {vel_y} - b, {vel_z} - c)",
            loc_x= hailstone.loc[0],
            loc_y= hailstone.loc[1],
            loc_z = hailstone.loc[2],
            vel_x = hailstone.velocity[0],
            vel_y = hailstone.velocity[1],
            vel_z = hailstone.velocity[2]
        )
        .unwrap();
    }

    writeln!(&mut output).unwrap();

    for hailstone in hailstones.iter().take(3) {
        let loc_x = hailstone.loc[0];
        let loc_y = hailstone.loc[1];
        let loc_z = hailstone.loc[2];
        let vel_x = hailstone.velocity[0];
        let vel_y = hailstone.velocity[1];
        let vel_z = hailstone.velocity[2];
        writeln!(
            &mut output,
            "(y - {loc_y})*({vel_z} - c) - (z - {loc_z})*({vel_y} -b) = 0",
        )
        .unwrap();
        writeln!(
            &mut output,
            "(z - {loc_z})*({vel_x} - a) - (x - {loc_x})*({vel_z} -c) = 0",
        )
        .unwrap();
        writeln!(
            &mut output,
            "(x - {loc_x})*({vel_y} - b) - (y - {loc_y})*({vel_x} -a) = 0",
        )
        .unwrap();
    }

    writeln!(&mut output).unwrap();

    writeln!(&mut output, "from sympy import solve").unwrap();
    writeln!(&mut output, "from sympy.abc import x, y, z, a, b, c").unwrap();
    writeln!(&mut output, "solve([").unwrap();
    for hailstone in hailstones.iter().take(3) {
        let loc_x = hailstone.loc[0];
        let loc_y = hailstone.loc[1];
        let loc_z = hailstone.loc[2];
        let vel_x = hailstone.velocity[0];
        let vel_y = hailstone.velocity[1];
        let vel_z = hailstone.velocity[2];
        writeln!(
            &mut output,
            "{vel_z}*y - c*y {:+} {loc_y:+}*c {:+}*z +b*z {:+}*b,",
            -loc_y * vel_z + loc_z * vel_y,
            -vel_y,
            -loc_z
        )
        .unwrap();
        writeln!(
            &mut output,
            "{vel_x}*z - a*z {:+} {loc_z:+}*a {:+}*x +c*x {:+}*c,",
            -loc_z * vel_x + loc_x * vel_z,
            -vel_z,
            -loc_x
        )
        .unwrap();
        writeln!(
            &mut output,
            "{vel_y}*x - b*x {:+} {loc_x:+}*b {:+}*y +a*y {:+}*a,",
            -loc_x * vel_y + loc_y * vel_x,
            -vel_x,
            -loc_y
        )
        .unwrap();
    }
    writeln!(&mut output, "], [x, y, z, a, b, c], set=True)").unwrap();

    let loc_x_0 = hailstones[0].loc[0];
    let loc_y_0 = hailstones[0].loc[1];
    let loc_z_0 = hailstones[0].loc[2];
    let vel_x_0 = hailstones[0].velocity[0];
    let vel_y_0 = hailstones[0].velocity[1];
    let vel_z_0 = hailstones[0].velocity[2];

    writeln!(&mut output).unwrap();

    writeln!(&mut output, "from sympy import solve").unwrap();
    writeln!(&mut output, "from sympy.abc import x, y, z, a, b, c").unwrap();
    writeln!(&mut output, "solve([").unwrap();
    for hailstone in hailstones.iter().skip(1).take(2) {
        let loc_x = hailstone.loc[0];
        let loc_y = hailstone.loc[1];
        let loc_z = hailstone.loc[2];
        let vel_x = hailstone.velocity[0];
        let vel_y = hailstone.velocity[1];
        let vel_z = hailstone.velocity[2];
        writeln!(
            &mut output,
            "{:+}*y {:+} {:+}*c {:+}*z {:+}*b {vel_z:+}*y {:+} {loc_y:+}*c {:+}*z {:+}*b,",
            -vel_z_0,
            loc_y_0 * vel_z_0 - loc_z_0 * vel_y_0,
            -loc_y_0,
            vel_y_0,
            loc_z_0,
            -loc_y * vel_z + loc_z * vel_y,
            -vel_y,
            -loc_z
        )
        .unwrap();

        writeln!(
            &mut output,
            "{:+}*z {:+} {:+}*a {:+}*x {:+}*c {vel_x:+}*z {:+} {loc_z:+}*a {:+}*x {:+}*c,",
            -vel_x_0,
            loc_z_0 * vel_x_0 - loc_x_0 * vel_z_0,
            -loc_z_0,
            vel_z_0,
            loc_x_0,
            -loc_z * vel_x + loc_x * vel_z,
            -vel_z,
            -loc_x
        )
        .unwrap();

        writeln!(
            &mut output,
            "{:+}*x {:+} {:+}*b {:+}*y {:+}*a {vel_y:+}*x {:+} {loc_x:+}*b {:+}*y {:+}*a,",
            -vel_y_0,
            loc_x_0 * vel_y_0 - loc_y_0 * vel_x_0,
            -loc_x_0,
            vel_x_0,
            loc_y_0,
            -loc_x * vel_y + loc_y * vel_x,
            -vel_x,
            -loc_y
        )
        .unwrap();
    }
    writeln!(&mut output, "], [x, y, z, a, b, c], set=True)").unwrap();
    writeln!(&mut output).unwrap();

    writeln!(&mut output, "from sympy import solve").unwrap();
    writeln!(&mut output, "from sympy.abc import x, y, z, a, b, c").unwrap();
    writeln!(&mut output, "solve([").unwrap();
    for hailstone in hailstones.iter().skip(1).take(2) {
        let loc_x = hailstone.loc[0];
        let loc_y = hailstone.loc[1];
        let loc_z = hailstone.loc[2];
        let vel_x = hailstone.velocity[0];
        let vel_y = hailstone.velocity[1];
        let vel_z = hailstone.velocity[2];
        writeln!(
            &mut output,
            "{:+}*y {:+} {:+}*c {:+}*z {:+}*b,",
            -vel_z_0 + vel_z,
            loc_y_0 * vel_z_0 - loc_z_0 * vel_y_0 - loc_y * vel_z + loc_z * vel_y,
            -loc_y_0 + loc_y,
            vel_y_0 - vel_y,
            loc_z_0 - loc_z
        )
        .unwrap();
        writeln!(
            &mut output,
            "{:+}*z {:+} {:+}*a {:+}*x {:+}*c,",
            -vel_x_0 + vel_x,
            loc_z_0 * vel_x_0 - loc_x_0 * vel_z_0 - loc_z * vel_x + loc_x * vel_z,
            -loc_z_0 + loc_z,
            vel_z_0 - vel_z,
            loc_x_0 - loc_x
        )
        .unwrap();
        writeln!(
            &mut output,
            "{:+}*x {:+} {:+}*b {:+}*y {:+}*a,",
            -vel_y_0 + vel_y,
            loc_x_0 * vel_y_0 - loc_y_0 * vel_x_0 - loc_x * vel_y + loc_y * vel_x,
            -loc_x_0 + loc_x,
            vel_x_0 - vel_x,
            loc_y_0 - loc_y
        )
        .unwrap();
    }
    writeln!(&mut output, "], [x, y, z, a, b, c], set=True)").unwrap();
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
