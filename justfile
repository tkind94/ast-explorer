set shell := ["zsh", "-uc"]

PROJECT_NAME := 'ast-explorer'

wasm-build:
    cargo build --target wasm32-unknown-unknown --release

wasm-bindgen: wasm-build
    wasm-bindgen --out-dir ./out --target web target/wasm32-unknown-unknown/release/{{PROJECT_NAME}}.wasm

wasm-launch: wasm-bindgen
    simple-http-server ./out --port 2024

wasm-watch:
    cargo watch -c -s 'just wasm-launch' -i out -i target