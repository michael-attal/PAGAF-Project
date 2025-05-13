## Web build & preview (WASM)

### Compile for WebAssembly:
"""
cargo build --release --target wasm32-unknown-unknown
"""

If it fails:
"""
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo build --release --target wasm32-unknown-unknown
"""

### Generate JS bindings:
"""
wasm-bindgen --out-dir ./docs/ --target web ./target/wasm32-unknown-unknown/release/PAGAF_Bevy_Game.wasm
"""

Make sure `index.html` is also inside the `docs/` folder.
Same for the folder `assets`

### Local preview:
"""
cd ../docs
python3 -m http.server
"""

### Live preview on GitHub Pages:
- Push changes to `main` branch
- GitHub Pages is configured to serve from `/docs`
- Preview URL: https://michael-attal.github.io/PAGAF-Project/