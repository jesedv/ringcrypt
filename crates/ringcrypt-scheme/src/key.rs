#![forbid(unsafe_code)]

use crate::ciphertext::{EvaluationKey, KeyPair, PublicKey, SecretKey};
use crate::params;
use ringcrypt_ntt::params::Rng;

pub fn generate_keys(seed: u64) -> KeyPair {
    let n = params::N;
    let q = params::Q;
    let p = params::P;
    let sigma = params::SIGMA;
    let mut rng = Rng::new(seed);

    let sk = SecretKey {
        s: params::sample_ternary(&mut rng, n),
    };
    let s_q = params::to_coeffs_q(&sk.s, q);

    let a = params::sample_uniform(&mut rng, n, q);
    let e = params::sample_error(&mut rng, n, sigma);
    let e_q = params::to_coeffs_q(&e, q);

    let neg_as = ringcrypt_ntt::mul_negacyclic_with_root(
        &a,
        &s_q,
        n,
        params::ntt_twice_root(n as u64),
        q,
    );
    let p0 = params::poly_sub(&e_q, &neg_as, q);

    let pk = PublicKey {
        p0,
        p1: a,
        n,
        q,
    };

    let a_evk = params::sample_uniform(&mut rng, n, p);
    let e_evk = params::sample_error(&mut rng, n, sigma);
    let e_evk_p = params::to_coeffs_q(&e_evk, p);

    let s_p = params::to_coeffs_q(&sk.s, p);
    let s2 = ringcrypt_ntt::mul_negacyclic_with_root(
        &s_p,
        &s_p,
        n,
        params::p_ntt_twice_root(n as u64),
        p,
    );
    let s2_scaled = params::poly_mul_scalar(&s2, q, p);
    let neg_as_evk = ringcrypt_ntt::mul_negacyclic_with_root(
        &a_evk,
        &s_p,
        n,
        params::p_ntt_twice_root(n as u64),
        p,
    );
    let k0 = params::poly_sub(
        &params::poly_add(&s2_scaled, &e_evk_p, p),
        &neg_as_evk,
        p,
    );

    let ek = EvaluationKey {
        k0,
        k1: a_evk,
    };

    KeyPair { sk, pk, ek }
}
