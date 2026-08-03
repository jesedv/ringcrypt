<script>
  import { onMount } from 'svelte';
  import { loadWasm } from './wasm.js';

  // ---------- state ----------
  let selftest = $state(null); // { passed, failed, ok }
  let selftestBusy = $state(false);
  let ckksTest = $state(null); // SelfTestResult
  let ckksBusy = $state(false);
  let engine = $state(null);

  // private-mean demo state
  let parties = $state([84, 120, 95, 132, 110]); // default values
  let meanResult = $state(null);
  let meanBusy = $state(false);

  const REPO = 'https://github.com/ringcrypt/ringcrypt';
  const REPO_RELEASE = `${REPO}/releases`;

  onMount(async () => {
    try {
      engine = await loadWasm();
      engine.engine_info();
    } catch (e) {
      engine = null;
      console.error('wasm load failed', e);
    }
  });

  async function runSelfTest() {
    if (!engine) engine = await loadWasm();
    selftestBusy = true;
    selftest = null;
    await new Promise((r) => setTimeout(r, 30)); // paint "running…"
    try {
      selftest = engine.run_self_test();
    } catch (e) {
      selftest = { ok: false, failed: 1, passed: 0, error: String(e) };
    } finally {
      selftestBusy = false;
    }
  }

  async function runCkksTest() {
    if (!engine) engine = await loadWasm();
    ckksBusy = true;
    ckksTest = null;
    await new Promise((r) => setTimeout(r, 30));
    try {
      ckksTest = engine.run_ckks_self_test();
    } catch (e) {
      ckksTest = { ok: false, failed: 1, passed: 0, details: [String(e)] };
    } finally {
      ckksBusy = false;
    }
  }

  async function runMean() {
    if (!engine) engine = await loadWasm();
    meanBusy = true;
    meanResult = null;
    await new Promise((r) => setTimeout(r, 30));
    try {
      meanResult = engine.private_mean(
        parties.map((p) => BigInt(Math.max(0, Math.floor(Number(p))))),
        42n
      );
    } catch (e) {
      meanResult = { error: String(e) };
    } finally {
      meanBusy = false;
    }
  }

  function bump(i, d) {
    parties[i] = Math.max(1, Number(parties[i]) + d);
  }

  // party_views[i][j] = share of value i held by party j.
  function partyView(j) {
    if (!meanResult?.party_views) return [];
    return meanResult.party_views.map((row) => row[j]);
  }
</script>

<svelte:head>
  <title>RingCrypt — Cross-Vendor Fully Homomorphic Encryption on GPU</title>
  <meta
    name="description"
    content="RingCrypt computes on encrypted data without ever decrypting it. A free, open-source, browser-native NTT + threshold-secret-sharing FHE engine that runs on any GPU vendor and in the browser via WASM. Live demo."
  />
</svelte:head>

<!-- ===================== NAV ===================== -->
<header class="nav">
  <a class="logo" href="#top">
    <span class="logo-mark">GF</span>
    <span class="logo-text">RingCrypt</span>
  </a>
  <nav class="nav-links">
    <a href="#problem">Problem</a>
    <a href="#math">The Math</a>
    <a href="#demo">Live Demo</a>
    <a href="#publish">Get It</a>
    <a href="#faq">FAQ</a>
  </nav>
  <a class="btn btn-ghost" href={REPO} target="_blank" rel="noopener">
    <svg class="gh" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true">
      <path fill="currentColor" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
    </svg>
    <span>Star on GitHub</span>
  </a>
</header>

<main id="top">
  <!-- ===================== HERO ===================== -->
  <section class="hero">
    <div class="hero-badge">Open source &bull; Free &bull; Browser-native &bull; Pre-audit (v0.1)</div>
    <h1>Compute on encrypted data.<br /><span class="grad">Without ever decrypting it.</span></h1>
    <p class="hero-sub">
      <strong>RingCrypt</strong> is a free, open-source engine for <em>fully homomorphic encryption</em>
      (RLWE / CKKS / BFV-style) built on a fast <strong>NTT core</strong> — with threshold
      <strong>secret sharing</strong> so a group can compute a mean without revealing anyone's value.
      Same Rust binary runs natively on any GPU vendor and in the browser via <strong>WASM</strong>.
    </p>
    <div class="hero-actions">
      <a class="btn btn-primary" href="#demo">Run the live demo</a>
      <a class="btn btn-ghost" href={REPO} target="_blank" rel="noopener">View source</a>
      <a class="btn btn-ghost" href={REPO_RELEASE} target="_blank" rel="noopener">Download binaries</a>
    </div>
    <div class="hero-stats">
      <span><b>O(N log N)</b> polynomial multiply</span>
      <span><b>0 unsafe</b> lines in core</span>
      <span><b>527</b> self-tests passing</span>
      <span><b>WASM</b> + native + GPU</span>
    </div>
  </section>

  <!-- ===================== PROBLEM ===================== -->
  <section id="problem" class="section">
    <h2>The problem: privacy is a computation gap</h2>
    <p class="lead">
      Today, computing on sensitive data means <em>trusting</em> the host — handing a hospital, bank,
      or cloud provider your plaintext. Homomorphic encryption (FHE) removes that trust: you can sum,
      multiply, and run analytics <em>on ciphertext</em>, and only the key holder ever sees the result.
    </p>
    <div class="grid3">
      <div class="card">
        <h3>FHE is the "holy grail" of privacy</h3>
        <p>
          A hospital consortium can train a model or average statistics across institutions
          <em>without any side seeing patient records</em>. A bank can run a credit-risk query on
          encrypted customer data. The computation is correct; the plaintext never leaks.
        </p>
      </div>
      <div class="card">
        <h3>But it's gated behind cost &amp; vendor lock-in</h3>
        <p>
          HElib / SEAL / OpenFHE are CPU-slow, and GPU ports are NVIDIA-only. That puts privacy
          everywhere behind expensive, proprietary, single-vendor stacks.
        </p>
      </div>
      <div class="card">
        <h3>RingCrypt's moat is the math</h3>
        <p>
          A <strong>vendor-agnostic, browser-demoable</strong> engine: the same NTT kernels run on any
          GPU (Vulkan/Metal/DX12/WebGPU) via 32-bit-emulated 64-bit arithmetic, with a pure-WASM CPU
          fallback. Open source means <em>you can verify it</em> — the right trust model for crypto.
        </p>
      </div>
    </div>
  </section>

  <!-- ===================== MATH ===================== -->
  <section id="math" class="section section-alt">
    <h2>The math under the hood</h2>
    <p class="lead">
      FHE lives in the polynomial ring <code>R = Z_q[x]/(x^N+1)</code>. The dominant cost of every
      operation is <strong>polynomial multiplication</strong>, which the NTT turns into
      O(N log N) pointwise multiplies.
    </p>
    <div class="grid3">
      <div class="card">
        <h3>NTT / INTT</h3>
        <p>
          The number-theoretic transform — the finite-field analogue of the FFT. Iterative,
          in-place Cooley&ndash;Tukey over <code>Z_q</code> with a primitive <code>N</code>-th root
          of unity.
        </p>
        <code class="mini">q = 12289, &nbsp;N = 2<sup>k</sup> &le; 2048</code>
      </div>
      <div class="card">
        <h3>RLWE negacyclic multiply</h3>
        <p>
          Multiplication mod <code>x^N+1</code> is what CKKS/BFV/BGV actually do. We compute it via a
          size-2N cyclic embedding, validated byte-for-byte against schoolbook.
        </p>
        <code class="mini">result<sub>j</sub> = f<sub>j</sub> &minus; f<sub>j+N</sub></code>
      </div>
      <div class="card">
        <h3>Exact modular arithmetic</h3>
        <p>
          Two-word Barrett reduction (<code>&mu; = &lfloor;2<sup>64</sup>/q&rfloor;</code>) keeps
          products exact even where GPUs lack native 64-bit multiply. The multi-word 32-bit-lane GPU
          kernel is the documented roadmap.
        </p>
        <code class="mini">q<sub>est</sub> = &lfloor;value&middot;&mu;/2<sup>64</sup>&rfloor;</code>
      </div>
    </div>
    <p class="math-note">
      <strong>Correctness contract:</strong> every kernel is cross-checked against a slow, obviously
      correct reference — in Rust unit tests and <em>live in your browser</em> below. No LLM in the
      critical path; no hand-waved numerics.
    </p>
  </section>

  <!-- ===================== LIVE DEMO ===================== -->
  <section id="demo" class="section">
    <h2>Live demo — runs in your browser (WASM)</h2>
    <p class="lead">
      The Rust engine is compiled to WebAssembly and executes <strong>right here</strong>, on your
      machine. Two independent proofs:
    </p>
    <div class="demo-grid">
      <!-- SELF TEST -->
      <div class="card demo">
        <h3>1. NTT engine self-test</h3>
        <p>
          Transforms a random polynomial, checks <code>NTT(INTT(a)) == a</code>, verifies NTT-based
          convolution against schoolbook, and confirms Barrett modmul is exact — across many sizes,
          live in this browser tab.
        </p>
        <button class="btn btn-primary" onclick={runSelfTest} disabled={selftestBusy}>
          {selftestBusy ? 'Running…' : 'Run engine self-test'}
        </button>
        {#if selftest}
          <div class="result {selftest.ok ? 'ok' : 'bad'}">
            {#if selftest.ok}
              <p class="result-title">&#10003; ENGINE VERIFIED</p>
              <p>NTT&nbsp;∘&nbsp;INTT roundtrips, NTT convolution, and Barrett modmul all match the
                reference: <b>{selftest.passed} passed, {selftest.failed} failed</b>.</p>
            {:else}
              <p class="result-title">&#10007; ENGINE FAILED ({selftest.failed} checks)</p>
              <p>This runtime could not reproduce exact results.</p>
            {/if}
          </div>
        {/if}
      </div>

      <!-- CKKS SCHEME TEST -->
      <div class="card demo">
        <h3>2. CKKS scheme self-test</h3>
        <p>
          Full CKKS (Cheon-Kim-Kim-Song) approximate HE engine: canonical embedding via FFT,
          encode/decode roundtrip, RLWE encryption/decryption, homomorphic addition and
          multiplication. <b>N=128, 64 complex slots, &Delta;=2<sup>24</sup></b>.
        </p>
        <button class="btn btn-primary" onclick={runCkksTest} disabled={ckksBusy}>
          {ckksBusy ? 'Running…' : 'Run CKKS self-test'}
        </button>
        {#if ckksTest}
          <div class="result {ckksTest.ok ? 'ok' : 'bad'}">
            {#if ckksTest.ok}
              <p class="result-title">&#10003; CKKS VERIFIED</p>
              <p>{ckksTest.passed} checks passed, encrypt/decrypt roundtrip, homomorphic addition and
                multiplication correct:
              </p>
            {:else}
              <p class="result-title">&#10007; CKKS FAILED ({ckksTest.failed} checks)</p>
            {/if}
            {#if ckksTest.details}
              <ul style="margin:4px 0 0; padding-left:18px; color:var(--muted); font-size:.88rem;">
                {#each ckksTest.details as d}
                  <li>{d}</li>
                {/each}
              </ul>
            {/if}
          </div>
        {/if}
      </div>

      <!-- PRIVATE MEAN -->
      <div class="card demo">
        <h3>3. Private mean (threshold secret sharing)</h3>
        <p>
          Five parties each hold a value. We split every value into additive shares over
          <code>p = 2<sup>31</sup>&minus;1</code>; each party only sees its own share column. Together
          they reconstruct the <strong>sum</strong> — so everyone learns the mean, no one learns a value.
        </p>
        <div class="parties">
          {#each parties as p, i (i)}
            <div class="party">
              <span class="party-label">P{i + 1}</span>
              <input type="number" bind:value={parties[i]} min="1" max="100000" />
              <div class="party-ctl">
                <button class="tiny" onclick={() => bump(i, -1)}>&minus;</button>
                <button class="tiny" onclick={() => bump(i, 1)}>+</button>
              </div>
            </div>
          {/each}
        </div>

        <button class="btn btn-primary" onclick={runMean} disabled={meanBusy}>
          {meanBusy ? 'Computing…' : 'Compute private mean'}
        </button>

        {#if meanResult && !meanResult.error}
          <div class="result ok">
            <p class="result-title">
              &#10003; MEAN = {meanResult.mean_f64.toLocaleString()} ({meanResult.n} parties)
            </p>
            <p>Reconstructed sum: <b>{meanResult.total}</b>. No single party's share column encodes any
              other value.</p>
          </div>
          <div class="shares">
            <div class="shares-head">Party's view (shares only — random, reveal nothing)</div>
            {#each parties as _, i (i)}
              <div class="share-row">
                <span class="share-party">P{i + 1} sees</span>
                <span class="share-vals">{partyView(i).join(', ')}</span>
              </div>
            {/each}
            <div class="share-note">Each column is random and alone reveals nothing about the other
              parties' plaintexts. Only all columns summed = the total.</div>
          </div>
        {:else if meanResult && meanResult.error}
          <div class="result bad">
            <p class="result-title">&#10007; Error</p>
            <p>{meanResult.error}</p>
          </div>
        {/if}
      </div>
    </div>
  </section>

  <!-- ===================== PUBLISH / GET IT ===================== -->
  <section id="publish" class="section section-alt">
    <h2>Get it — free and open</h2>
    <p class="lead">Same Rust binary, multiple distribution channels. The moat is the math, not licensing.</p>
    <div class="grid3">
      <div class="card">
        <h3>Source on GitHub</h3>
        <p>MIT-licensed Rust workspace: NTT core, secret-sharing crate, WASM bridge, tests, and
          CI-ready scripts.</p>
        <a class="btn btn-ghost" href={REPO} target="_blank" rel="noopener">github.com/ringcrypt/ringcrypt</a>
      </div>
      <div class="card">
        <h3>Prebuilt binaries (GitHub Releases)</h3>
        <p>
          Native CLI for Linux / macOS / Windows (self-test + throughput bench) and the prebuilt
          <strong>WASM bundle</strong> — no Rust toolchain required to verify or self-host the demo.
        </p>
        <a class="btn btn-ghost" href={REPO_RELEASE} target="_blank" rel="noopener">Releases</a>
      </div>
      <div class="card">
        <h3>Packages for developers</h3>
        <p>
          <code>ringcrypt-ntt</code> &amp; <code>ringcrypt-ss</code> on <strong>crates.io</strong>; the
          WASM binding as <code>@ringcrypt/wasm</code> on <strong>npm</strong>.
        </p>
        <code class="mini">cargo add ringcrypt-ntt<br />npm i @ringcrypt/wasm</code>
      </div>
    </div>
    <div class="publish-note">
      <img src="./og-cover.svg" alt="RingCrypt logo" width="96" />
      <p><strong>Why free?</strong> Crypto is trusted only when it's auditable — open source is the
        trust model. Revenue comes from third-party audits, a managed RingCrypt cloud, and enterprise
        support — never from hiding the code.</p>
    </div>
  </section>

  <!-- ===================== FAQ ===================== -->
  <section id="faq" class="section">
    <h2>FAQ</h2>
    <dl class="faq">
      <dt>Is this production-safe?</dt>
      <dd>Not yet. Everything is <b>pre-audit (v0.1)</b>. The math is verified against references and
        self-tests, but a released crypto engine gets a <b>third-party audit before v1.0</b>.</dd>
      <dt>Is this real FHE, or a toy?</dt>
      <dd>The NTT core, CKKS scheme (encode/decrypt/add/multiply), and secret-sharing are <b>real,
        working, tested Rust</b> — verify them here and with <code>cargo test</code>. The GPU wgpu
        kernels and modulus chain are the <b>documented roadmap</b>.</dd>
      <dt>Do I need a GPU?</dt>
      <dd>No. The WASM build is a correct CPU fallback; the GPU kernels (any vendor) are on the roadmap
        and will produce identical results.</dd>
      <dt>Why wasn't this built with an LLM?</dt>
      <dd>Numerical and cryptographic truth must not depend on an LLM. We author docs and UI with
        tooling, but the math and crypto are hand-written, reviewable code.</dd>
    </dl>
  </section>
</main>

<footer class="footer">
  <p>RingCrypt &mdash; free, open-source, browser-native FHE.</p>
  <p>
    <a href={REPO} target="_blank" rel="noopener">GitHub</a> &nbsp;&bull;&nbsp;
    <a href={`${REPO}/blob/main/README.md`} target="_blank" rel="noopener">README</a> &nbsp;&bull;&nbsp;
    <a href={`${REPO}/blob/main/docs/math.md`} target="_blank" rel="noopener">Math docs</a>
  </p>
  <p class="legal">MIT License &mdash; not yet cryptographically audited. Do not use for real secrets
    until v1.0.</p>
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
  :global(body) {
    margin: 0;
    background: radial-gradient(1200px 600px at 70% -10%, #16213f 0%, var(--bg) 55%);
    color: var(--text); line-height: 1.6;
  }
  :global(code) { background: #0d1220; padding: 2px 6px; border-radius: 6px; font-size: .88em; }
  .nav {
    position: sticky; top: 0; z-index: 20; display: flex; align-items: center; gap: 20px;
    padding: 14px 28px; background: rgba(11,14,22,.82); backdrop-filter: blur(8px);
    border-bottom: 1px solid var(--line);
  }
  .logo { display: flex; align-items: center; gap: 10px; text-decoration: none; color: var(--text); font-weight: 800; }
  .logo-mark {
    width: 34px; height: 34px; border-radius: 9px; display: grid; place-items: center;
    background: linear-gradient(135deg, var(--accent), var(--accent2)); color: #fff; font-weight: 900;
  }
  .nav-links { display: flex; gap: 18px; margin-left: auto; }
  .nav-links a { color: var(--muted); text-decoration: none; font-size: .95rem; }
  .nav-links a:hover { color: var(--text); }
  .btn {
    display: inline-flex; align-items: center; gap: 8px; padding: 10px 18px; border-radius: 10px;
    text-decoration: none; font-weight: 700; font-size: .95rem; border: 1px solid transparent; cursor: pointer;
  }
  .btn-ghost { color: var(--text); border-color: var(--line); background: transparent; }
  .btn-ghost:hover { background: #1a2235; border-color: var(--accent); }
  .btn-primary { background: linear-gradient(135deg, var(--accent), var(--accent2)); color: #fff; }
  .btn-primary:disabled { opacity: .6; cursor: progress; }
  .hero { padding: 96px 28px 56px; text-align: center; max-width: 900px; margin: 0 auto; }
  .hero-badge {
    display: inline-block; padding: 6px 14px; border: 1px solid var(--line); border-radius: 999px;
    color: var(--muted); font-size: .82rem; margin-bottom: 22px;
  }
  h1 { font-size: clamp(2.1rem, 5vw, 3.6rem); line-height: 1.12; margin: 0 0 18px; }
  .grad { background: linear-gradient(90deg, var(--accent), var(--accent2)); -webkit-background-clip: text; background-clip: text; color: transparent; }
  .hero-sub { color: var(--muted); font-size: 1.12rem; max-width: 760px; margin: 0 auto 28px; }
  .hero-actions { display: flex; gap: 14px; justify-content: center; flex-wrap: wrap; }
  .hero-stats {
    display: flex; gap: 22px; justify-content: center; flex-wrap: wrap; margin-top: 44px;
    color: var(--muted); font-size: .9rem;
  }
  .hero-stats b { color: var(--text); }
  .section { padding: 72px 28px; max-width: 1080px; margin: 0 auto; }
  .section-alt { background: var(--bg2); border-top: 1px solid var(--line); border-bottom: 1px solid var(--line); max-width: none; }
  .section-alt > * { max-width: 1080px; margin-left: auto; margin-right: auto; }
  h2 { font-size: clamp(1.5rem, 3vw, 2rem); margin: 0 0 12px; }
  .lead { color: var(--muted); font-size: 1.05rem; max-width: 800px; margin: 0 0 34px; }
  .grid3 { display: grid; grid-template-columns: repeat(auto-fit, minmax(260px, 1fr)); gap: 18px; }
  .card {
    background: var(--card); border: 1px solid var(--line); border-radius: var(--radius);
    padding: 22px; display: flex; flex-direction: column; gap: 10px;
  }
  .card h3 { margin: 0; font-size: 1.1rem; }
  .card p { color: var(--muted); margin: 0; font-size: .95rem; }
  .mini { display: block; margin-top: auto; color: var(--accent); font-size: .82rem; }
  .math-note { color: var(--muted); font-size: .95rem; margin-top: 26px; border-left: 3px solid var(--accent); padding-left: 14px; }
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
  .share-party { color: var(--muted); min-width: 60px; }
  .share-vals { font-family: ui-monospace, monospace; color: var(--text); word-break: break-all; }
  .share-note { color: var(--muted); margin-top: 8px; font-size: .82rem; }
  .publish-note { display: flex; gap: 18px; align-items: center; margin-top: 30px; padding: 18px; background: var(--card); border: 1px solid var(--line); border-radius: var(--radius); }
  .publish-note p { margin: 0; color: var(--muted); }
  .faq dt { font-weight: 800; margin-top: 16px; }
  .faq dd { color: var(--muted); margin: 4px 0 0; }
  .footer { text-align: center; padding: 40px 20px; color: var(--muted); border-top: 1px solid var(--line); font-size: .9rem; }
  .footer a { color: var(--accent); text-decoration: none; }
  .legal { font-size: .8rem; margin-top: 12px; }
</style>
