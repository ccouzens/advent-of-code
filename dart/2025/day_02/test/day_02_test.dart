import 'package:day_02/day_02.dart';
import 'package:test/test.dart';

void main() {
  var example1 =
      "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
  test('part 1 example 1', () {
    expect(part1(example1), 1227775554);
  });

  group("part 2 invalid", () {
    for (var productId in [11, 2121212121, 2222]) {
      test("$productId", () {
        expect(part2Invalid(productId), true);
      });
    }
  });

  group("countDigits", () {
    for (var [num, digits] in [[9, 1], [10, 2], [11, 2], [99, 2], [100, 3], [101, 3], [999, 3], [1000, 4], [1001, 4]]) {
      test("countDigits($num) = $digits", () {
        expect(countDigits(num), digits);
      });
    }
  });

  group("part 2 valid", () {
    for (var productId in [12, 1000, 3222]) {
      test("$productId", () {
        expect(part2Invalid(productId), false);
      });
    }
  });

  test('part 2 example 1', () {
    expect(part2(example1), 4174379265);
  });
}
