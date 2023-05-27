mod utils;
use ark_serialize::CanonicalDeserialize;
use hex;
use liminal_ark_relations::environment::{Groth16, ProvingSystem};
use liminal_ark_relations::serialization::serialize;
use liminal_ark_relations::shielder::types::{FrontendNullifier, FrontendTrapdoor};
use liminal_ark_relations::shielder::{
    compute_note, DepositRelationWithFullInput, WithdrawRelationWithFullInput,
};
use rand::Rng;
use serde_json;
use utils::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn bar() -> String {
    return "OK".to_string();
}

#[wasm_bindgen]
pub async fn deposit(deposit_data_string: String, pk_bytes_string: String) -> String {
    console::log_1(&"[CORE_DEBUG] START".into());
    let pk_bytes_lol: PkBytes = serde_json::from_str(&pk_bytes_string).unwrap();
    let pk_bytes = pk_bytes_lol.nested;
    // let pk_bytes: Vec<u8> = fetch_pk_bytes(DEPOSIT_PK_URL.to_string()).await;
    console::log_1(&format!("[CORE_DEBUG] PK_BYTES_LENGTH: {:?}", pk_bytes.len()).into());
    let prepare_deposit_data: PrepareDeposit = serde_json::from_str(&deposit_data_string).unwrap();
    let (trapdoor, nullifier) = rand::thread_rng().gen::<(FrontendTrapdoor, FrontendNullifier)>();
    console::log_1(&"[CORE_DEBUG] TRAPDOOR AND NULLIFIER GENERATED".into());

    let note = compute_note(
        prepare_deposit_data.token_id,
        prepare_deposit_data.token_amount,
        trapdoor,
        nullifier,
    );

    console::log_1(&"[CORE_DEBUG] NOTE COMPUTED".into());
    let circuit = DepositRelationWithFullInput::new(
        note,
        prepare_deposit_data.token_id,
        prepare_deposit_data.token_amount,
        trapdoor,
        nullifier,
    );
    let pk = <<Groth16 as ProvingSystem>::ProvingKey>::deserialize(&*pk_bytes);
    console::log_1(&"[CORE_DEBUG] PK SERIALIZED".into());
    let pk_unwrapped = pk.unwrap();
    let proof = hex::encode(serialize(&Groth16::prove(&pk_unwrapped, circuit)));
    console::log_1(&"[CORE_DEBUG] PROOF COMPUTED".into());
    let deposit_data = Deposit {
        deposit_id: prepare_deposit_data.deposit_id,
        token_id: prepare_deposit_data.token_id,
        token_amount: prepare_deposit_data.token_amount,
        leaf_idx: 0,
        trapdoor,
        nullifier,
        note,
        proof,
    };
    return serde_json::to_string(&deposit_data).unwrap_or(String::from("ERROR_WASM"));
}

#[wasm_bindgen]
pub async fn withdraw(withdraw_data_string: String, pk_bytes_string: String) -> String {
    console::log_1(&"[CORE_DEBUG] START".into());
    let pk_bytes_lol: PkBytes = serde_json::from_str(&pk_bytes_string).unwrap();
    let pk_bytes: Vec<u8> = pk_bytes_lol.nested;
    console::log_1(&format!("[CORE_DEBUG] PK_BYTES_LENGTH: {:?}", pk_bytes.len()).into());

    let withdraw_data: Withdraw = serde_json::from_str(&withdraw_data_string).unwrap();
    let mut deposit_data: Deposit = withdraw_data.deposit;

    let (new_trapdoor, new_nullifier) =
        rand::thread_rng().gen::<(FrontendTrapdoor, FrontendNullifier)>();
    console::log_1(&"[CORE_DEBUG] TRAPDOOR AND NULLIFIER GENERATED".into());

    let new_token_amount = deposit_data.token_amount - withdraw_data.withdraw_amount;
    let new_note = compute_note(
        deposit_data.token_id,
        new_token_amount,
        new_trapdoor,
        new_nullifier,
    );
    console::log_1(&"[CORE_DEBUG] NEW NOTE COMPUTED".into());

    let circuit = WithdrawRelationWithFullInput::new(
        MERKLE_PATH_MAX_LEN,
        withdraw_data.fee,
        withdraw_data.recipient,
        deposit_data.token_id,
        deposit_data.nullifier,
        new_note,
        withdraw_data.withdraw_amount,
        withdraw_data.merkle_root,
        deposit_data.trapdoor,
        new_trapdoor,
        new_nullifier,
        withdraw_data.merkle_path,
        deposit_data.leaf_idx.into(),
        deposit_data.note,
        deposit_data.token_amount,
        new_token_amount,
    );

    let pk = <<Groth16 as ProvingSystem>::ProvingKey>::deserialize(&*pk_bytes).unwrap();
    console::log_1(&"[CORE_DEBUG] PK SERIALIZED".into());
    let proof = hex::encode(serialize(&Groth16::prove(&pk, circuit)));
    console::log_1(&"[CORE_DEBUG] PROOF COMPUTED".into());
    //update deposit and return it
    deposit_data.proof = proof;
    deposit_data.trapdoor = new_trapdoor;
    deposit_data.nullifier = new_nullifier;
    deposit_data.token_amount = new_token_amount;
    deposit_data.note = new_note;
    console::log_1(&"[CORE_DEBUG] DEPOSIT DATA UPDATED".into());

    return serde_json::to_string(&deposit_data).unwrap_or(String::from("ERROR_WASM"));
}
