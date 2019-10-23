

# Production build

```sh
cargo build --release    # it generates the binary target/release/rt
strip target/release/rt  # it reduces the file size removing all symbols
```

For more info about the current release profile defined in the Cargo.toml:

```toml
[profile.release]
lto = true
codegen-units = 1
opt-level = "z"
```

see [min-sized-rust](https://github.com/johnthagen/min-sized-rust).
