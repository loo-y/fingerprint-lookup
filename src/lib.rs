use js_sys::Object;
use js_sys::Reflect;
use wasm_bindgen::convert::IntoWasmAbi;
use wasm_bindgen::prelude::*;
use web_sys::window;
use web_sys::CryptoKey;
use web_sys::{Crypto, SubtleCrypto, AesCbcParams};
use serde_json;
use base64;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;
// use wasm_bindgen::JsCast;
// use aes_gcm_siv::{
//     aead::{Aead, KeyInit, OsRng},
//     Aes256GcmSiv, Nonce // Or `Aes128GcmSiv`
// };

mod blowfish_crypto;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// struct Object {
//     data: String
// }

async fn await_promise(promise_result: Result<Promise, JsValue>) -> Result<JsValue, JsValue> {
    match promise_result {
        Ok(promise) => {
            // let promise = promise_result?;
            let result = JsFuture::from(promise).await?;
            Ok(result)
        },
        Err(err) => {
            return Err(err);
        }   
    }
}

async fn encrypt(data: &str) -> String {
    let crypto: Crypto = web_sys::window().unwrap().crypto().unwrap();
    let subtle: SubtleCrypto = crypto.subtle();
    let algorithm =  Object::new();
    Reflect::set(&algorithm, &"name".into(), &"RSA-OAEP".into()).unwrap();
    let key_pair = subtle.generate_key_with_object(
        &algorithm,
        true,
        &JsValue::from_str("encrypt, decrypt"),
    );
    let key_pair_promise = key_pair.unwrap();
    let key_pair_end = JsFuture::from(key_pair_promise).await.unwrap();

    let key: CryptoKey = CryptoKey::from(key_pair_end);

    log(&format!("CryptoKey: {:?}", &key));

    // let algorithm = AesCbcParams::new("AES-CBC", iv.into());
    let data_object = Object::new();
    Reflect::set(&data_object, &"data".into(), &data.into()).unwrap();
    // data_object.create("data", data);
    // data_object.data = data;
    let encrypted_data: Result<Promise, JsValue> = subtle.encrypt_with_str_and_buffer_source("RSA-OAEP", &key, &data_object);
    
    let result: Result<JsValue, JsValue> = await_promise(encrypted_data).await;

    let ciphertext: String = match result {
        Ok(value) => {
            let result_string = js_sys::JSON::stringify(&value).unwrap();
            // Ok(result_string.into())
            result_string.into()
        },
        Err(err) => {
            log(std::format!("Error: {:?}", err).as_str());
            // Err(String::new().into()) // 如果加密失败，则返回空字符串
            String::new().into()
        }
    };

    base64::encode(&ciphertext)
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
        let user_agent_start = String::from("user_agent_start ======> ");
        let user_agent_end = String::from(" <====== user_agent_end");
        
        let user_agent_statment = String::new() + &user_agent_start + &user_agent + &user_agent_end;

        log(&format!("original user_agent_statment: {}", &user_agent_statment));

        let user_agent_statment_encrypted = blowfish_crypto::twice_encrypt(&user_agent_statment, "12345678");
        if user_agent_statment_encrypted.is_err() {
            log(&format!("user_agent_statment_encrypted error: {:?}", user_agent_statment_encrypted.err()));
            return None;
        }

        let user_agent_statment_encrypted = user_agent_statment_encrypted.unwrap();
        log(&format!("user_agent_statment_encrypted: {}", &user_agent_statment_encrypted));

        let user_agent_statment_decrypted = blowfish_crypto::twice_decrypt(&user_agent_statment_encrypted, "12345678").unwrap();
        log(&format!("user_agent_statment_decrypted: {}", &user_agent_statment_decrypted));

        return Some(user_agent_statment_encrypted);

        // let encrypted_ua_statement = encrypt(&user_agent_statment).await;

        // return Some(encrypted_ua_statement);
        // let key = String::from("abcdefghijklmnopqrstuvwx");
        // let iv = String::from("1234567890123456");
        // let ss = string1 + &string2 + &user_agent;
        // let result = encrypt(&ss);
        // return Some(result);
    }

    None
}
