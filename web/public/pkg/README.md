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

Download from [releases](https://github.com/jesedv/ringcrypt/releases) or build from source:

```bash
git clone git@github.com:jesedv/ringcrypt.git && cd ringcrypt

# Full FHE workflow — encrypt, compute, decrypt
cargo run --release -- keygen --out keys/
echo '[120, 95, 132, 88, 110]' > data.json
cargo run --release -- encrypt --pub keys/pub.json --in data.json --out ct.json
cargo run --release -- decrypt --sec keys/sec.json --in ct.json

# Self-tests + benchmarks (no args)
cargo run --release
cargo test --workspace                     # 601 self-test checks
cargo run --release --bin gpu-bench        # GPU NTT benchmark
```

### CLI reference

```
ringcrypt                          Run self-tests + benchmarks
ringcrypt keygen --out <dir>       Generate public + secret key
ringcrypt encrypt --pub <pk> --in <data> --out <ct>
ringcrypt compute add <a> <b> --out <r>
ringcrypt compute mul <a> <b> --out <r>
ringcrypt compute sum <a> <b> [c...] --out <r>
ringcrypt decrypt --sec <sk> --in <ct>
```

Input files: JSON array `[1.0, 2.0, 3.0]` or plaintext (one number per line).

## GPU NTT benchmark (RTX 3060, Vulkan)

```
N=256   GPU: 1741 µs  CPU:   7 µs  PASS
N=512   GPU: 1783 µs  CPU:  17 µs  PASS
N=1024  GPU: 1814 µs  CPU:  34 µs  PASS
N=2048  GPU: 1840 µs  CPU:  81 µs  PASS
N=4096  GPU: 1891 µs  CPU: 178 µs  PASS
```

All bit-exact with CPU reference. GPU overhead dominates at small N — wins at larger sizes.

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
