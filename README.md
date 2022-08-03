# openfga-dsl-parser-wasm
A WASM target of the OpenFGA DSL Parser with examples.

### Building the WASM Target
You must have `rustup` and the `rust` toolchain
1. `git clone` this repo
1. Make any development changes/edits (if desired)
1. Add the wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`
1. Compile to the wasm release:  `cargo build --target wasm32-unknown-unknown --release`
1. Copy the release WASM to the `examples/wasm_binaries` directory