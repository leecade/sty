# Contributing

## Run examples

```bash
cargo run --example simple

# Debug with macro expansion
cargo install cargo-expand
cargo expand --example simple
```

## Run benchmarks

```bash
cargo bench
```

output like:

```
time:   [61.722 ns 61.792 ns 61.865 ns]
```

ops/sec = 1s/61.792ns = 1,000,000,000/61.792 = 16,179,402

## Setup to publish new version

1. Test the code

```bash
cargo test
```

2. Update the version in `Cargo.toml`

3. Push the main branch to trigger the github actions

```bash
git push origin main
```

github actions will automatically publish the new version to crates.io
