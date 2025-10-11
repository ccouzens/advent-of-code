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
