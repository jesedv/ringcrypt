# RingCrypt

**Cross-vendor fully homomorphic encryption on GPU — free, open-source, browser-native.**

[![CI](https://github.com/ringcrypt/ringcrypt/actions/workflows/ci.yml/badge.svg)](https://github.com/ringcrypt/ringcrypt/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.97+-orange.svg)](https://www.rust-lang.org/)
![unsafe: forbidden](https://img.shields.io/badge/unsafe-forbidden-brightgreen.svg)
[![Live demo](https://img.shields.io/badge/demo-live-5b8cff)](https://ringcrypt.jesed.dev/)

**RingCrypt computes on encrypted data without ever decrypting it.**

It ships a verified [NTT / INTT](#how-it-works) core, a **CKKS scheme**
(encode/decode, RLWE encryption, homomorphic add + multiply), and a threshold
[secret-sharing](#private-mean) "private mean" — as a Rust workspace that
compiles to a native CLI, a GPU-ready engine, and a **browser-native WASM
demo**. The wrong answer is not an option: every kernel is cross-checked
against a reference, *including live inside your browser*.

> ⚠️ **Pre-audit (v0.1).** The math is verified, but a released crypto engine
> gets a **third-party audit before v1.0**. Do not store real secrets yet.

---

## Why homomorphic encryption?

- **A hospital consortium** can average statistics across institutions *without any side seeing patient records*.
- **A bank** can run a credit-risk query on encrypted customer data and never learn the data.
- **A cloud** can serve analytics on ciphertext — correctness without trust.

Today FHE is CPU-slow (HElib, SEAL, OpenFHE) and GPU ports are NVIDIA-only.
RingCrypt's moat is the math: a **vendor-agnostic, browser-demoable** engine where
the same kernels run on any GPU and in WebAssembly.

## How it works

Homomorphic encryption lives in the polynomial ring **`R = Z_q[x]/(x^N+1)`**.
The dominant cost of every operation is **polynomial multiplication**, which
the **Number-Theoretic Transform (NTT)** reduces from O(N²) to **O(N log N)**:

| Piece | What it is | Verdict |
|---|---|---|
| [`ringcrypt-ntt`](crates/ringcrypt-ntt) | Iterative in-place NTT/INTT, RLWE negacyclic multiply mod `x^N+1`, two-word Barrett modmul | ✅ shipped & tested |
| [`ringcrypt-scheme`](crates/ringcrypt-scheme) | CKKS: encode/decode (canonical embedding via FFT), RLWE keygen/encrypt/decrypt, homomorphic add + multiply | ✅ shipped & tested |
| [`ringcrypt-ss`](crates/ringcrypt-ss) | Additive threshold secret sharing, private mean (p = 2³¹−1) | ✅ shipped & tested |
| GPU kernels (wgpu: Vulkan/Metal/DX12/WebGPU), 32-bit-lane emulation | — | 🚧 roadmap |
| Modulus chain / rescaling / relinearization | — | 🚧 roadmap |

The CKKS scheme uses the Goldilocks prime `Q = 2⁶⁴ − 2³² + 1` with `N=128`
(64 complex slots) and scale `Δ = 2²⁴`. The two-word Barrett multiplier
`μ = ⌊2⁶⁴/q⌋` keeps products exact even where GPUs lack native 64-bit
multiply — the basis of the GPU port.

## Quick start

**Native CLI (verify on bare metal):**

```bash
cargo run          # NTT + CKKS self-test, throughput micro-benchmark
cargo test         # unit suite: NTT, CKKS, Barrett, secret sharing
scripts/regress-ntt.sh   # identical checks, CI-style
```

**Examples:**

```bash
cargo run --example encrypted_average    # 5-party encrypted mean via CKKS
cargo run --example encrypted_dot_product # encrypted dot product (ML building block)
```

**WASM demo (run live in your browser):**

```bash
scripts/build-wasm.sh
cd web && npm run dev   # open the printed localhost URL
```

## Live demo

The landing page runs the **real engine compiled to WebAssembly in your
browser** — no server, no library, no trust:

1. **NTT engine self-test** — proves `NTT(INTT(a)) == a`, NTT convolution vs
   schoolbook, and Barrett modmul exactness, *on your machine*.
2. **CKKS scheme self-test** — encode/decode roundtrip, RLWE encryption,
   homomorphic addition and multiplication — verified live.
3. **Private mean** — five parties reveal nothing yet all learn the mean, via
   additive secret sharing.

→ [Run it now](https://ringcrypt.jesed.dev/)

## Design principles (`AGENTS.md`)

- **Math is the product.** No LLM in the cryptographic or numerical critical path.
- **Correct first, fast second.** Bit-exact against references; the demo runs the same binary.
- **Reproducible.** Seeded PRNGs and unified self-tests.
- **Audited.** Third-party audit before v1.0.
- **Browser-native.** One Rust binary → native + WASM.

## Repository layout

```
ringcrypt/
├── crates/
│   ├── ringcrypt-ntt/     # NTT/INTT core, negacyclic multiply, Barrett modmul
│   ├── ringcrypt-scheme/  # CKKS: encode/decode, encrypt/decrypt, homomorphic ops
│   ├── ringcrypt-ss/      # threshold secret sharing / private mean
│   └── ringcrypt-wasm/    # wasm-bindgen bridge for the live demo
├── examples/            # encrypted average, encrypted dot product
├── web/                 # Svelte + Vite static landing page (+ WASM demo)
├── scripts/             # build, test, regression, bench
├── docs/math.md         # the math, written out
└── .github/workflows/   # CI, GitHub Pages, Releases
```

## Publish channels

| Channel | What | Where |
|---|---|---|
| Source (MIT) | this repo | [github.com/ringcrypt/ringcrypt](https://github.com/ringcrypt/ringcrypt) |
| Binaries | native CLI for Linux/macOS/Windows + WASM bundle | [GitHub Releases](../../releases) |
| crates.io | `ringcrypt-ntt`, `ringcrypt-scheme`, `ringcrypt-ss` | *pending publish* |
| npm | `@ringcrypt/wasm` (wasm-bindgen) | *pending publish* |
| Site | Svelte static app + live demo | [GitHub Pages](../../pages) |

See [docs/publishing.md](docs/publishing.md) for the full strategy.

## Roadmap

- [x] NTT/INTT core + negacyclic multiply + Barrett modmul, verified live in-browser
- [x] CKKS scheme: encode/decode, RLWE keygen/encrypt/decrypt, homomorphic add + multiply
- [x] Threshold secret sharing / private mean demo
- [ ] Modulus chain (RNS) for rescaling / level management
- [ ] Relinearization key + proper relinearize
- [ ] GPU kernels via wgpu (Vulkan / Metal / DX12 / WebGPU) with identical results
- [ ] Multi-word 32-bit-lane modular arithmetic kernel
- [ ] Third-party security audit

## Contributing

Issues and PRs welcome. Please keep the **no unsafe in core** and **verified
math** rules: any new kernel must ship with a reference cross-check. Run
`cargo test --workspace` before opening a PR.

## License

[MIT](LICENSE) © 2026 ringcrypt contributors. Not yet cryptographically audited.
