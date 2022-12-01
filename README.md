# Advent of Code Sketchbook

## Prep for new day
### Rust
```bash
mkdir yyyy/rust/
cd yyyy/rust/
cargo new --lib day-dd
cd day-dd
mkdir src/bin
touch src/bin/part-1.rs
touch src/bin/part-2.rs
touch input.txt
```
### Setup cargo watch
while in the day directory:
```bash
cargo watch -x "run --bin part-x"
```
