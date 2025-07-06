# Testing

```bash
cargo test
rm -rf ./target
cargo build --release
rm -f ~/Downloads/scout-seeker
cp target/release/m4b-extractor ~/Downloads
~/Downloads/scout-seeker ~/Downloads
```

## Publish

```bash
cargo publish --dry-run
```

```bash
cargo publish
```
