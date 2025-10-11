import 'package:day_01/day_01.dart';
import 'package:test/test.dart';

void main() {
  test('part 1 example 1', () {
    expect(part1("R2, L3"), 5);
  });

  test('part 1 example 2', () {
    expect(part1("R2, R2, R2"), 2);
  });

  test('part 1 example 3', () {
    expect(part1("R5, L5, R5, R3"), 12);
  });

  test('part 2 example 1', () {
    expect(part2("R8, R4, R4, R8"), 4);
  });
}
