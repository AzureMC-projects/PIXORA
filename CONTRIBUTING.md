# Contributing to PIXORA

Thanks for considering a contribution. PIXORA is early in development, so clear scope and maintainability matter more than adding many unfinished features.

## Before starting

1. Check existing issues for related work.
2. For large changes, open an issue first to agree on scope.
3. Keep changes focused and avoid unrelated refactors.
4. Do not claim a feature works unless implemented and tested.

## Development checks

    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace

## Pull requests

Include a concise explanation, relevant tests, documentation updates, platform impact, and dependency/license changes. Include screenshots or recordings for UI changes when useful.

## Code expectations

- Prefer clear, idiomatic Rust and explicit error handling.
- Avoid unsafe code unless a documented exception is reviewed.
- Document public APIs and architectural decisions.
- Treat imported files and media as untrusted input.
- Avoid unnecessary dependencies and track third-party notices.

## License and attribution

Contributions are intended to be submitted under the repository's Apache-2.0 license unless a separate arrangement is explicitly agreed. Do not submit code, assets, fonts, or other material you lack permission to contribute.
