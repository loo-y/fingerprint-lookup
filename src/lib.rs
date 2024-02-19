use wasm_bindgen::prelude::*;
use web_sys::window;

mod blowfish_crypto;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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
        
        let current_location = window.location();
        let current_page_url = current_location.href().unwrap();
        log(&format!("current_page_url: {}", &current_page_url));
        let navigator: web_sys::Navigator = window.navigator();
        let user_agent = navigator.user_agent().unwrap();
        let user_agent_start = String::from("user_agent_start ======> ");
        let user_agent_end = String::from(" <====== user_agent_end");
        
        let user_agent_statment = String::new() + &current_page_url + &user_agent_start + &user_agent + &user_agent_end;

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
    }

    None
}
