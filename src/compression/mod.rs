use wasm_bindgen::prelude::*;
use cas_lib::compression::zstd::{compress as zstd_compress, decompress as zstd_decompress};


#[wasm_bindgen]
pub struct ZTSDWrapper;

#[wasm_bindgen]
impl ZTSDWrapper {

    #[wasm_bindgen(constructor)]
    pub fn new() -> ZTSDWrapper {
        ZTSDWrapper
    }

    /// Compresses the byte array with the specified Zstandard (zstd) compression level.
    /// Zstandard (zstd) supports 22 compression levels, ranging from -22 to 22. Lower levels, such as 1–9, 
    /// are faster but result in larger file sizes, while higher levels, such as 10–22, provide better compression ratios.
    #[wasm_bindgen(js_name = compress)]
    pub fn compress(&self, data: Vec<u8>, level: i32) -> Vec<u8> {
        if level < -22 || level > 22 {
            panic!("Invalid compression level. Level must be between -22 and 22.");
        }
        let compressed_data = zstd_compress(data, level);
        compressed_data
    }

    /// Decompresses a previously compressed byte array with ZSTD.
    /// No level is required to decompress.
    #[wasm_bindgen(js_name = decompress)]
    pub fn decompress(&self, compressed_data: Vec<u8>) -> Vec<u8> {
        let decompressed_data = zstd_decompress(compressed_data);
        decompressed_data
    }
}
