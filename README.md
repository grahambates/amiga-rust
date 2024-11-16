# Amiga Rust example!

## Setup:

```bash
rustup install nightly
rustup override set nightly
rustup component add rust-src
```

Uses toolchain binaries from [vscode-amiga-debug](https://github.com/BartmanAbyss/vscode-amiga-debug/tree/master/bin).
Need to add the platform dir and the `opt/bin` subdir to your path.

## Build/Run:

```bash
make run
```

## Notes

Target JSON file generated using:

```bash
rustc +nightly -Z unstable-options --print target-spec-json --target m68k-unknown-linux-gnu
```

Then added to using information from https://wiki.osdev.org/Rust_Bare_Bones.
