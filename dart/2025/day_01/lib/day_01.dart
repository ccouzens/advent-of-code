int part1(List<String> sequence) {
  int dial = 50;
  int zeros = 0;
  for (var rotation in sequence) {
    int direction = rotation[0] == "L" ? -1 : 1;
    int magnitude = int.parse(rotation.substring(1));
    dial = (dial + direction * magnitude) % 100;
    if (dial == 0) {
      zeros += 1;
    }
  }
  return zeros;
}

int part2(List<String> sequence) {
  int dial = 50;
  int zeros = 0;
  for (var rotation in sequence) {
    int direction = rotation[0] == "L" ? -1 : 1;
    int magnitude = int.parse(rotation.substring(1));
    zeros += magnitude ~/ 100;

    int unmodulatedPostRotation = dial + direction * (magnitude % 100);
    if (unmodulatedPostRotation >= 100 ||
        (unmodulatedPostRotation <= 0 && dial != 0)) {
      zeros++;
    }
    dial = unmodulatedPostRotation % 100;
  }
  return zeros;
}
