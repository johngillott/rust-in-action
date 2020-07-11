# rust-in-action

Rust in Action book exercises

## build

```bash
cargo build --manifest-path ./grep-lite/Cargo.toml
```

## run

### example

```bash
cargo run --manifest-path ./grep-lite/Cargo.toml -- picture ch2-read-file/readme.md
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

### generating documentation from source

```bash
rustdoc ch3-file-docs/src/main.rs -o ch3-file-docs/doc/
```

## ide

### vscode

[rls-vscode](https://github.com/rust-lang/rls-vscode)

[Visual Studio Code is unable to watch for file changes in this large workspace](https://code.visualstudio.com/docs/setup/linux#_visual-studio-code-is-unable-to-watch-for-file-changes-in-this-large-workspace-error-enospc)

```json
"files.watcherExclude": {
    "**/.git/objects/**": true,
    "**/.git/subtree-cache/**": true,
    "**/node_modules/*/**": true,
    "**/target/**": true // add me
  }
```
