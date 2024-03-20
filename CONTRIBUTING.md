# Contributing

## Setup to publish new version

1. Test the code

```bash
cargo test
```

2. Get the lastest version

```bash
git describe --tags $(git rev-list --tags --max-count=1)
```

3. Create a new version

```bash
git tag -a v0.2.0 -m "Your message"
```

4. Push the new version

```bash
git push origin v0.2.0
```

github actions will automatically publish the new version to crates.io
