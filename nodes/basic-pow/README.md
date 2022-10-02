# GeoBlockchain Consensus

Aditional steps to build and run the node:
- Go to `.cargo/registry/src/github.com-1ecc6299db9ec823/fs-swap-0.2.5/src/platform/linux.rs`
- Add: `use std::convert::TryInto;`
- Change `libc::RENAME_EXCHANGE` for `libc::RENAME_EXCHANGE.try_into().unwrap()`

Build command:

- `cargo build --release -p basic-pow`

Start command:

- `./target/release/basic-pow`

Start command with logs:

- `RUST_LOG=runtime=debug ./target/release/basic-pow --dev`
