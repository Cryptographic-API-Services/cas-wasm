use cas_lib::asymmetric::{
    cas_rsa::CASRSA,
    types::{CASRSAEncryption, RSAKeyPairResult},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RSAKeyPair {
    private_key: String,
    public_key: String,
}

impl From<RSAKeyPairResult> for RSAKeyPair {
    fn from(value: RSAKeyPairResult) -> Self {
        Self {
            private_key: value.private_key,
            public_key: value.public_key,
        }
    }
}

#[wasm_bindgen]
impl RSAKeyPair {
    #[wasm_bindgen(getter, js_name = privateKey)]
    pub fn private_key(&self) -> String {
        self.private_key.clone()
    }

    #[wasm_bindgen(getter, js_name = publicKey)]
    pub fn public_key(&self) -> String {
        self.public_key.clone()
    }
}

#[wasm_bindgen]
pub struct RSAWrapper;

#[wasm_bindgen]
impl RSAWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RSAWrapper {
        RSAWrapper
    }

    /// Generates an RSA key pair based of parameter sent in 1024, 2048, and 4096 are supported.
    #[wasm_bindgen(js_name = generateRsaKeys)]
    pub fn generate_rsa_keys(&self, key_size: usize) -> RSAKeyPair {
        CASRSA::generate_rsa_keys(key_size).into()
    }

    /// Signs a byte array with an RSA private key for verification.
    #[wasm_bindgen(js_name = sign)]
    pub fn sign(&self, private_key: String, data: Vec<u8>) -> Vec<u8> {
        CASRSA::sign(private_key, data)
    }

    /// Verifies signed data by the corresponding private key with an RSA public key.
    #[wasm_bindgen(js_name = verify)]
    pub fn verify(&self, public_key: String, data: Vec<u8>, signature: Vec<u8>) -> bool {
        CASRSA::verify(public_key, data, signature)
    }
}
