//! RingCrypt native CLI — runs engine self-tests, CKKS scheme verification,
//! and NTT throughput benchmarks. Shipped in GitHub Releases.

use std::time::Instant;

fn main() {
    println!("=== RingCrypt {} ===", env!("CARGO_PKG_VERSION"));
    println!("NTT ring: Z_q[x]/(x^N+1), q = {}", ringcrypt_ntt::params::Q);
    println!("secret-sharing prime p = 2^31 - 1 = {}\n", ringcrypt_ss::P);

    let mut ckks = ringcrypt_scheme::CkksScheme::new();
    let info = ckks.info();
    println!(
        "CKKS: N={} slots={} Q={:#x} delta={:#x} scale={:.2e}\n",
        info.n, info.slots, info.q, info.delta, info.scale
    );

    let (passed, failed) = ringcrypt_ntt::self_test();
    println!("NTT self-test: {passed} passed, {failed} failed");
    if failed != 0 {
        eprintln!("FATAL: NTT engine self-test failed");
        std::process::exit(1);
    }

    let ckks_result = ckks.self_test();
    println!(
        "CKKS self-test: {} passed, {} failed",
        ckks_result.passed, ckks_result.failed
    );
    for d in &ckks_result.details {
        println!("  - {d}");
    }
    if !ckks_result.ok {
        eprintln!("FATAL: CKKS scheme self-test failed");
        std::process::exit(1);
    }

    for n in [1024usize, 2048] {
        let q = ringcrypt_ntt::params::Q;
        let a = ringcrypt_ntt::params::rand_poly(n, q);
        let b = ringcrypt_ntt::params::rand_poly(n, q);
        let iters = 2000;
        let t0 = Instant::now();
        for _ in 0..iters {
            let _ = ringcrypt_ntt::negacyclic::mul_negacyclic(&a, &b, n, q);
        }
        let dt = t0.elapsed().as_secs_f64() / iters as f64;
        let ops = 1.0 / dt;
        println!("N={n:5}: {dt:.3} µs/mult  ~ {ops:.0} mults/s  (CPU ref)");
    }

    let pm = ringcrypt_ss::private_mean(&[120, 95, 110, 130, 88], 42);
    println!(
        "\nprivate mean of [120, 95, 110, 130, 88] = {} (exact {}/{})",
        pm.mean_f64, pm.mean_num, pm.mean_den
    );
    println!("\nOK — engine verified on this host.");
}