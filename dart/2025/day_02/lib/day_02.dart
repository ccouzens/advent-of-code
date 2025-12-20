import 'dart:math';

bool part1Invalid(int productId) {
  final digits = (log(productId) / ln10).ceil();
  if (digits.isOdd) {
    productId = pow(10, digits).ceil();
    return false;
  }

  final divider = pow(10, digits / 2).ceil();

  return productId ~/ divider == productId % divider;
}

int part1(String input) {
  var sum = 0;
  for (final s in input.split(',')) {
    final [start, end] = s.split('-').take(2).map(int.parse).toList();
    sum += Iterable.generate(end - start + 1)
        .map<int>((i) => i + start)
        .where(part1Invalid)
        .fold(0, (sum, i) => sum + i);
    ;
  }
  return sum;
}
