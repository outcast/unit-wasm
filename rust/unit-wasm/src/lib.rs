extern crate js_sys;
extern crate console_error_panic_hook;

mod utils;

use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use std::convert::Into;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    pub type IncomingMessage;

    #[wasm_bindgen(method, getter)]
    fn headers(this: &IncomingMessage) -> JsValue;

    #[wasm_bindgen(method, setter)]
    fn set_headers(this: &IncomingMessage, headers: JsValue);
}

#[wasm_bindgen]
extern {
    pub type ServerResponse;
    
    #[wasm_bindgen(method)]
    fn write(this: &ServerResponse, data: String);

    #[wasm_bindgen(method)]
    fn writeHead(this: &ServerResponse, status_code: u32, message: String, headers: JsValue);

    #[wasm_bindgen(method)]
    fn end(this: &ServerResponse, data: String);

}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn handle_request(req: IncomingMessage, res: ServerResponse) {
    utils::set_panic_hook();
    let headers_in: HashMap<String, String> = req.headers().into_serde().unwrap();
    for (k,v) in headers_in {
        println!("{}=>{}", k,v);
    }
    let mut headers_out = HashMap::new();
    headers_out.insert("Content-Type", "text/plain");
    res.writeHead(200, "OK".into(), JsValue::from_serde(&headers_out).unwrap());
    res.write("Hello WASM!\n\n".into());
}
