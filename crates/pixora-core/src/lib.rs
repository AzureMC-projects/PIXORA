//! Shared foundational types and logic for PIXORA.
//!
//! This crate is intentionally small at the start. Editing engines should be
//! introduced as their requirements become concrete.
//!
//! Copyright 2026 `JacobProjects`. Licensed under Apache-2.0.

#![forbid(unsafe_code)]

/// The current PIXORA product version, not a project-file format version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Broad categories of creative documents planned for PIXORA.
///
/// These identifiers do not imply that editors or file formats are implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DocumentKind {
    /// Raster image document.
    RasterImage,
    /// Vector illustration document.
    VectorGraphic,
    /// Video editing project.
    VideoProject,
    /// Motion graphics or animation project.
    MotionProject,
    /// Page layout or publication document.
    LayoutDocument,
    /// PDF document workflow.
    PdfDocument,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_not_empty() {
        assert_ne!(VERSION, "");
    }

    #[test]
    fn document_kinds_are_distinct() {
        assert_ne!(DocumentKind::RasterImage, DocumentKind::VectorGraphic);
        assert_ne!(DocumentKind::VideoProject, DocumentKind::MotionProject);
    }
}
