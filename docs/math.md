# Math notes — RingCrypt

Sources: Fan & Vercauteren, *Somewhat Practical FHE* (2017); Cheon, Kim, Kim,
Song, *CKKS* (Eurocrypt 2017); Brakerski–Gentry–Vaikuntanathan, *FHE without
Bootstrapping* (2012); the RingCrypt literature (cuHE: Dai & Danker; "HEonGPUs").

## 1. The ring

Every RLWE scheme works in the cyclotomic ring

```
R = Z_q[x] / (x^N + 1),   N a power of two
```

Ciphertexts are polynomials with coefficients mod a prime `q`. All that matters
for performance is **polynomial multiplication mod x^N + 1**.

## 2. Parameters

The demo uses the NTT-friendly prime

```
q = 12289,   q − 1 = 12288 = 2^12 · 3
```

`q ≡ 1 (mod N)` for every `N = 2^k ≤ 2048`, so a primitive `N`-th root of unity
exists mod `q` for all supported sizes, and a `2N`-th root exists for the
negacyclic (RLWE) embedding. `g = 11` is a primitive root of the group.

## 3. NTT / INTT

The NTT is the finite-field analogue of the FFT: it maps coefficient
multiplication (O(N²)) to O(N log N) pointwise multiplies. We use an iterative,
in-place **Cooley–Tukey radix-2 DIT** with a bit-reversal permutation and a
primitive `N`-th root `ω = g^{(q−1)/N}`:

```
NTT(a)[k] = Σ_j a[j] · ω^{jk}  (mod q)
```

The inverse uses `ω⁻¹` and scales by `N⁻¹`. Correctness invariant
`NTT(INTT(a)) == a` is tested across sizes and re-proven live in the browser.

## 4. RLWE negacyclic multiply

Multiplication mod `x^N + 1`: because `x^N ≡ −1`, the product of two degree-<N
polynomials reduces by folding the top half in with a sign flip:

```
f  = a · b            (zero-padded to length 2N)
r_j = f_j − f_{j+N}    for j < N
```

This is computed with a **size-2N cyclic NTT** (the embedding trick), so it
reuses the shared NTT core and stays bit-exact against schoolbook.

## 5. Modular arithmetic

Products `a·b` reach ~2⁶⁰, far beyond 32 bits. We use **Barrett reduction**
with a two-word multiplier precomputed once per modulus:

```
μ  = ⌊2^64 / q⌋                 (precomputed)
q_est = ⌊(value · μ) / 2^64⌋     (u128 intermediate)
r  = value − q_est · q           (≤ one or two corrections)
```

A single-word multiplier is only accurate when the product fits in 32 bits —
which it does *not* in FHE — so two-word Barrett is required. The GPU kernel
that maps this onto 32-bit lanes (multi-word emulation) is the roadmap.

## 6. Threshold secret sharing (private mean)

To average `n` values without revealing any:

1. Split each value `v` into `n` additive shares `r_1..r_n` with `Σ r_j ≡ v (mod p)`,
   `p = 2³¹−1` (Mersenne prime — exact in JavaScript numbers).
2. Party `j` holds the `j`-th share of every value. Any `n−1` shares reveal
   nothing about `v` (they are statistically independent of it).
3. Summing **all** shares reconstructs `Σ v`; the mean is
   `(Σ v)/n` as an exact fraction.

This is the honest-MPC building block behind FHE-flavoured analytics and is
used as the browser demo.

## 7. Error / noise budget (roadmap)

Real CKKS, BFV, BGV track a *noise budget*: each multiplication roughly
squares the error, and rescaling/re-linearization keeps it under control. That
layer (encode, rescale, re-linearize, modulus switching) is the next milestone
and reuses the exact same NTT core shipped here.
