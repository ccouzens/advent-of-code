import 'dart:math';

int part1(String input) {
  var sum = 0;
  for (final s in input.split(',')) {
    final [start, end] = s.split('-').take(2).map(int.parse).toList();
    for (var i = start; i <= end; i++) {
      var digits = (log(i) / ln10).ceil();
      if (digits.isOdd) {
        i = pow(10, digits).ceil();
        continue;
      }

      var divider = pow(10, digits / 2).ceil();

      if (i ~/ divider == i % divider) {
        sum += i;
      }
    }
  }
  return sum;
}
