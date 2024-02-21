use wasm_bindgen::prelude::*;
mod blowfish_crypto;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn get_fingerprint(cipher_text: &str) -> Option<String> {
    let decrypted: Result<String, String> = blowfish_crypto::twice_decrypt(cipher_text, &"12345678");
    if decrypted.is_err() {
        log(&format!("decrypted error: {:?}", decrypted.err()));
        return None;
    }
    let decrypted = decrypted.unwrap();
    // log(&format!("decrypted: {}", &decrypted));

    Some(decrypted)
}