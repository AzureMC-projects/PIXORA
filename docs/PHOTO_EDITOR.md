# PIXORA Photo (early desktop editor)

PIXORA Photo is the first runnable creative application in the workspace. It is an early raster editor, not a Photoshop replacement yet.

## Current features

- Open common PNG, JPEG, WebP, BMP, GIF, and TIFF images.
- Preview the image on a fit-to-window canvas.
- Adjust brightness.
- Apply grayscale or invert colours.
- Rotate by 90 or 180 degrees; flip horizontally or vertically.
- Reset edits to the image as originally opened.
- Export to a chosen image path.

## Run from source

Install the stable Rust toolchain, then run:

```sh
cargo run --release -p pixora-photo
```

Run tests and formatting checks from the repository root:

```sh
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

## Windows installer

The Inno Setup definition is in `installer/PIXORA.iss`. Build the Windows release executable first, install Inno Setup 6, and compile the script. The automated workflow `.github/workflows/windows-release.yml` builds an installer artifact on a manual run or when a version tag such as `v0.1.0` is pushed.

The installer targets `C:\Program Files (x86)\PIXORA` as requested. This directory name does not make the app 32-bit; the current workflow builds an x64 Windows application. The installer is not code-signed yet, so Windows SmartScreen may show a warning.

## Known limitations

This is a first milestone, not a production-ready release. It does not yet have layers, selections, masks, brush tools, non-destructive history, colour-managed workflows, PSD support, or the mature reliability and performance expected of a Photoshop alternative. The access-key service is a separate deployment and is not integrated into this editor build yet.
