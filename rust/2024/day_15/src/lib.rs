mod part_1;
mod part_2;

pub use part_1::part_1;
pub use part_2::part_2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(include_str!("../example_2.txt")), 2028);
        assert_eq!(part_1(include_str!("../example_1.txt")), 10092);
    }

    #[test]
    fn challenge_part_1() {
        assert_eq!(part_1(include_str!("../input.txt")), 1412971);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(include_str!("../example_1.txt")), 9021);
    }

    #[test]
    fn challenge_part_2() {
        assert_eq!(part_2(include_str!("../input.txt")), 1429299);
    }
}
