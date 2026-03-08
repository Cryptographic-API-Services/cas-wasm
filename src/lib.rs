use cas_lib::hashers::cas_hasher::CASHasher;
use cas_lib::hashers::sha::CASSHA;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sha3_256_hex(input: Vec<u8>) -> Vec<u8> {
    let digest = <CASSHA as CASHasher>::hash_256(input);
    digest
}