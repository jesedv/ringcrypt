#![forbid(unsafe_code)]

pub mod ciphertext;
pub mod dft;
pub mod encode;
pub mod encrypt;
pub mod eval;
pub mod key;
pub mod params;

use ciphertext::Ciphertext;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct CkksScheme {
    pub n: usize,
    pub q: u64,
    pub scale: f64,
    pub keys: Option<ciphertext::KeyPair>,
}

#[derive(Serialize, Deserialize)]
pub struct SchemeInfo {
    pub n: usize,
    pub slots: usize,
    pub q: u64,
    pub scale: f64,
    pub delta: u64,
}

#[derive(Serialize, Deserialize)]
pub struct SelfTestResult {
    pub passed: usize,
    pub failed: usize,
    pub ok: bool,
    pub details: Vec<String>,
}

impl CkksScheme {
    pub fn new() -> Self {
        CkksScheme {
            n: params::N,
            q: params::Q,
            scale: params::DELTA as f64,
            keys: None,
        }
    }

    pub fn generate_keys(&mut self, seed: u64) {
        self.keys = Some(key::generate_keys(seed));
    }

    pub fn keys(&self) -> &ciphertext::KeyPair {
        self.keys
            .as_ref()
            .expect("keys must be generated first")
    }

    pub fn encode(&self, message: &[f64]) -> Vec<u64> {
        let n2 = self.n / 2;
        let mut padded = vec![0.0f64; n2];
        let len = message.len().min(n2);
        padded[..len].copy_from_slice(&message[..len]);
        encode::encode_real(&padded, self.scale)
    }

    pub fn encode_pt(&self, plain: &[f64]) -> Vec<u64> {
        let n2 = self.n / 2;
        let mut padded = vec![0.0f64; n2];
        let len = plain.len().min(n2);
        padded[..len].copy_from_slice(&plain[..len]);
        encode::encode_real(&padded, params::DELTA as f64)
    }

    pub fn decode(&self, poly: &[u64]) -> Vec<f64> {
        let decoded = encode::decode_real(poly, self.scale);
        let n2 = self.n / 2;
        decoded[..n2].to_vec()
    }

    pub fn encrypt_from_poly(&self, msg_poly: &[u64], scale: f64, seed: u64) -> Ciphertext {
        encrypt::encrypt(self.keys().pk(), msg_poly, scale, seed)
    }

    pub fn decrypt_to_poly(&self, ct: &Ciphertext) -> Vec<u64> {
        eval::decrypt(ct, self.keys().sk())
    }

    pub fn info(&self) -> SchemeInfo {
        SchemeInfo {
            n: self.n,
            slots: self.n / 2,
            q: self.q,
            scale: self.scale,
            delta: params::DELTA,
        }
    }

    pub fn self_test(&mut self) -> SelfTestResult {
        let mut passed = 0usize;
        let mut failed = 0usize;
        let mut details = Vec::new();

        let seed = 0xBEEF;
        self.generate_keys(seed);

        let n2 = self.n / 2;
        let message: Vec<f64> = (0..n2)
            .map(|i| (i as f64 / n2 as f64 - 0.5) * 10.0)
            .collect();
        let scale = self.scale;

        let pt = self.encode(&message);
        let ct = self.encrypt_from_poly(&pt, scale, seed + 1);
        let decrypted = self.decrypt_to_poly(&ct);
        let decoded = self.decode(&decrypted);

        for (i, (&a, &b)) in message.iter().zip(decoded.iter()).enumerate() {
            let err = (a - b).abs();
            if err < 1e-2 {
                passed += 1;
            } else {
                failed += 1;
                if details.len() < 10 {
                    details.push(format!("slot {i}: expected {a:.4}, got {b:.4}, err={err:.6}"));
                }
            }
        }
        if failed == 0 {
            details.push("encrypt→decrypt roundtrip OK".into());
        }

        let msg_a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let msg_b: Vec<f64> = vec![10.0, 20.0, 30.0, 40.0, 50.0];

        let pt_a = self.encode(&msg_a);
        let pt_b = self.encode(&msg_b);
        let ct_a = self.encrypt_from_poly(&pt_a, scale, seed + 2);
        let ct_b = self.encrypt_from_poly(&pt_b, scale, seed + 3);

        let ct_sum = eval::add(&ct_a, &ct_b);
        let dec_sum = self.decrypt_to_poly(&ct_sum);
        let decoded_sum = self.decode(&dec_sum);
        for i in 0..5 {
            let expected = msg_a[i] + msg_b[i];
            let err = (expected - decoded_sum[i]).abs();
            if err < 2e-2 {
                passed += 1;
            } else {
                failed += 1;
                if details.len() < 20 {
                    details.push(format!(
                        "add slot {i}: expected {expected:.4}, got {:.4}, err={err:.6}",
                        decoded_sum[i]
                    ));
                }
            }
        }
        if failed <= (n2 as usize * 0) {
            details.push("homomorphic addition OK".into());
        }

        let ct_prod = eval::multiply(&ct_a, &ct_b);
        let dec_prod = self.decrypt_to_poly(&ct_prod);
        let scale_sq = scale * scale;
        let decoded_prod = encode::decode_real(&dec_prod, scale_sq);

        for i in 0..5 {
            let expected = msg_a[i] * msg_b[i];
            let err = (expected - decoded_prod[i]).abs();
            if err < 0.5 {
                passed += 1;
            } else {
                failed += 1;
                if details.len() < 30 {
                    details.push(format!(
                        "mul slot {i}: expected {expected:.4}, got {:.4}, err={err:.6}",
                        decoded_prod[i]
                    ));
                }
            }
        }
        if failed <= (n2 as usize * 0) {
            details.push("homomorphic multiplication OK".into());
        }

        SelfTestResult {
            passed,
            failed,
            ok: failed == 0,
            details,
        }
    }
}

impl Default for CkksScheme {
    fn default() -> Self {
        Self::new()
    }
}

impl ciphertext::KeyPair {
    fn pk(&self) -> &ciphertext::PublicKey {
        &self.pk
    }
    fn sk(&self) -> &ciphertext::SecretKey {
        &self.sk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheme_self_test_passes() {
        let mut scheme = CkksScheme::new();
        let result = scheme.self_test();
        assert_eq!(result.failed, 0, "self-test failures: {:?}", result.details);
        assert!(result.passed > 0);
    }

    #[test]
    fn add_then_sub_roundtrip() {
        let mut scheme = CkksScheme::new();
        let msg_a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let msg_b: Vec<f64> = vec![10.0, 20.0, 30.0, 40.0, 50.0];

        scheme.generate_keys(0xCAFE);
        let scale = scheme.scale;

        let pt_a = scheme.encode(&msg_a);
        let pt_b = scheme.encode(&msg_b);
        let ct_a = scheme.encrypt_from_poly(&pt_a, scale, 1);
        let ct_b = scheme.encrypt_from_poly(&pt_b, scale, 2);

        let ct_sum = eval::add(&ct_a, &ct_b);
        let ct_back = eval::sub(&ct_sum, &ct_b);

        let dec = scheme.decrypt_to_poly(&ct_back);
        let decoded = scheme.decode(&dec);

        for i in 0..msg_a.len().min(decoded.len()) {
            let err = (msg_a[i] - decoded[i]).abs();
            assert!(err < 1e-1, "slot {i}: {:.4} vs {:.4}", msg_a[i], decoded[i]);
        }
    }

    #[test]
    fn multiply_identity_check() {
        let mut scheme = CkksScheme::new();
        let msg_a: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let msg_b: Vec<f64> = vec![1.0, 0.0, 0.0, 0.0, 0.0];
        let expected: Vec<f64> = vec![1.0, 0.0, 0.0, 0.0, 0.0];

        scheme.generate_keys(0xCAFE);
        let scale = scheme.scale;

        let pt_a = scheme.encode(&msg_a);
        let pt_b = scheme.encode(&msg_b);
        let ct_a = scheme.encrypt_from_poly(&pt_a, scale, 1);
        let ct_b = scheme.encrypt_from_poly(&pt_b, scale, 2);

        let ct_mult = eval::multiply(&ct_a, &ct_b);
        let dec_mult = scheme.decrypt_to_poly(&ct_mult);
        let decoded = encode::decode_real(&dec_mult, scale * scale);

        for i in 0..5 {
            assert!(
                (decoded[i] - expected[i]).abs() < 1e-3,
                "mult slot {i}: {:.6} vs {:.6}",
                decoded[i], expected[i]
            );
        }
    }
}
