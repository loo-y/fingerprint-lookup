use wasm_bindgen::prelude::*;
use web_sys::window;
mod blowfish_crypto;
use serde::{Deserialize, Serialize};
use serde_json;
use js_sys::Date;


#[derive(Serialize, Deserialize)]
struct BrowserInfo {
    is_browser: bool,
    user_agent: String,
    page_url: String,
    timestamp: u64,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn check_environment() -> bool {
    let is_browser = cfg!(target_arch = "wasm32");
    let environment = if cfg!(target_arch = "wasm32") {
        "Browser"
    } else {
        "Non-Browser"
    };

    log(&format!("Current environment: {}", environment));
    
    is_browser
}

pub fn get_current_time() -> u64 {
    let js_date = Date::new_0();
    let timestamp = js_date.get_time() as u64;
    timestamp
}

#[wasm_bindgen]
pub fn get_browser_info() -> Option<String> {
    if let Some(window) = window() {
        let is_browser = check_environment();
        let current_location = window.location();
        let current_page_url = current_location.href().unwrap();
        log(&format!("current_page_url: {}", &current_page_url));
        let navigator: web_sys::Navigator = window.navigator();
        let user_agent = navigator.user_agent().unwrap();
        // let user_agent_start = String::from("user_agent_start ======> ");
        // let user_agent_end = String::from(" <====== user_agent_end");
        // let user_agent_statment = String::new() + &current_page_url + &user_agent_start + &user_agent + &user_agent_end;

        let now = get_current_time();

        let browser_info_json = BrowserInfo {
            is_browser,
            user_agent: user_agent,
            page_url: current_page_url,
            timestamp: now,
        };

        let browser_info = serde_json::to_string(&browser_info_json).unwrap();

        // log(&format!("original browser_info: {}", &browser_info));

        let browser_info_encrypted = blowfish_crypto::twice_encrypt(&browser_info, None);
        if browser_info_encrypted.is_err() {
            log(&format!("browser_info_encrypted error: {:?}", browser_info_encrypted.err()));
            return None;
        }

        let browser_info_encrypted = browser_info_encrypted.unwrap();
        log(&format!("browser_info_encrypted: {}", &browser_info_encrypted));

        let browser_info_decrypted = blowfish_crypto::twice_decrypt(&browser_info_encrypted, None).unwrap();
        log(&format!("browser_info_decrypted: {}", &browser_info_decrypted));

        return Some(browser_info_encrypted);
    }

    None
}
