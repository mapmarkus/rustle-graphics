# Turtle Graphics in Rust

Project to learn Rust through implementing a simple Turtle Graphics engine.

References:
- [Turtle Graphics](https://en.wikipedia.org/wiki/Turtle_graphics)

Rust docs:
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Carbo Book](https://doc.rust-lang.org/cargo/index.html)
- [WASM pack](https://rustwasm.github.io/docs/wasm-pack/)

Canvas docs:
- [RenderingContext](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D)
- [Path2D](https://developer.mozilla.org/en-US/docs/Web/API/Path2D)

## Build

```
wasm-pack build --target web
```

NOTE: `wasm-pack` is a shortcut for some commands. It would do something like:

```
cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/turtle.wasm --out-dir pkg --web
```

## Running examples

- Run the server with
  ```
  . scripts/serve.sh
  ```
- Open your browser in localhost:8000
- NOTE: Make sure you open DevTools, that prevents the page from being cached between reloads
