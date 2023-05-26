mod utils;
use utils::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde_json;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// #[wasm_bindgen]
// pub async fn fetch_pk(url: String) -> Result<JsValue, JsValue> {
//     let mut opts = RequestInit::new();
//     opts.method("GET");
//     opts.mode(RequestMode::Cors);
    
//     let request = Request::new_with_str_and_init(&url, &opts)?;
    
//     // request
//     //     .headers()
//     //     .set("Accept", "application/vnd.github.v3+json")?;
    
//     let window = web_sys::window().unwrap();
//     let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    
//     // `resp_value` is a `Response` object.
//     assert!(resp_value.is_instance_of::<Response>());
//     let resp: Response = resp_value.dyn_into().unwrap();

//     // Convert this other `Promise` into a rust `Future`.
//     let json = JsFuture::from(resp.json()?).await?;
    
//     // Send the JSON response back to JS.
//     Ok(json)
// }


#[wasm_bindgen]
pub fn json_test_string(arg: String) -> String {
    let mut val: TestJSON = serde_json::from_str(&arg).unwrap();
    val.num += 5;
    return serde_json::to_string(&val).unwrap_or(String::from("ERROR_WASM"));
}