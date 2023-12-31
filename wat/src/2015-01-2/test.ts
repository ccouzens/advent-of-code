import fs from "node:fs/promises";
import type { FloorCalculatorExports } from "./script";

let instanceExports: undefined | FloorCalculatorExports;
const encoder = new TextEncoder();

beforeAll(async () => {
  const buffer = await fs.readFile(`${__dirname}/compute.wasm`);
  const module = await WebAssembly.instantiate(buffer);
  instanceExports = module.instance
    .exports as unknown as FloorCalculatorExports;
});

test.each<[string, number]>([
  [")", 1],
  ["()())", 5],
])("calculatePosition(%p) === %i", (directions, expected) => {
  const directionsBytes = encoder.encode(directions);
  const memBuffer = new Uint8Array(
    instanceExports!.mem.buffer,
    0,
    directionsBytes.length,
  );
  memBuffer.set(directionsBytes);
  expect(instanceExports!.calculatePosition(directionsBytes.length)).toEqual(
    expected,
  );
});
