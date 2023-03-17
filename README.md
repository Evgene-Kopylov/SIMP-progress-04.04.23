# SIMP


### cargo watch + tests
Отслеживать изменения и при сохранении запускать тесты.
```console
cargo install cargo-watch
cd app
cargo watch -x run
```

Сделать WASM
```console
cargo build --target wasm32-unknown-unknown; cp target/wasm32-unknown-unknown/debug/*.wasm .wasm
```