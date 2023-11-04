# SIMP

### Запуск
Запуск локально через [index.html](index.html).

[Живое демо](https://evgene-kopylov.github.io/) на странице автора.

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