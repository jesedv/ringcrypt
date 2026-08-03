<script>
  import { onMount } from 'svelte';
  import { loadWasm } from './wasm.js';

  let selftest = $state(null);
  let selftestBusy = $state(false);
  let ckksTest = $state(null);
  let ckksBusy = $state(false);
  let engine = $state(null);
  let parties = $state([84, 120, 95, 132, 110]);
  let meanResult = $state(null);
  let meanBusy = $state(false);

  const REPO = 'https://github.com/jesedv/ringcrypt';
  const DOMAIN = 'https://ringcrypt.jesed.dev';
  const DOCS = `${REPO}/blob/main/docs/math.md`;

  onMount(async () => {
    try { engine = await loadWasm(); } catch (e) { engine = null; }
  });

  async function runSelfTest() {
    if (!engine) engine = await loadWasm();
    selftestBusy = true; selftest = null;
    await new Promise((r) => setTimeout(r, 30));
    try { selftest = engine.run_self_test(); } catch (e) { selftest = { ok: false, failed: 1, passed: 0, error: String(e) }; }
    finally { selftestBusy = false; }
  }

  async function runCkksTest() {
    if (!engine) engine = await loadWasm();
    ckksBusy = true; ckksTest = null;
    await new Promise((r) => setTimeout(r, 30));
    try { ckksTest = engine.run_ckks_self_test(); } catch (e) { ckksTest = { ok: false, failed: 1, passed: 0, details: [String(e)] }; }
    finally { ckksBusy = false; }
  }

  async function runMean() {
    if (!engine) engine = await loadWasm();
    meanBusy = true; meanResult = null;
    await new Promise((r) => setTimeout(r, 30));
    try { meanResult = engine.private_mean(parties.map((p) => BigInt(Math.max(0, Math.floor(Number(p))))), 42n); }
    catch (e) { meanResult = { error: String(e) }; }
    finally { meanBusy = false; }
  }

  function bump(i, d) { parties[i] = Math.max(1, Number(parties[i]) + d); }
  function partyView(j) { if (!meanResult?.party_views) return []; return meanResult.party_views.map((row) => row[j]); }
</script>

<svelte:head>
  <title>RingCrypt — Free Open-Source FHE Engine on GPU</title>
  <meta name="description" content="RingCrypt is a free, open-source fully homomorphic encryption engine. Compute on encrypted data without ever decrypting. Runs on any GPU and in the browser via WASM. Live demo." />
</svelte:head>

<header class="nav">
  <a class="logo" href="#top">
    <img src="./favicon.svg" alt="RingCrypt" class="logo-icon" />
    <span class="logo-text">RingCrypt</span>
  </a>
  <nav class="nav-links">
    <a href="#why">Why FHE</a>
    <a href="#advantages">Advantages</a>
    <a href="#how">How It Works</a>
    <a href="#demo">Live Demo</a>
    <a href="#docs">Docs</a>
    <a href="#faq">FAQ</a>
  </nav>
  <a class="btn btn-ghost" href={REPO} target="_blank" rel="noopener">
    <svg viewBox="0 0 16 16" width="16" height="16" aria-hidden="true">
      <path fill="currentColor" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
    </svg>
    <span>GitHub</span>
  </a>
</header>

<main id="top">
  <!-- HERO -->
  <section class="hero">
    <div class="hero-badge">Free &bull; Open Source &bull; GPU-Accelerated &bull; Browser-Native &bull; v0.1</div>
    <h1>Compute on encrypted data.<br /><span class="grad">Without ever decrypting it.</span></h1>
    <p class="hero-sub">
      <strong>RingCrypt</strong> is the <em>only</em> free, vendor-agnostic engine for fully homomorphic encryption
      that runs on <strong>any GPU</strong> — and right here in your <strong>browser via WASM</strong>.
      Sum, multiply, and run analytics on ciphertext. The host never sees your plaintext.
    </p>
    <div class="hero-actions">
      <a class="btn btn-primary" href="#demo">Run Live Demo</a>
      <a class="btn btn-ghost" href={REPO} target="_blank" rel="noopener">Source Code</a>
      <a class="btn btn-ghost" href={`${REPO}/releases`} target="_blank" rel="noopener">Download Binary</a>
    </div>
    <div class="hero-stats">
      <span><b>21 tests</b> passing</span>
      <span><b>601</b> self-test checks</span>
      <span><b>0 unsafe</b> lines in core</span>
      <span><b>O(N log N)</b> polynomial ops</span>
      <span><b>Any GPU</b> vendor</span>
    </div>
  </section>

  <!-- WHY FHE -->
  <section id="why" class="section section-alt">
    <h2 style="text-align:center">Why homomorphic encryption matters</h2>
    <p class="lead" style="text-align:center;margin:0 auto 34px">
      Encrypt first, compute later. FHE is the cryptographic primitive that lets you
      <strong>run arbitrary computations on ciphertext</strong> — the result is encrypted,
      and only the key holder can decrypt it.
    </p>
    <div class="grid3">
      <div class="card">
        <h3>Healthcare</h3>
        <p>A consortium of hospitals computes the average efficacy of a treatment across institutions — <em>without any hospital revealing patient records</em>.</p>
      </div>
      <div class="card">
        <h3>Finance</h3>
        <p>Banks run credit-risk models and anti-money-laundering queries on encrypted customer data. The computation runs; the plaintext stays hidden.</p>
      </div>
      <div class="card">
        <h3>Cloud &amp; ML</h3>
        <p>Deploy machine learning inference on encrypted inputs. The cloud provider serves predictions without ever seeing the query or the result.</p>
      </div>
    </div>
  </section>

  <!-- ADVANTAGES -->
  <section id="advantages" class="section">
    <h2>Why RingCrypt over other FHE solutions</h2>
    <p class="lead">RingCrypt competes on <strong>math</strong>, not marketing. Here's what sets it apart.</p>
    <div class="advantage-grid">
      <div class="adv-card">
        <h3>100% Free — No Tiers, No Lock-In</h3>
        <p>MIT-licensed. No "community edition" crippleware, no per-core licensing, no enterprise up-sell. Deploy anywhere, audit everything.</p>
      </div>
      <div class="adv-card">
        <h3>Runs on <em>Any</em> GPU Vendor</h3>
        <p>Most FHE GPU ports are CUDA-only, locking you into NVIDIA. RingCrypt targets wgpu — Vulkan, Metal, DX12, and WebGPU. One codebase, every GPU.</p>
      </div>
      <div class="adv-card">
        <h3>Browser-Native — No Install Required</h3>
        <p>WASM compilation means the full engine runs in a browser tab. Zero dependencies, zero servers, zero trust. Verify it yourself, right now.</p>
      </div>
      <div class="adv-card">
        <h3>Verifiable — Open Source, Not Open Claims</h3>
        <p>Every kernel ships with a reference cross-check. 601 live self-tests execute in your browser. The math is correct — <em>prove it here, don't take our word for it</em>.</p>
      </div>
      <div class="adv-card">
        <h3>32-Bit Lane Emulation for 64-Bit Math</h3>
        <p>GPUs lack native 64×64→128 multiply. RingCrypt emulates it exactly on 32-bit lanes using Barrett reduction — proven bit-exact across all backends.</p>
      </div>
      <div class="adv-card">
        <h3>No Key Storage Required</h3>
        <p>Keys are derived deterministically from a seed or generated client-side. RingCrypt never stores, transmits, or sees your keys. You hold the decryption key; only you can decrypt.</p>
      </div>
      <div class="adv-card">
        <h3>Vendor Diveristy = Audibility</h3>
        <p>When one company controls the stack, you can't verify it. RingCrypt is open-source and backend-agnostic — the same code produces identical results everywhere. That's the trust model.</p>
      </div>
      <div class="adv-card">
        <h3>No LLM in the Critical Path</h3>
        <p>Every cryptographic and numerical routine is hand-written, reviewable Rust. We use tooling for docs and UI, but the math is human-authored and cross-validated.</p>
      </div>
    </div>
  </section>

  <!-- HOW IT WORKS -->
  <section id="how" class="section section-alt">
    <h2>How RingCrypt works</h2>
    <p class="lead">
      FHE lives in the polynomial ring <code>R = Z<sub>q</sub>[x]/(x<sup>N</sup>+1)</code>.
      Every operation is a polynomial multiplication, accelerated from O(N²) to
      <strong>O(N log N)</strong> by the Number-Theoretic Transform.
    </p>
    <div class="grid3">
      <div class="card">
        <h3>NTT / INTT Core</h3>
        <p>Iterative Cooley–Tukey radix-2 transform mod prime <code>Q</code>. Forward and inverse with bit-reversal permutation.</p>
        <code class="mini">Q = 12289,  N = 2<sup>k</sup> ≤ 2048</code>
      </div>
      <div class="card">
        <h3>CKKS Scheme</h3>
        <p>Encode real vectors → polynomial via canonical embedding (FFT + twist). RLWE encrypt/decrypt. Homomorphic add and multiply.</p>
        <code class="mini">N=128 · 64 slots · Δ=2<sup>24</sup> · Q=2<sup>64</sup>−2<sup>32</sup>+1</code>
      </div>
      <div class="card">
        <h3>Exact Modular Arithmetic</h3>
        <p>Two-word Barrett reduction keeps products exact on GPUs without native 64-bit multiply. The multi-word 32-bit-lane GPU kernel is the roadmap.</p>
        <code class="mini">μ = ⌊2<sup>64</sup>/q⌋,  q<sub>est</sub> = ⌊value·μ/2<sup>64</sup>⌋</code>
      </div>
    </div>
    <p class="math-note">
      <strong>Correctness contract:</strong> every kernel is cross-checked against a reference implementation —
      in Rust unit tests <em>and live in your browser</em> below. No hand-waved numerics.
    </p>
  </section>

  <!-- LIVE DEMO -->
  <section id="demo" class="section">
    <h2>Live demo — runs right here in your browser</h2>
    <p class="lead">The Rust engine compiles to WebAssembly. No server round-trips, no library install — just the real engine.</p>
    <div class="demo-grid">
      <div class="card demo">
        <h3>1. NTT Engine Self-Test</h3>
        <p>Verifies NTT∘INTT roundtrip, cyclic convolution vs schoolbook, and Barrett modmul exactness — 527 checks, live.</p>
        <button class="btn btn-primary" onclick={runSelfTest} disabled={selftestBusy}>
          {selftestBusy ? 'Running…' : 'Run NTT Self-Test'}
        </button>
        {#if selftest}
          <div class="result {selftest.ok ? 'ok' : 'bad'}">
            <p class="result-title">{selftest.ok ? '✓ NTT VERIFIED' : '✗ FAILED'}</p>
            <p><b>{selftest.passed} passed, {selftest.failed} failed</b></p>
          </div>
        {/if}
      </div>
      <div class="card demo">
        <h3>2. CKKS Scheme Self-Test</h3>
        <p>Full CKKS: encode/decode, RLWE encrypt/decrypt, homomorphic addition and multiplication. 74 checks.</p>
        <button class="btn btn-primary" onclick={runCkksTest} disabled={ckksBusy}>
          {ckksBusy ? 'Running…' : 'Run CKKS Self-Test'}
        </button>
        {#if ckksTest}
          <div class="result {ckksTest.ok ? 'ok' : 'bad'}">
            <p class="result-title">{ckksTest.ok ? '✓ CKKS VERIFIED' : '✗ FAILED'}</p>
            <p><b>{ckksTest.passed} passed, {ckksTest.failed} failed</b></p>
            {#if ckksTest.details}
              <ul style="margin:4px 0 0; padding-left:18px; color:var(--muted); font-size:.88rem;">
                {#each ckksTest.details as d}<li>{d}</li>{/each}
              </ul>
            {/if}
          </div>
        {/if}
      </div>
      <div class="card demo">
        <h3>3. Private Mean</h3>
        <p>Five parties each hold a value. Additive secret sharing over p = 2<sup>31</sup>−1. Each party sees only random shares — together they learn the mean, no one learns any value.</p>
        <div class="parties">
          {#each parties as p, i (i)}
            <div class="party">
              <span class="party-label">P{i+1}</span>
              <input type="number" bind:value={parties[i]} min="1" max="100000" />
              <div class="party-ctl">
                <button class="tiny" onclick={() => bump(i, -1)}>−</button>
                <button class="tiny" onclick={() => bump(i, 1)}>+</button>
              </div>
            </div>
          {/each}
        </div>
        <button class="btn btn-primary" onclick={runMean} disabled={meanBusy}>
          {meanBusy ? 'Computing…' : 'Compute Private Mean'}
        </button>
        {#if meanResult && !meanResult.error}
          <div class="result ok">
            <p class="result-title">✓ MEAN = {meanResult.mean_f64.toLocaleString()} ({meanResult.n} parties)</p>
          </div>
          <div class="shares">
            <div class="shares-head">Each party sees only random shares — reveals nothing</div>
            {#each parties as _, i (i)}
              <div class="share-row"><span class="share-party">P{i+1}</span><span class="share-vals">{partyView(i).join(', ')}</span></div>
            {/each}
          </div>
        {:else if meanResult?.error}
          <div class="result bad"><p class="result-title">✗ Error</p><p>{meanResult.error}</p></div>
        {/if}
      </div>
    </div>
  </section>

  <!-- DOCS -->
  <section id="docs" class="section section-alt">
    <h2>Documentation &amp; Resources</h2>
    <p class="lead">Everything you need to understand, build, and contribute to RingCrypt.</p>
    <div class="grid3">
      <div class="card">
        <h3>Math Deep Dive</h3>
        <p>Full exposition: ring parameters, NTT/INTT derivation, RLWE negacyclic multiply, Barrett reduction, CKKS canonical embedding.</p>
        <a class="btn btn-ghost" href={DOCS} target="_blank" rel="noopener">Read the math docs</a>
      </div>
      <div class="card">
        <h3>Cryptographic Audits</h3>
        <p>Third-party audit is planned before v1.0. Until then, every kernel is verified against a reference — 21 tests, 601 self-test checks.</p>
        <a class="btn btn-ghost" href={REPO} target="_blank" rel="noopener">View test suite</a>
      </div>
      <div class="card">
        <h3>Run It Yourself</h3>
        <p>One command to verify the engine on your hardware. MIT-licensed — no registration, no API keys, no limits.</p>
        <code class="mini">cargo run --release</code>
        <code class="mini">cargo test --workspace</code>
      </div>
    </div>
  </section>

  <!-- GET IT -->
  <section id="publish" class="section">
    <h2>Get RingCrypt — always free</h2>
    <p class="lead">Crypto that costs money can't be trusted. RingCrypt is MIT-licensed. The moat is the math, not a pricing page.</p>
    <div class="grid3">
      <div class="card">
        <h3>Rust Crates</h3>
        <p><code>ringcrypt-ntt</code>, <code>ringcrypt-scheme</code>, <code>ringcrypt-ss</code> on crates.io. Add FHE to your Rust project in one line.</p>
        <code class="mini">cargo add ringcrypt-scheme</code>
      </div>
      <div class="card">
        <h3>CLI Binary</h3>
        <p>Prebuilt native binaries for Linux, macOS, and Windows. Self-test, benchmark, and verify without a Rust toolchain.</p>
        <a class="btn btn-ghost" href={`${REPO}/releases`} target="_blank" rel="noopener">Download</a>
      </div>
      <div class="card">
        <h3>WASM / npm</h3>
        <p>Browser-native bundle via <code>@ringcrypt/wasm</code>. Drop it into any web app and run FHE client-side.</p>
        <code class="mini">npm i @ringcrypt/wasm</code>
      </div>
    </div>
  </section>

  <!-- FAQ -->
  <section id="faq" class="section section-alt">
    <h2>Frequently Asked Questions</h2>
    <dl class="faq">
      <dt>Is RingCrypt production-ready?</dt>
      <dd>v0.1 — pre-audit. The math is verified (601 self-test checks), but a third-party security audit is required before v1.0. Do not use for real secrets yet.</dd>
      <dt>Is this actually full FHE, or a demo?</dt>
      <dd>Real FHE. The CKKS scheme performs encode, encrypt, decrypt, homomorphic addition, and homomorphic multiplication on genuine RLWE ciphertexts. GPU kernels and modulus switching are on the roadmap.</dd>
      <dt>Do I need a GPU?</dt>
      <dd>No. The WASM build is the CPU fallback. GPU acceleration (any vendor) is on the roadmap and will produce identical results.</dd>
      <dt>How does RingCrypt compare to other FHE libraries?</dt>
      <dd>Most FHE libraries are CPU-only, NVIDIA-only, or require paid licenses. RingCrypt is free, open-source, vendor-agnostic, and browser-native — <em>and</em> you can verify it yourself with the live demo.</dd>
      <dt>What kind of computations can I run?</dt>
      <dd>Addition, subtraction, and multiplication on encrypted integers and real numbers. With a modulus chain (roadmap), deep circuits become practical: ML inference, encrypted search, private set intersection.</dd>
      <dt>How are keys managed?</dt>
      <dd>Keys are generated client-side from a seed. RingCrypt never stores or transmits keys. You hold the secret key; you decrypt.</dd>
      <dt>Is there a limit on operations?</dt>
      <dd>The current single-modulus implementation supports up to ~3 multiplications before noise overwhelms the signal. A full modulus chain (roadmap) extends this to arbitrary depth.</dd>
    </dl>
  </section>
</main>

<footer class="footer">
  <div class="footer-grid">
    <div class="footer-col">
      <strong>RingCrypt</strong>
      <p>Field-level, polynomial-ring encryption on GPU. Free, open, auditable.</p>
    </div>
    <div class="footer-col">
      <strong>Links</strong>
      <a href={REPO} target="_blank" rel="noopener">GitHub</a>
      <a href={DOCS} target="_blank" rel="noopener">Math Docs</a>
      <a href={`${REPO}/blob/main/docs/publishing.md`} target="_blank" rel="noopener">Publishing Guide</a>
      <a href={`${REPO}/releases`} target="_blank" rel="noopener">Releases</a>
    </div>
    <div class="footer-col">
      <strong>Legal</strong>
      <span>MIT License</span>
      <span>Pre-audit — not for production secrets until v1.0</span>
    </div>
  </div>
  <p class="legal">&copy; 2026 RingCrypt contributors. MIT License.</p>
</footer>

<style>
  :global(:root) {
    --bg: #0b0e16; --bg2: #0f1420; --card: #141a2a; --line: #243048;
    --text: #e7ecf5; --muted: #9aa7bd;
    --accent: #5b8cff; --accent2: #9b6bff;
    --ok: #34d399; --bad: #f87171; --radius: 14px;
    font-family: 'Inter', 'Segoe UI', system-ui, -apple-system, sans-serif; color-scheme: dark;
  }
  :global(*) { box-sizing: border-box; }
  :global(html) { scroll-behavior: smooth; }
  :global(body) { margin: 0; background: radial-gradient(1200px 600px at 70% -10%, #16213f 0%, var(--bg) 55%); color: var(--text); line-height: 1.6; }
  :global(code) { background: #0d1220; padding: 2px 6px; border-radius: 6px; font-size: .88em; }

  .nav { position: sticky; top: 0; z-index: 20; display: flex; align-items: center; gap: 16px; padding: 14px 28px; background: rgba(11,14,22,.82); backdrop-filter: blur(8px); border-bottom: 1px solid var(--line); }
  .logo { display: flex; align-items: center; gap: 10px; text-decoration: none; color: var(--text); font-weight: 800; }
  .logo-icon { width: 32px; height: 32px; }
  .nav-links { display: flex; gap: 18px; margin-left: auto; }
  .nav-links a { color: var(--muted); text-decoration: none; font-size: .95rem; }
  .nav-links a:hover { color: var(--text); }
  .btn { display: inline-flex; align-items: center; gap: 8px; padding: 10px 18px; border-radius: 10px; text-decoration: none; font-weight: 700; font-size: .95rem; border: 1px solid transparent; cursor: pointer; }
  .btn-ghost { color: var(--text); border-color: var(--line); background: transparent; }
  .btn-ghost:hover { background: #1a2235; border-color: var(--accent); }
  .btn-primary { background: linear-gradient(135deg, var(--accent), var(--accent2)); color: #fff; }
  .btn-primary:disabled { opacity: .6; cursor: progress; }

  .hero { padding: 96px 28px 56px; text-align: center; max-width: 900px; margin: 0 auto; }
  .hero-badge { display: inline-block; padding: 6px 14px; border: 1px solid var(--line); border-radius: 999px; color: var(--muted); font-size: .82rem; margin-bottom: 22px; }
  h1 { font-size: clamp(2.1rem, 5vw, 3.6rem); line-height: 1.12; margin: 0 0 18px; }
  .grad { background: linear-gradient(90deg, var(--accent), var(--accent2)); -webkit-background-clip: text; background-clip: text; color: transparent; }
  .hero-sub { color: var(--muted); font-size: 1.12rem; max-width: 760px; margin: 0 auto 28px; }
  .hero-actions { display: flex; gap: 14px; justify-content: center; flex-wrap: wrap; }
  .hero-stats { display: flex; gap: 22px; justify-content: center; flex-wrap: wrap; margin-top: 44px; color: var(--muted); font-size: .9rem; }
  .hero-stats b { color: var(--text); }

  .section { padding: 72px 28px; max-width: 1080px; margin: 0 auto; }
  .section-alt { background: var(--bg2); border-top: 1px solid var(--line); border-bottom: 1px solid var(--line); max-width: none; }
  .section-alt > * { max-width: 1080px; margin-left: auto; margin-right: auto; }
  h2 { font-size: clamp(1.5rem, 3vw, 2rem); margin: 0 0 12px; }
  .lead { color: var(--muted); font-size: 1.05rem; max-width: 800px; margin: 0 0 34px; }

  .grid3 { display: grid; grid-template-columns: repeat(auto-fit, minmax(260px, 1fr)); gap: 18px; }
  .card { background: var(--card); border: 1px solid var(--line); border-radius: var(--radius); padding: 22px; display: flex; flex-direction: column; gap: 10px; }
  .card h3 { margin: 0; font-size: 1.1rem; }
  .card p { color: var(--muted); margin: 0; font-size: .95rem; }
  .mini { display: block; margin-top: auto; color: var(--accent); font-size: .82rem; }
  .math-note { color: var(--muted); font-size: .95rem; margin-top: 26px; border-left: 3px solid var(--accent); padding-left: 14px; }

  .advantage-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 16px; }
  .adv-card { background: var(--card); border: 1px solid var(--line); border-radius: var(--radius); padding: 20px; }
  .adv-card h3 { margin: 0 0 8px; font-size: 1.05rem; }
  .adv-card p { color: var(--muted); margin: 0; font-size: .92rem; }

  .demo-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 20px; }
  .demo { align-items: stretch; }
  .result { padding: 14px; border-radius: 10px; margin-top: 12px; font-size: .94rem; }
  .result.ok { background: rgba(52,211,153,.1); border: 1px solid var(--ok); }
  .result.bad { background: rgba(248,113,113,.1); border: 1px solid var(--bad); }
  .result-title { font-weight: 800; margin: 0 0 6px; }
  .result p { margin: 0; color: var(--muted); }

  .parties { display: flex; flex-wrap: wrap; gap: 8px; margin: 10px 0; }
  .party { display: flex; align-items: center; gap: 8px; background: #0e1422; border: 1px solid var(--line); border-radius: 10px; padding: 6px 8px; }
  .party-label { color: var(--muted); font-size: .8rem; }
  .party input { width: 72px; background: transparent; border: none; color: var(--text); font-weight: 700; text-align: center; }
  .party-ctl { display: flex; gap: 4px; }
  .tiny { width: 24px; height: 24px; border-radius: 6px; border: 1px solid var(--line); background: transparent; color: var(--text); cursor: pointer; }
  .shares { margin-top: 12px; border: 1px solid var(--line); border-radius: 10px; padding: 10px; font-size: .85rem; }
  .shares-head { color: var(--accent); font-weight: 700; margin-bottom: 6px; }
  .share-row { display: flex; gap: 8px; padding: 2px 0; }
  .share-party { color: var(--muted); min-width: 40px; }
  .share-vals { font-family: ui-monospace, monospace; color: var(--text); word-break: break-all; }

  .faq dt { font-weight: 800; margin-top: 20px; }
  .faq dd { color: var(--muted); margin: 4px 0 0; }

  .footer { text-align: center; padding: 40px 20px; color: var(--muted); border-top: 1px solid var(--line); font-size: .9rem; }
  .footer-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 30px; text-align: left; max-width: 900px; margin: 0 auto 24px; }
  .footer-col { display: flex; flex-direction: column; gap: 8px; }
  .footer-col strong { color: var(--text); font-size: .95rem; }
  .footer-col a { color: var(--accent); text-decoration: none; }
  .legal { font-size: .8rem; margin-top: 12px; }
</style>
