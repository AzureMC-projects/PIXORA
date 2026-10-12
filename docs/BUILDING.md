# Building PIXORA

The repository is at an early foundation stage. These commands cover the Rust workspace; a desktop or browser app is not yet configured.

## Prerequisites

- Git
- Stable Rust toolchain
- Cargo, rustfmt, and Clippy components

Install Rust using the official instructions: https://www.rust-lang.org/tools/install

## Clone

    git clone https://github.com/AzureMC-projects/PIXORA.git
    cd PIXORA

## Check the workspace

    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace

Run these checks before submitting changes. App-specific prerequisites will be documented after the UI framework and packaging strategy have been selected and checked in.
