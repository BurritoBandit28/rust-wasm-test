use wasm_bindgen::prelude::*;
use password_validator::check_password_strength;


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = window)]
    pub fn prompt(s: &str, d : &str) -> String;
}

#[wasm_bindgen]
pub fn do_password_check() {
    let password = prompt("Enter your password: ", "");
    let feedback = check_password_strength(&password);
    alert(format!("{}. Score : {}", feedback.message, feedback.code).as_str())
}
