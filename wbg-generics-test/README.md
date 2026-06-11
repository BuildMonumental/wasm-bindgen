# wbg-generics-test

End-to-end test for the `#[wasm_bindgen(instantiate(...))]` attribute added on
this branch. Exercises monomorphized exports of generic structs/impls
(`Holder<P>`, `Pair<A, B>`) from a Node runtime.

`src/lib.rs` defines the generic types and their instantiations; `test.mjs`
drives the generated bindings and asserts behaviour.

## Build & run

Requires the patched `wasm-bindgen` CLI built from this branch (the stock CLI
does not understand the `instantiate` schema). Build it once:

```bash
cargo build -p wasm-bindgen-cli --release   # from the repo root
```

Then, from this directory:

```bash
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/wbg_generics_test.wasm \
    --out-dir pkg --target nodejs
node test.mjs
```

Expected output: `all extended runtime assertions passed`.
