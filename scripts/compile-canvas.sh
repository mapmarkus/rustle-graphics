#!/bin/sh

# NOTE: `wasm-pack` is a shortcut for some commands. It would do something like:
#
# ```
# cargo build --target wasm32-unknown-unknown
# wasm-bindgen target/wasm32-unknown-unknown/debug/turtle.wasm --out-dir pkg --web
# ```

wasm-pack build --target web
