Steps to launch the app on web:


Compilation:


```cargo build --release --target wasm32-unknown-unknown```


if compilation doesn't work, use:


RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo build --release --target wasm32-unknown-unknown


Generate javascript files:


```wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/PAGAF_Bevy_Game.wasm```


For testing: start a local server: 

In the index.html folder run:


```python3 -m http.server```