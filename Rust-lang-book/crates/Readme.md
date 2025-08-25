# 1. Building in release mode
cargo build --release

# 2. Publishing a crate to crates.io
# Update Cargo.toml with package info and version
cargo login <API_KEY>
cargo publish

# 3. Using a Cargo workspace
# workspace/Cargo.toml
[workspace]
members = ["crate1", "crate2"]

# 4. Installing binaries from crates.io
cargo install ripgrep

# 5. Extending Cargo with custom commands
# Example: cargo-say
cargo install cargo-say
cargo say "Hello, world!"
