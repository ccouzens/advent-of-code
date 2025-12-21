import 'dart:math';

int countDigits(int productId) {
  var digits = 0;
  while (productId != 0) {
    productId ~/= 10;
    digits ++;
  }
  return digits;

}

bool part1Invalid(int productId) {
  final digits = countDigits(productId);
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
  }
  return sum;
}

bool part2Invalid(int productId) {
  final digits = countDigits(productId);
  sequenceLengthLoop:
  for (var i = 1; i <= digits / 2; i++) {
    if (digits % i != 0) {
      continue sequenceLengthLoop;
    }
    final divider = pow(10, i).ceil();
    final target = productId % divider;
    var divided = productId;
    for (var j = i; j < digits; j += i) {
      divided ~/= divider;
      if (divided % divider != target) {
        continue sequenceLengthLoop;
      }
    }
    return true;
  }
  return false;
}

int part2(String input) {
  var sum = 0;
  for (final s in input.split(',')) {
    final [start, end] = s.split('-').take(2).map(int.parse).toList();
    sum += Iterable.generate(end - start + 1)
        .map<int>((i) => i + start)
        .where(part2Invalid)
        .fold(0, (sum, i) => sum + i);
  }
  return sum;
}
