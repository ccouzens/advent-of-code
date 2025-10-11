int part1(String sequence) {
  int direction = 0;
  int x = 0;
  int y = 0;
  for (var instruction in sequence.split(", ")) {
    direction = (direction + (instruction[0] == "L" ? -1 : 1)) % 4;
    var magnitude = int.parse(instruction.substring(1));
    x += magnitude * (direction % 2) * ((direction + 2) % 4 - 2);
    y += magnitude * ((direction + 1) % 2) * ((direction + 1) % 4 - 2);
  }
  return x.abs() + y.abs();
}

int part2(String sequence) {
  int direction = 0;
  int x = 0;
  int y = 0;
  var visited = {(0, 0)};
  sequenceLoop:
  for (var instruction in sequence.split(", ")) {
    direction = (direction + (instruction[0] == "L" ? -1 : 1)) % 4;
    var magnitude = int.parse(instruction.substring(1));
    for (int i = 0; i < magnitude; i++) {
      x += (direction % 2) * ((direction + 2) % 4 - 2);
      y += ((direction + 1) % 2) * ((direction + 1) % 4 - 2);
      if (visited.contains((x, y))) {
        break sequenceLoop;
      }
      visited.add((x, y));
    }
  }
  return x.abs() + (y.abs());
}
