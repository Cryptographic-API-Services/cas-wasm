import init, { RSAWrapper } from "./pkg/cas_wasm.js";

async function run() {
  const output = document.getElementById("output");
  try {
    await init();
    const rsaWrapper = new RSAWrapper();
    console.log(rsaWrapper);
    const keyPair = rsaWrapper.generateRsaKeys(1024);
    const message = Array.from(new TextEncoder().encode("Hello, World!"));
    const signature = rsaWrapper.sign(keyPair.privateKey, message);
    const isValid = rsaWrapper.verify(keyPair.publicKey, message, signature);
    output.textContent = `Signature valid: ${isValid}`;
  } catch (err) {
    output.textContent = `error: ${err}`;
  }
}

run();
