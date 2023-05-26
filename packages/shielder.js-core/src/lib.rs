mod utils;
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

#[wasm_bindgen]
pub async fn run_prover() {
    let url = "https://bafybeifjsmmivrq2hxmujww7t2t3prbu3r2vffh7bq7pf6su2v5qgmzd4u.ipfs.w3s.link/ipfs/bafybeifjsmmivrq2hxmujww7t2t3prbu3r2vffh7bq7pf6su2v5qgmzd4u/deposit.pk.bytes".to_string();
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

    let token_id = 0;
    let amount = 1;
    let (trapdoor, nullifier) = rand::thread_rng().gen::<(FrontendTrapdoor, FrontendNullifier)>();
    console::log_1(
        &format!(
            "Trapdoor: {:?}, Nullifier: {:?}",
            trapdoor.clone(),
            nullifier.clone()
        )
        .into(),
    );
    let note = compute_note(token_id, amount, trapdoor, nullifier);
    console::log_1(&format!("Note: {:?}", note).into());
    let circuit = DepositRelationWithFullInput::new(note, token_id, amount, trapdoor, nullifier);
    let pk = <<Groth16 as ProvingSystem>::ProvingKey>::deserialize(&*chunk).unwrap();
    let proof = serialize(&Groth16::prove(&pk, circuit));
    console::log_1(&format!("Proof: {:?}", hex::encode(proof.to_vec())).into());
}

#[wasm_bindgen]
pub fn json_test_string(arg: String) -> String {
    let mut val: TestJSON = serde_json::from_str(&arg).unwrap();
    val.num += 5;
    return serde_json::to_string(&val).unwrap_or(String::from("ERROR_WASM"));
}
