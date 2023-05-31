## Noob game by rust beginners

---

- `cargo run`

or

- `cargo watch -x run` (watch mode!)

---

## Building for web

```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/noob_game.wasm
```
