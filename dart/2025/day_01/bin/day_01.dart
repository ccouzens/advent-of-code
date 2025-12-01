import 'dart:convert';
import 'dart:io';

import 'package:day_01/day_01.dart' as day_01;

void main(List<String> arguments) async {
  List<String> sequence = await stdin
      .transform(utf8.decoder)
      .transform(const LineSplitter())
      .takeWhile((l) => l.isNotEmpty)
      .toList();
  print(day_01.part1(sequence));
  print(day_01.part2(sequence));
}
