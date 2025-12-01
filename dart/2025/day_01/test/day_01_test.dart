import 'package:day_01/day_01.dart';
import 'package:test/test.dart';

void main() {
  test('part 1 example 1', () {
    expect(
      part1([
        "L68",
        "L30",
        "R48",
        "L5",
        "R60",
        "L55",
        "L1",
        "L99",
        "R14",
        "L82",
      ]),
      3,
    );
  });

  test('part 2 example 1', () {
    expect(
      part2([
        "L68",
        "L30",
        "R48",
        "L5",
        "R60",
        "L55",
        "L1",
        "L99",
        "R14",
        "L82",
      ]),
      6,
    );
  });
}
