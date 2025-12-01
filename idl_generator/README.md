The util to generate Rust stubs from Anchor IDL files.

# Generate Stubs
1. [Install Solores](https://github.com/igneous-labs/solores/blob/master/README.md#installation): `cargo install solores`
1. Go to the `stubs/` directory.
1. Run `solores ../idl/orca_whirlpool.json` to generate the `whirlpool_interface` crate.

# Use the Generated Stubs
1. Add the generated crates cargo to the root level [Cargo.toml](../Cargo.toml) to make them available for use. Example: add `"idl_generator/stubs/whirlpool_interface"`.
2. Run `cargo build` to make sure there generated stubs compiled and wired successfully.
