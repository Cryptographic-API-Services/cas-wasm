import init, { sha3_256_hex } from "./pkg/cas_wasm.js";

async function run() {
  const output = document.getElementById("output");
  try {
    await init();
    output.textContent = sha3_256_hex("hello");
  } catch (err) {
    output.textContent = `error: ${err}`;
  }
}

run();
