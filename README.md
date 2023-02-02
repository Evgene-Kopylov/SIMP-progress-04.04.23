# SIMP


### cargo watch + tests
Отслеживать изменения и при сохранении запускать тесты.
```console
cargo install cargo-watch
cd api
cargo watch -q -c -w src/ -x 'test -- --test-threads=1 --nocapture'
```
