// Lazy-loader for the wasm-bindgen bundle produced by the Rust build.
//
// The bundle is emitted to `public/pkg/` by `scripts/build-wasm.sh` and is a
// plain ES module (`--target web`) that instantiates the `.wasm` at
// `import` time. We dynamic-import it with a runtime-computed URL so:
//   * Rollup leaves it as-is (no bundler resolution error), and
//   * it works whether the site is served at the origin root (dev) or under
//     a GitHub Pages project sub-path (e.g. /ringcrypt/).

let instancePromise = null;

/**
 * @returns {Promise<any>} the wasm module (ringcrypt_wasm.js)
 */
export function loadWasm() {
  if (!instancePromise) {
    // baseURI ends with the directory of the current page.
    let base = document.baseURI;
    if (!base.endsWith('/')) base = new URL('./', base).href;
    const url = new URL('pkg/ringcrypt_wasm.js', base).href;
    instancePromise = import(url).catch((e) => {
      instancePromise = null; // allow retry on transient failure
      throw e;
    });
  }
  return instancePromise;
}
