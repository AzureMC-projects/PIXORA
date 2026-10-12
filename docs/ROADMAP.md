# Roadmap

This roadmap is directional. Milestones are complete only when code, tests, and documentation are merged.

## Phase 0 — Repository foundation

- [x] Public repository established
- [x] Project overview and creator credit
- [x] Complete Apache-2.0 license text
- [x] Cargo workspace and core crate
- [ ] CI for formatting, linting, and tests
- [x] Contribution and security policies

## Phase 1 — Runnable core

- [ ] Verify workspace on stable Rust
- [ ] Define project metadata and a versioned native project format
- [ ] Implement safe save/load with validation and tests
- [ ] Document supported platforms and build prerequisites

## Phase 2 — First visible application

- [ ] Prototype a cross-platform UI shell
- [ ] Build a minimal project browser and empty-document workflow
- [ ] Add keyboard accessibility and user-facing error reporting
- [ ] Package a developer preview for one desktop platform

## Phase 3 — First editing workflow

Choose one achievable vertical slice before building many empty editors. Evaluate basic raster viewing/transforms or vector canvas editing based on prototype results.

- [ ] Document scope and non-goals
- [ ] Implement an end-to-end import/edit/save/export workflow
- [ ] Add fixtures and regression tests
- [ ] Publish known limitations

## Phase 4 — Expand by modules

Potential modules include Photo, Vector, Video, Motion, Layout, Media, and PDF. Each requires scope, dependency/license review, format strategy, tests, and user documentation.

## Release readiness

Before stable release, establish semantic versioning, release notes, reproducible build instructions, supported-platform policy, accessibility checks, and a backup/migration strategy.
