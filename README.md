# rust-in-action

Rust in Action book exercises

## build

```bash
cargo build --manifest-path ./grep-lite/Cargo.toml
```

## run

### example

```bash
cargo run --manifest-path ./grep-lite/Cargo.toml -- picture
```

### help

```bash
cargo run --manifest-path ./grep-lite/Cargo.toml rustc --help
```

## documentation

### standard documentation

```bash
rustup doc
```

### crate documentation

```bash
cargo doc --open --manifest-path grep-lite/Cargo.toml
```

## ide

### vscode

[rls-vscode](https://github.com/rust-lang/rls-vscode)
