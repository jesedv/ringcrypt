//! wasm-bindgen bridge: runs the RingCrypt engine in the browser.
//!
//! The web landing page loads this bundle and calls:
//!
//! - [`run_self_test`] — proves the NTT / negacyclic-engine is bit-correct
//!   *inside the user's browser* (a live, verifiable self-test).
//! - [`run_ckks_self_test`] — full CKKS scheme self-test (encode/decode,
//!   encrypt/decrypt, homomorphic add/multiply).
//! - [`private_mean`] — the threshold-secret-sharing "average without
//!   revealing anyone's value" demo.

use wasm_bindgen::prelude::*;
use serde::Serialize;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[derive(Serialize)]
struct SelfTestReport {
    passed: usize,
    failed: usize,
    ok: bool,
}

#[wasm_bindgen]
pub fn run_self_test() -> JsValue {
    let (passed, failed) = ringcrypt_ntt::self_test();
    serde_wasm_bindgen::to_value(&SelfTestReport {
        passed,
        failed,
        ok: failed == 0,
    })
    .unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn run_ckks_self_test() -> JsValue {
    let mut scheme = ringcrypt_scheme::CkksScheme::new();
    let result = scheme.self_test();
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn ckks_engine_info() -> JsValue {
    let scheme = ringcrypt_scheme::CkksScheme::new();
    let info = scheme.info();
    serde_wasm_bindgen::to_value(&info).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn private_mean(values: Box<[u64]>, seed: u64) -> JsValue {
    let pm = ringcrypt_ss::private_mean(&values, seed);
    serde_wasm_bindgen::to_value(&pm).unwrap_or(JsValue::NULL)
}

#[derive(Serialize)]
struct EngineInfo {
    q: u64,
    ring: &'static str,
    max_negacyclic_n: usize,
    secret_sharing_prime: u64,
    scheme: &'static str,
    status: &'static str,
}

#[wasm_bindgen]
pub fn engine_info() -> JsValue {
    serde_wasm_bindgen::to_value(&EngineInfo {
        q: ringcrypt_ntt::params::Q,
        ring: "Z_q[x]/(x^N+1)",
        max_negacyclic_n: 2048,
        secret_sharing_prime: ringcrypt_ss::P,
        scheme: "RLWE (CKKS/BFV-style)",
        status: "pre-audit (v0.1)",
    })
    .unwrap_or(JsValue::NULL)
}

