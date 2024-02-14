## How to use
```bash
cargo build --target wasm32-unknown-unknown;
wasm-bindgen target/wasm32-unknown-unknown/debug/fingerprint_lookup.wasm --out-dir ./output
```