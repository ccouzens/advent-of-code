import 'dart:io';

import 'package:day_01/day_01.dart' as day_01;

void main(List<String> arguments) {
  String sequence = stdin.readLineSync() ?? '';
  print(day_01.part1(sequence));
}
