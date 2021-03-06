<div align="center">
  <h1><code>wasmweb sample</code></h1>
  
  ![Rust - WASM](https://github.com/CUBETIQ/wasmweb-sample/workflows/Rust%20-%20WASM/badge.svg)
  [![Build status](https://ci.appveyor.com/api/projects/status/c2ybi4806wh5jsqw/branch/master?svg=true)](https://ci.appveyor.com/project/SomboChea/wasmweb-sample/branch/master)
  [![Build Status](https://travis-ci.org/CUBETIQ/wasmweb-sample.svg?branch=master)](https://travis-ci.org/CUBETIQ/wasmweb-sample)
  
  <strong>A template for kick starting a Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>
</div>
## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/CUBETIQ/wasmweb-sample.git --name my-wasm-project
cd my-wasm-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
