# Bitsong Rust

## TODO
- rust <-> python bindings (via: https://github.com/PyO3/pyo3)
- demo webapp interacting with each command


## Crates

| Name                 | Description                 | crates.io | docs.rs | CI Build |
|----------------------|-----------------------------|-----------|---------|----------|
| [`btisong-rs`]           | Bitsong SDK for Rust         | [![crates.io][bitsong-rs-crate-img]][bitsong-rs-crate-link] | [![docs.rs][bitsong-rs-docs-img]][bitsong-rs-docs-link] | [![CI][bitsong-rs-ci-img]][bitsong-rs-ci-link] |
| [`rs-bitsong‑proto`] | Proto and gRPC definitions  | [![crates.io][rs-bitsong‑proto-crate-img]][rs-bitsong‑proto-crate-link] | ![docs.rs][rs-bitsong‑proto-docs-img] | [![CI][rs-bitsong‑proto-ci-img]][rs-bitsong‑proto-ci-link] |
| [`rs-bitsong‑derive`] | Custom derive traits to generate QueryWrappers  | [![crates.io][rs-bitsong‑derive-crate-img]][rs-bitsong‑derive-crate-link] | ![docs.rs][rs-bitsong‑derive-docs-img] | [![CI][rs-bitsong‑derive-ci-img]][rs-bitsong‑derive-ci-link] |

## Building Proto files from source

```sh
cd proto-build && cargo run 
```
 