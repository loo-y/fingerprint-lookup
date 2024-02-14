use wasm_bindgen::prelude::*;
use web_sys::window;

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
        
        let navigator = window.navigator();
        let user_agent = navigator.user_agent().unwrap();
        let string1 = String::from("aabbcc");
        let string2 = String::from(" 112233!");
        return Some(string1 + &string2 + &user_agent);
    }

    None
}
