use js_sys::Object;
use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use web_sys::window;
use web_sys::CryptoKey;
use web_sys::{Crypto, SubtleCrypto, AesCbcParams};
use serde_json;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;
// use wasm_bindgen::JsCast;
// use aes_gcm_siv::{
//     aead::{Aead, KeyInit, OsRng},
//     Aes256GcmSiv, Nonce // Or `Aes128GcmSiv`
// };


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// struct Object {
//     data: String
// }

async fn await_promise(promise_result: Result<Promise, JsValue>) -> Result<JsValue, JsValue> {
    let promise = promise_result?;
    let result = JsFuture::from(promise).await?;
    Ok(result)
}

async fn encrypt(data: &str, key: &CryptoKey) -> Result<String, JsValue> {
    let crypto: Crypto = web_sys::window().unwrap().crypto().unwrap();
    let subtle: SubtleCrypto = crypto.subtle();
    let x = "xxx";
    // let algorithm = AesCbcParams::new("AES-CBC", iv.into());
    let data_object = Object::new();
    Reflect::set(&data_object, &"data".into(), &data.into()).unwrap();
    // data_object.create("data", data);
    // data_object.data = data;
    let encryptedData = subtle.encrypt_with_str_and_buffer_source("AES-CBC", key, &data_object);
    
    let result: Result<JsValue, JsValue> = await_promise(encryptedData).await;

    result
}

// fn encrypt(data: &String) -> String {
//     let key = Aes256GcmSiv::generate_key(&mut OsRng);
//     let cipher = Aes256GcmSiv::new(&key);
//     let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
//     // let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())?;
//     let ciphertext = match cipher.encrypt(nonce, data.as_bytes()) {
//         Ok(ciphertext) => ciphertext,
//         Err(_) => return String::new(), // 如果加密失败，则返回空字符串
//     };
//     base64::encode(&ciphertext)
// }


#[wasm_bindgen]
pub fn check_environment() {
    let environment = if cfg!(target_arch = "wasm32") {
        "Browser"
    } else {
        "Non-Browser"
    };

    log(&format!("Current environment: {}", environment));
}

#[wasm_bindgen]
pub fn get_browser_info() -> Option<String> {
    if let Some(window) = window() {
        
        let navigator = window.navigator();
        let user_agent = navigator.user_agent().unwrap();
        let string1 = String::from("aabbcc");
        let string2 = String::from(" 112233!");
        return Some(string1 + &string2 + &user_agent);
        // let key = String::from("abcdefghijklmnopqrstuvwx");
        // let iv = String::from("1234567890123456");
        // let ss = string1 + &string2 + &user_agent;
        // let result = encrypt(&ss);
        // return Some(result);
    }

    None
}
