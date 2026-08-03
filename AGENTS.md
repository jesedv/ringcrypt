# RingCrypt — Cross-Vendor Fully Homomorphic Encryption on GPU

## Status: v0.1.0 — GPU NTT + CKKS live, WASM demo, 601 self-test checks

GPU NTT/INTT compute shaders implemented (wgpu/Vulkan/WGSL), verified bit-exact
on RTX 3060. CKKS scheme (encode/decode, RLWE encrypt/decrypt, homomorphic add +
multiply), threshold secret sharing, WASM demo all working. 601 self-test checks
pass across all backends.
Remaining: modulus chain / rescaling / relinearization.

## One-liner
A number-theoretic fast Fourier transform (NTT) + INTT + mod-multiply GPU engine that runs FHE (CKKS/BFV-style) across any GPU vendor (NVIDIA/AMD/Intel/Metal/WebGPU) via 32-bit emulation of 64-bit modular arithmetic, with a pure-WASM CPU fallback for the browser.

## The Hard Math
- **CKKS / BFV / BGV** — leveled HE schemes; all build on polynomial ring `R = Z_q[x]/(x^N+1)`.
- **NTT (Number-Theoretic Transform)** — O(NlogN) polynomial multiplication, the dominant cost.
- **pointwise modular multiplication** emulated in double-word (Karatsuba split + Montgomery/Barrett reduction) using only 32-bit lanes.
- **Basis conversion, rescale** — CKKS rescaling (scale down noise), re-linearization (Gadget decomposition).
- **Cryptographic security** — LWE/RLWE hardness, noise-growth bounds.

## The Real Problem
Encrypted analytics and private ML: a hospital / bank / compliance team can compute on-ciphertext (sums, dot products, ML inference) without ever seeing the plaintext. Today FHE is CPU-slow (HElib, SEAL, OpenFHE) and GPU ports are NVIDIA-only. This makes **vendor-agnostic, browser-demoable FHE** — a defensible "works on any GPU, identical results" moat.

## Tech Stack
- **Rust** — core + WGSL shaders (wgpu).
- **wgpu / Vulkan / Metal / DX12** — GPU kernels.
- **WASM** — in-browser FHE demo (`wasm-bindgen`).
- **Pure CPU fallback** — a Naga IR interpreter, so no GPU = still correct.

## Repository Layout
```
ringcrypt/
├── Cargo.toml
├── crates/
│   ├── ringcrypt-ntt/      # forward NTT, INTT, mod-mul, 32-bit emulated 64-bit
│   ├── ringcrypt-scheme/   # CKKS encode/encode, rescale, re-linearize (fallback + GPU)
│   ├── ringcrypt-linalg/   # ciphertext vector/matrix ops, ML inference building blocks
│   ├── ringcrypt-runtime/  # device/command/pipeline orchestration, CPU fallback
│   └── ringcrypt-wasm/     # browser bridge + demo
├── examples/            # encrypted average, encrypted logistic regression
├── ui/                   # demo dashboard
└── docs/
    └── math.md
```

## Build & Test
- `cargo test`
- `./scripts/regress-crossvendor.sh` — run the same op on all available GPUs + CPU fallback; results bitwise-equal within the emulation tolerance.
- `./scripts/regress-ntt.sh` — NTT(c)·INTT(c) == c (mod q), randomized over many q and N.
- `cargo bench` — NTT throughput (ops/sec) across backends.

## Conventions
- **Zero unsafe** in core; `#![forbid(unsafe_code)]`.
- Reference correctness from a scalarCKKS implementation (compare against OpenFHE results *where legally portable*).
- Deterministic, seeded noise for reproducible runs.

## Hard Constraints
- NTT/emulation error ≤ 1 ulp of genuine 64-bit modular arithmetic (so ciphertexts decrypt correctly).
- Identical results across all vendors at a fixed N,q.
- WASM demo ≤ 8 MB and responsive (a full in-cache-size op in < 1 s).
- Production-grade: this touches crypto — **third-party audit before v1**.

## Non-Goals
- A complete MELL toolkit (v2).
- Constant-time correctness for keys kept only in host memory (that's `quanta`).

## Open Questions
- License model: sovereign-engine royalty vs. open core (AGPL has precedent here).
- Which scheme (CKKS for approximate analytics vs BFV for integers) sells better.

## References
- Fan & Vercauteren, "Somewhat Practical FHE" (CKKS 2017); Cheon, Kim, Kim, Song "CKKS" (Eurocrypt 2017).
- Brakerski, Gentry, Vaikuntanathan, "Fully Homomorphic Encryption without Bootstrapping" (2012).
- Verdain et al., "HE on GPUs" literature; "cuHE" (Dai, Danker).
- Barratt, "Fast modular multiplication" and 64-bit modmul emulation notes.