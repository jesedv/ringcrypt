#![forbid(unsafe_code)]

use crate::ciphertext::{Ciphertext, SecretKey};
use crate::params;

pub fn add(ct0: &Ciphertext, ct1: &Ciphertext) -> Ciphertext {
    assert!((ct0.scale - ct1.scale).abs() < 1e-9, "scale mismatch");
    assert_eq!(ct0.n, ct1.n);
    let n = ct0.n;
    let q = params::Q;
    let c0 = params::poly_add(&ct0.c0, &ct1.c0, q);
    let c1 = params::poly_add(&ct0.c1, &ct1.c1, q);
    let c2 = match (&ct0.c2, &ct1.c2) {
        (Some(a), Some(b)) => Some(params::poly_add(a, b, q)),
        (Some(a), None) => Some(a.clone()),
        (None, Some(b)) => Some(b.clone()),
        (None, None) => None,
    };
    Ciphertext {
        c0,
        c1,
        c2,
        scale: ct0.scale,
        n,
    }
}

pub fn sub(ct0: &Ciphertext, ct1: &Ciphertext) -> Ciphertext {
    assert!((ct0.scale - ct1.scale).abs() < 1e-9, "scale mismatch");
    assert_eq!(ct0.n, ct1.n);
    let n = ct0.n;
    let q = params::Q;
    let c0 = params::poly_sub(&ct0.c0, &ct1.c0, q);
    let c1 = params::poly_sub(&ct0.c1, &ct1.c1, q);
    let c2 = match (&ct0.c2, &ct1.c2) {
        (Some(a), Some(b)) => Some(params::poly_sub(a, b, q)),
        (Some(a), None) => Some(a.clone()),
        (None, Some(b)) => {
            let zero = vec![0u64; n];
            Some(params::poly_sub(&zero, b, q))
        }
        (None, None) => None,
    };
    Ciphertext {
        c0,
        c1,
        c2,
        scale: ct0.scale,
        n,
    }
}

pub fn multiply(ct0: &Ciphertext, ct1: &Ciphertext) -> Ciphertext {
    assert_eq!(ct0.n, ct1.n);
    let n = ct0.n;
    let q = params::Q;
    let root2 = params::ntt_twice_root(n as u64);

    let c00 = ringcrypt_ntt::mul_negacyclic_with_root(&ct0.c0, &ct1.c0, n, root2, q);
    let c01 = ringcrypt_ntt::mul_negacyclic_with_root(&ct0.c0, &ct1.c1, n, root2, q);
    let c10 = ringcrypt_ntt::mul_negacyclic_with_root(&ct0.c1, &ct1.c0, n, root2, q);
    let c11 = ringcrypt_ntt::mul_negacyclic_with_root(&ct0.c1, &ct1.c1, n, root2, q);

    Ciphertext {
        c0: c00,
        c1: params::poly_add(&c01, &c10, q),
        c2: Some(c11),
        scale: ct0.scale * ct1.scale,
        n,
    }
}

pub fn multiply_plain(ct: &Ciphertext, plain: &[u64]) -> Ciphertext {
    let n = ct.n;
    let q = params::Q;
    let root2 = params::ntt_twice_root(n as u64);

    let c0 = ringcrypt_ntt::mul_negacyclic_with_root(&ct.c0, plain, n, root2, q);
    let c1 = ringcrypt_ntt::mul_negacyclic_with_root(&ct.c1, plain, n, root2, q);
    let c2 = ct
        .c2
        .as_ref()
        .map(|c| ringcrypt_ntt::mul_negacyclic_with_root(c, plain, n, root2, q));

    Ciphertext {
        c0,
        c1,
        c2,
        scale: ct.scale * params::DELTA as f64,
        n,
    }
}

pub fn decrypt(ct: &Ciphertext, sk: &SecretKey) -> Vec<u64> {
    let n = ct.n;
    let q = params::Q;
    let s_q = params::to_coeffs_q(&sk.s, q);
    let root2 = params::ntt_twice_root(n as u64);

    let cs = ringcrypt_ntt::mul_negacyclic_with_root(&ct.c1, &s_q, n, root2, q);
    let mut msg = params::poly_add(&ct.c0, &cs, q);

    if let Some(ref c2) = ct.c2 {
        let s2 = ringcrypt_ntt::mul_negacyclic_with_root(&s_q, &s_q, n, root2, q);
        let c2s2 = ringcrypt_ntt::mul_negacyclic_with_root(c2, &s2, n, root2, q);
        msg = params::poly_add(&msg, &c2s2, q);
    }

    msg
}

pub fn rescale(ct: &Ciphertext) -> Ciphertext {
    let n = ct.n;
    let q = params::Q;
    let d = params::DELTA;

    let c0 = params::poly_rescale(&ct.c0, d, q);
    let c1 = params::poly_rescale(&ct.c1, d, q);
    let c2 = ct.c2.as_ref().map(|c| params::poly_rescale(c, d, q));

    Ciphertext {
        c0,
        c1,
        c2,
        scale: ct.scale / d as f64,
        n,
    }
}
