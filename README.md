# PIXORA

**A free, open-source creative suite built with Rust.**

PIXORA aims to bring image editing, vector design, video editing, motion graphics, page layout, media management, and PDF tools into one cohesive creative environment for desktop and the web.

> **Project status:** Early foundation. PIXORA is under active initial development; planned capabilities are not yet claims of implemented functionality.

## Project identity

- **Project:** PIXORA
- **Creator credit:** JacobProjects
- **Primary language:** Rust
- **Targets:** Desktop and browser, with platform-specific integration where needed
- **License:** Apache License 2.0

## Goals

- Build a modular, reusable Rust core.
- Share formats and business logic across supported platforms where practical.
- Keep user projects and media handling predictable and robust.
- Prioritize accessibility, performance, clear UX, and data portability.
- Maintain transparent development with tests, documentation, and automated checks.

## Planned pricing and free versions

PIXORA is intended to offer **free versions alongside a future paid offering**. The current pricing target for the paid version is around **£20**, but this is an early plan, not a final price or release announcement. The exact feature split, whether the paid version is a one-time purchase or another model, and the launch timing have not been decided yet. **PIXORA Vector and PIXORA Layout may require a purchase in the future**; this is a possibility, not a final decision, and their availability in free versions has not been confirmed.

The project is still in early development, and no paid release is available at this stage. Details will be updated here as the plan becomes clearer.

## Planned creative modules

These are roadmap areas, not promises that each editor is already available.

| Module | Purpose |
| --- | --- |
| PIXORA Photo | Raster image editing |
| PIXORA Vector | Vector illustration and design |
| PIXORA Motion | Motion graphics and animation |
| PIXORA Video | Video editing and export |
| PIXORA Layout | Page and publication layout |
| PIXORA Media | Asset and photo management |
| PIXORA PDF | PDF viewing and document tools |

## Repository layout

The project will use a Cargo workspace. Shared engine crates will be kept separate from user-facing applications so they can be tested and reused independently.

- `crates/` — reusable Rust libraries and engines
- `apps/` — desktop and web application shells
- `docs/` — architecture, development, and product documentation
- `tests/` — integration and end-to-end test assets
- `.github/` — CI and contribution templates

The layout will grow as modules are implemented; avoid adding placeholder subsystems without a concrete purpose.

## Getting started

The first runnable desktop application, **PIXORA Photo**, is now in the workspace as an early raster editor. It can open common image formats, adjust brightness, apply grayscale/invert, rotate/flip, reset, and export images. See [PIXORA Photo documentation](docs/PHOTO_EDITOR.md).

Planned development tools include:

- A stable Rust toolchain managed with `rust-toolchain.toml`
- Cargo for builds, tests, and dependency management
- `rustfmt` for formatting
- Clippy for linting

## Development principles

1. **Working software over feature checklists.** Features must be implemented and tested before being described as available.
2. **Small, reviewable changes.** Prefer focused commits and clear pull requests.
3. **Document decisions.** Record significant architectural choices and trade-offs.
4. **Respect licenses.** Track third-party dependencies, fonts, icons, codecs, and other assets with their own notices.
5. **Protect user work.** Treat project files and media as valuable user data; design for safe saves and clear failure handling.

## Documentation

- [PIXORA Photo editor](docs/PHOTO_EDITOR.md) — first runnable raster-editor milestone\n- [Windows installer build](installer/PIXORA.iss) — Inno Setup installer definition\n- [Key service setup](docs/KEY_SERVICE_SETUP.md) — backend deployment steps; keys are not live until configured\n- [Architecture](docs/ARCHITECTURE.md)
- [Access keys and activation](docs/ACCESS_KEYS.md) — one-time keys with 10-day access
- [Key portal frontend](key-portal/index.html) — static GitHub Pages site (backend configuration required)
- [Key portal deployment](docs/KEY_PORTAL_DEPLOYMENT.md) — hosting and secure API contract
- [Roadmap](docs/ROADMAP.md)
- [Building from source](docs/BUILDING.md)
- [Contributing](CONTRIBUTING.md)
- [Security policy](SECURITY.md)
- [Changelog](CHANGELOG.md)

## Contributing

Contributions, bug reports, and thoughtful feature proposals are welcome. Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request. Please avoid posting private data or security-sensitive details in public issues.

## Credits

PIXORA is a project credited to **JacobProjects**.

Third-party projects and assets remain the property of their respective authors and are governed by their own licenses. See [THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md) as dependencies are added.

## License

PIXORA is distributed under the terms of the Apache License, Version 2.0. See [LICENSE](LICENSE).
