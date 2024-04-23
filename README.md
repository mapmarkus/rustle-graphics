# Turtle Graphics in Rust

Project to learn Rust through implementing a simple Turtle Graphics engine.

> **NOTE**
> This is toy project that I'm using to learn rust.

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

Other:
- [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)

## Build

```
. scripts/compile-canvas.sh
```

## Running examples

- Run the server with
  ```
  . scripts/serve.sh
  ```
- Open your browser in localhost:8000
- NOTE: Make sure you open DevTools, that prevents the page from being cached between reloads
