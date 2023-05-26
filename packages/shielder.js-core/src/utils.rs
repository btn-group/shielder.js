use serde::{Deserialize, Serialize};
use serde_json;
use utils::*;
use wasm_bindgen::prelude::*;
use ark_serialize::CanonicalDeserialize;
use js_sys::Uint8Array;
use js_sys::Object;
use liminal_ark_relations::environment::{Groth16, ProvingSystem};
use liminal_ark_relations::serialization::serialize;
use liminal_ark_relations::shielder::types::{FrontendNullifier, FrontendTrapdoor};
use liminal_ark_relations::shielder::{compute_note, DepositRelationWithFullInput};
use rand::Rng;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, ReadableStreamDefaultReader, Request, RequestInit, RequestMode, Response};
pub const DEPOSIT_PK_URL: String = "https://bafybeifjsmmivrq2hxmujww7t2t3prbu3r2vffh7bq7pf6su2v5qgmzd4u.ipfs.w3s.link/ipfs/bafybeifjsmmivrq2hxmujww7t2t3prbu3r2vffh7bq7pf6su2v5qgmzd4u/deposit.pk.bytes".to_string();

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub async fn fetch_pk_bytes(url: String) -> Vec<u8> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .expect("error while fetching");

    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();

    let reader_value = resp.body().unwrap().get_reader();
    let reader: ReadableStreamDefaultReader = reader_value.dyn_into().unwrap();

    let result_value = JsFuture::from(reader.read()).await.expect("errr");

    let result: Object = result_value.dyn_into().unwrap();
    let chunk_value = js_sys::Reflect::get(&result, &JsValue::from_str("value")).unwrap();
    let chunk_array: Uint8Array = chunk_value.dyn_into().unwrap();
    let chunk = chunk_array.to_vec();

    return chunk;
}