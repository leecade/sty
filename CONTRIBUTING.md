# Contributing

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
