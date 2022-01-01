use wasm_bindgen::prelude::*;

mod utl;

#[wasm_bindgen]
extern{
    pub fn alert( s: &str );
    #[wasm_bindgen(js_namespace = console)]
    pub fn log( s: &str);
}

#[wasm_bindgen]
pub fn greet( name : &str ){
    alert(&format!("Hello, {}!", name ));
}

#[wasm_bindgen]
pub fn console_log( s : &str){
    log( s );
}

#[wasm_bindgen]
pub fn display_fibonacci_seq( count : u32 ){
    log ( utl::get_fibonacci_string(count as usize).as_str() );
}