import fs from "node:fs/promises";
import { compute, imports, type ComputerExports } from "./script";

let computer: undefined | ComputerExports;

beforeAll(async () => {
  const buffer = await fs.readFile(`${__dirname}/compute.wasm`);
  computer = (await WebAssembly.instantiate(buffer, imports)).instance
    .exports as unknown as ComputerExports;
});

const testCase = `
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
`;

test("problem example gives 35", async () => {
  expect(compute(testCase, computer!)).toEqual(35n);
});
