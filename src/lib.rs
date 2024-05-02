use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    pub fn print_line(line: &str);
}

#[wasm_bindgen]
pub fn alert_rs(msg: &str) {
    log(msg);
}

#[wasm_bindgen]
pub fn print_file(data: &str) {
    log(data);
}

#[wasm_bindgen]
pub fn render_file(data: &str) {
    print_line(data);
}

