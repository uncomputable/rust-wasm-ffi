# Rust, WASM and C FFI

Deploy a minimal website that exposes C functions via WASM.

Makes use of [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen).

Targets wasm32-unknown-unknown.

## Enter the development environment

Enter the nix-shell. Yes, it includes a web browser.

```
nix-shell
```

## Build the website

Deploy the website locally and visit it in the browser.

```
just serve
just visit
```

Don't forget to kill the web server daemon when you finish.

```
just kill
```

## Run the unit tests

Run the unit tests in a headless web browser.

```
wasm-pack test --headless --firefox
```
