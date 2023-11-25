export interface FloorCalculatorExports {
  mem: WebAssembly.Memory;
  floorCalculator: (length: number) => number;
}

const encoder = new TextEncoder();
const input = document.getElementById("input")! as HTMLTextAreaElement;
const output = document.getElementById("output")! as HTMLPreElement;
const button = document.getElementById("submit")! as HTMLButtonElement;

const module = await WebAssembly.instantiateStreaming(fetch("2015-01-1.wasm"));
const instanceExports = module.instance
  .exports as unknown as FloorCalculatorExports;
function eventListener() {
  const directions = encoder.encode(input.value.trim());
  new Uint8Array(instanceExports.mem.buffer, 0, directions.length).set(
    directions,
  );
  output.textContent = `${instanceExports.floorCalculator(directions.length)}`;
}

input.addEventListener("change", eventListener);
button.addEventListener("click", eventListener);
eventListener();

export {};
