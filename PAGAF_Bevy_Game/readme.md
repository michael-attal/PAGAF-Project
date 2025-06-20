## Web build & preview (WASM)

### Install wasm-bindgen-cli if not already installed
```
cargo install wasm-bindgen-cli
```

### Compile for WebAssembly:
```
cargo build --release --target wasm32-unknown-unknown
```

If it fails:
```
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo build --release --target wasm32-unknown-unknown
```

### Generate JS bindings:
```
wasm-bindgen --out-dir ../docs/ --target web ./target/wasm32-unknown-unknown/release/PAGAF_Bevy_Game.wasm
```

Make sure `index.html` is also inside the `../docs/` folder.
Same for the folder `assets`.

### Local preview:
```
cd ../docs
python3 -m http.server
```

### Live preview on GitHub Pages:
- Push changes to `main` branch
- GitHub Pages is configured to serve from `/docs`
- Preview URL: https://michael-attal.github.io/PAGAF-Project/


# Rappel pour moi (bevy actuellement est cassé avec MacOS 26 Beta) :

# Build sur macOS 26 Beta avec Apple Container

Dans `~/Developments/containers/bevy-arm64`

## Dockerfile

```dockerfile
# syntax=docker/dockerfile:1
FROM rust:slim

# Installer Git, Python, nano, pkg-config et la cible wasm
RUN apt-get update \
 && apt-get install -y git python3 nano pkg-config \
 && rustup target add wasm32-unknown-unknown

WORKDIR /workspace
CMD ["sh"]
```

## Build de l'image

```bash
container build \
  --arch arm64 \
  --tag bevy-arm64:latest \
  .
```

## Lancer le container (6 Go RAM / 6 CPUs)

```bash
container run \
  --name bevy-dev \
  --tty \
  --interactive \
  --memory 6g \
  --cpus 6 \
  --volume ${HOME}/Developments/ESGI/PAGAF-Project:/workspace/app \
  bevy-arm64:latest
```

## Lancer le container (32 Go RAM / 12 CPUs pour plus de perfs)

```bash
container run \
  --name bevy-dev \
  --tty \
  --interactive \
  --memory 32g \
  --cpus 12 \
  --volume ${HOME}/Developments/ESGI/PAGAF-Project:/workspace/app \
  bevy-arm64:latest
```

## Compilation du projet Bevy pour le WebAssembly

```bash
cd /workspace/app/PAGAF_Bevy_Game
cargo install wasm-bindgen-cli
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo build --release --target wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ../docs/ --target web ./target/wasm32-unknown-unknown/release/PAGAF_Bevy_Game.wasm
```

## Lancer un serveur HTTP pour tester localement

```bash
python3 -m http.server 8000 --directory ./../docs
```

## Lancer ngrok pour https en localhost:

```bash
ngrok http --host-header=rewrite http://192.168.64.2:8000
```