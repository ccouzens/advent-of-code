import 'package:day_02/day_02.dart' as day_02;

import 'dart:convert';
import 'dart:io';

void main(List<String> arguments) async {
  String sequence = await stdin
      .transform(utf8.decoder)
      .transform(const LineSplitter())
      .first;
  print(day_02.part1(sequence));
}
