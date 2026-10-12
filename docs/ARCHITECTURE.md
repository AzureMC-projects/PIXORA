# Architecture

**Status:** Initial direction; revise as prototypes establish real constraints.

## Goals

PIXORA should have a modular Rust core and keep platform-specific UI and integration separate from domain logic. Shared code should be reused when it reduces duplication without forcing desktop and browser into an awkward lowest common denominator.

## Proposed layers

1. **Core domain (crates):** project metadata, document models, commands, validation, and shared types.
2. **Editing engines:** focused crates for raster, vector, timeline/video, animation, layout, media cataloguing, and PDF workflows as scoped.
3. **Platform adapters (apps):** desktop and browser integration, input, persistence, accessibility, and platform APIs.
4. **Presentation:** panels, canvas/timeline views, tools, and interaction state.
5. **Interoperability:** import/export adapters and explicit format/version handling.

## Rust and browser support

Rust is the primary language for shared logic. Browser builds may use WebAssembly for suitable components, but not every native capability maps directly to the browser. Filesystem access, codecs, GPU APIs, threading, and memory limits need platform-specific evaluation. Do not promise identical feature parity until prototypes prove it.

## User data safety

- Never overwrite source media without explicit intent.
- Prefer atomic or recoverable saves for project metadata.
- Validate imported files and treat them as untrusted input.
- Keep file-format versions explicit and plan migration paths before formats stabilize.
- Test persistence and import/export boundaries.

## Dependency policy

Every dependency needs a clear purpose, acceptable maintenance status, compatible license, and a note about native/browser implications. Track notices for libraries, fonts, icons, codecs, and bundled assets.

## Current implementation

The initial workspace is planned to contain pixora-core, a small foundational crate. Initial types are scaffolding, not completed editors or file-format support.
