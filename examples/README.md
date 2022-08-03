# Example Use of `opengfga_dsl_parser.wasm`

## Prerequisites
### Download

### Build
1. `git clone` this repo
1. Make any development changes/edits (if desired)
1. Add the wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`
1. Compile to the wasm release:  `cargo build --target wasm32-unknown-unknown --release`
1. Copy the release WASM to the `examples/wasm_binaries` directory