import init, { ZTSDWrapper } from "./pkg/cas_wasm.js";

async function run() {
  const output = document.getElementById("output");
  try {
    await init();
    const ztsdWrapper = new ZTSDWrapper();
    console.log(ztsdWrapper);
    const dataToCompress = Array.from(new TextEncoder().encode("Hello, World! lets compress this data"));
    const compressedData = ztsdWrapper.compress(dataToCompress, 3);
    output.textContent = `Compressed data: ${compressedData}`;
    console.log(dataToCompress.length > compressedData.length ? "Compression successful!" : "Compression failed.");
    
  } catch (err) {
    output.textContent = `error: ${err}`;
  }
}

run();
