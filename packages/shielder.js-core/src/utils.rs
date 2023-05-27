use js_sys::Object;
use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ReadableStreamDefaultReader, Request, RequestInit, RequestMode, Response};

// pub const DEPOSIT_PK_URL: &str = "https://bafybeifjsmmivrq2hxmujww7t2t3prbu3r2vffh7bq7pf6su2v5qgmzd4u.ipfs.w3s.link/ipfs/bafybeifjsmmivrq2hxmujww7t2t3prbu3r2vffh7bq7pf6su2v5qgmzd4u/deposit.pk.bytes";
// pub const WITHDRAW_PK_URL: &str = "https://bafybeiffwdejn5skbc5kkd4x53zaol2gk2h6fm5pwp7mfc5a6zfg7pu4f4.ipfs.w3s.link/ipfs/bafybeiffwdejn5skbc5kkd4x53zaol2gk2h6fm5pwp7mfc5a6zfg7pu4f4/withdraw.pk.bytes";
pub const MERKLE_PATH_MAX_LEN: u8 = 16;
//pub conts MERKLE_PATH_MAX_LEN: u8 = 16;
pub fn _set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub async fn _fetch_pk_bytes(url: String) -> Vec<u8> {
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
#[derive(Serialize, Deserialize)]
pub struct PrepareDeposit {
    pub deposit_id: u16,
    pub token_id: u16,
    pub token_amount: u128,
}
#[derive(Serialize, Deserialize)]
pub struct Deposit {
    pub deposit_id: u16,
    pub token_id: u16,
    pub token_amount: u128,
    pub leaf_idx: u32,
    pub trapdoor: [u64; 4],
    pub nullifier: [u64; 4],
    pub note: [u64; 4],
    pub proof: String,
}

#[derive(Serialize, Deserialize)]
pub struct Withdraw {
    pub deposit: Deposit,
    pub withdraw_amount: u128,
    pub recipient: [u8; 32],
    pub fee: u128,
    pub merkle_root: [u64; 4],
    pub merkle_path: Vec<[u64; 4]>,
}

#[derive(Serialize, Deserialize)]
pub struct PkBytes {
    pub nested: Vec<u8>,
}
