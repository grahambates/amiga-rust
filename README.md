# Amiga Rust example!

## Setup:
```
rustup install nightly
rustup override set nightly
rustup component add rust-src
cargo install cargo-xbuild
```

Uses toolchain binaries from [vscode-amiga-debug](https://github.com/BartmanAbyss/vscode-amiga-debug/tree/master/bin).
Need to add the platform dir and the `opt/bin` subdir to your path.

## Build/Run:
```
make run
```
