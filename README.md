# RingCrypt — Compute on Encrypted Data Without Decrypting

**Free, open-source, browser-native fully homomorphic encryption by [jesed](https://jesed.dev/).**

[![CI](https://github.com/jesedv/ringcrypt/actions/workflows/ci.yml/badge.svg)](https://github.com/jesedv/ringcrypt/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![unsafe: forbidden](https://img.shields.io/badge/unsafe-forbidden-brightgreen.svg)
[![Live demo](https://img.shields.io/badge/demo-live-5b8cff)](https://ringcrypt.jesed.dev)
[![Release](https://img.shields.io/badge/release-v0.1.0-5b8cff)](https://github.com/jesedv/ringcrypt/releases/tag/v0.1.0)

> ⚠️ **Pre-audit (v0.1).** Verified math, pending third-party audit before v1.0.

---

## What is RingCrypt?

RingCrypt lets you **encrypt data, compute on it, and decrypt only the result** — sums,
products, dot products, averages — all on ciphertext. The host running the
computation never sees your plaintext.

| Capability | Status |
|---|---|
| NTT / INTT core (O(N log N) polynomial multiply) | ✅ |
| CKKS scheme: encode, encrypt, decrypt, add, multiply | ✅ |
| Threshold secret sharing (private mean) | ✅ |
| Browser live demo (WASM) | ✅ |
| GPU NTT compute shaders (wgpu/Vulkan), verified bit-exact on RTX 3060 | ✅ |
| Modulus chain / rescaling / relinearization | 🚧 |

## Quick start

Download the latest binary from the [releases page](https://github.com/jesedv/ringcrypt/releases), or build from source:

```bash
git clone git@github.com:jesedv/ringcrypt.git
cd ringcrypt
cargo run --release                      # full self-test + benchmarks
cargo run --release --bin gpu-bench      # GPU NTT benchmark (wgpu/Vulkan)
cargo test --workspace                   # 601 self-test checks
cargo run --example encrypted_workflow   # full FHE: encrypt → compute → decrypt
cargo run --example encrypted_average    # 5-party encrypted mean
cargo run --example encrypted_dot_product
```

## GPU NTT benchmark (RTX 3060, Vulkan)

| Operation | N | CPU (wasm) | GPU (wgpu) | Speedup |
|---|---|---|---|---|
| NTT forward | 2048 | 1.2 ms | 0.08 ms | 15× |
| NTT inverse | 2048 | 1.3 ms | 0.09 ms | 14× |
| Pointwise mul | 2048 | 0.05 ms | 0.004 ms | 12× |
| Negacyclic mul (2N NTT) | 1024 | 1.8 ms | 0.14 ms | 13× |

> Bit-exact against CPU reference across all operations. Single dispatch, no batching.

## Architecture

RingCrypt lives in the polynomial ring **R = Z_q[x]/(x^N+1)**. Polynomial
multiplication — the dominant cost of every FHE operation — is accelerated by
the **Number-Theoretic Transform** (finite-field FFT), turning O(N²) into
**O(N log N)**.

```
ringcrypt/
├── crates/
│   ├── ringcrypt-ntt/      # NTT/INTT, RLWE negacyclic multiply, Barrett modmul
│   ├── ringcrypt-scheme/   # CKKS: canonical embedding, encrypt/decrypt, homomorphic ops
│   ├── ringcrypt-ss/       # Threshold secret sharing (p = 2^31 − 1)
│   └── ringcrypt-wasm/     # wasm-bindgen bridge
├── examples/               # encrypted workflow, average, dot product
├── web/                    # Svelte static site + live WASM demo
├── scripts/                # build, test, bench
└── docs/                   # Math exposition, publishing guide
```

## CKKS parameters

| Parameter | Value |
|---|---|
| Polynomial degree N | 128 (64 complex slots) |
| Ciphertext modulus Q | 2⁶⁴ − 2³² + 1 (Goldilocks/Solinas) |
| Scale Δ | 2²⁴ (~7 decimal digits) |
| Secret key | Ternary (−1, 0, 1) |
| Noise σ | 3.2 |

## Live demo

The real engine runs **in your browser** — zero servers, zero trust:

1. **NTT self-test** — verifies transforms, convolutions, and modular arithmetic
2. **CKKS self-test** — encode/decode roundtrip, encrypt/decrypt, homomorphic add + multiply
3. **Private mean** — five parties reveal no values, all learn the mean

→ [ringcrypt.jesed.dev](https://ringcrypt.jesed.dev)

## Design

- **No unsafe in core.** `#![forbid(unsafe_code)]` across the NTT and scheme crates
- **Correctness over speed.** Every kernel is cross-checked against a reference
- **Reproducible.** Seeded PRNGs; the self-test runs identically everywhere
- **Browser-native.** Same Rust → native CLI + WASM

## Why "RingCrypt"?

FHE lives in **polynomial rings** — Z_q[x]/(x^N+1). The ring is the cryptosystem.
The name is the math.

## License

[MIT](LICENSE) © 2026 RingCrypt contributors.
