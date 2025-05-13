#!/bin/bash

set -e

echo "🔧 Building Bevy app for wasm32..."
cargo build --release --target wasm32-unknown-unknown

echo "🔁 Running wasm-bindgen..."
wasm-bindgen --out-dir ../docs/ --target web ./target/wasm32-unknown-unknown/release/PAGAF_Bevy_Game.wasm

echo "🧼 Cleaning previous assets in docs..."
rm -rf ../docs/assets

echo "📦 Copying assets..."
mkdir -p ../docs/assets
cp -r assets/* ../docs/assets/

echo "✅ Done! You can now push to GitHub:"
echo "   git add docs && git commit -m 'Web deploy' && git push then publish on main branch!"