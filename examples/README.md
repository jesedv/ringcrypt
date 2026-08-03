# RingCrypt Examples

## encrypted_average.rs

Five parties encrypt their individual values. The encrypted values are summed homomorphically, and the client decrypts the sum — no party learns any other party's plaintext.

```bash
cargo run --release --example encrypted_average
```

Output:
```
Encrypted average over 5 parties
  values:  [120.0, 95.0, 132.0, 88.0, 110.0]
  sum:     545.0000 (exact: 545.0000)
  mean:    109.0000 (exact: 109.0000)
  error:   0.000002
```

---

## encrypted_dot_product.rs

Two vectors are encrypted elementwise. The encrypted product of each pair is computed homomorphically, then summed — a building block for encrypted ML inference.

```bash
cargo run --release --example encrypted_dot_product
```

Output:
```
Encrypted dot product
  a:  [1.0, 2.0, 3.0, 4.0, 5.0]
  b:  [5.0, 4.0, 3.0, 2.0, 1.0]
  dot product (encrypted):  35.0001
  dot product (plain):      35.0000
  error:                    0.000133
```

---

## encrypted_analytics.rs

Hospital consortium scenario: 5 institutions encrypt patient cholesterol data. A central analyst computes the average **on ciphertext** — never sees any plaintext. Only the key holder decrypts the result.

```bash
cargo run --release --example encrypted_analytics
```

See [PRIVATE_CONTENT.md](PRIVATE_CONTENT.md) for the full scenario and architecture diagram.
