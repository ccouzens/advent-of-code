import 'package:day_02/day_02.dart';
import 'package:test/test.dart';

void main() {
  var example1 =
      "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
  test('part 1 example 1', () {
    expect(part1(example1), 1227775554);
  });
}
