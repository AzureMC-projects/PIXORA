//! PIXORA PDF: a lightweight desktop PDF viewer.
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
#![allow(clippy::all, clippy::pedantic)]

use eframe::egui;
use pdfium_render::prelude::*;
use std::path::{Path, PathBuf};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1180.0, 800.0])
            .with_min_inner_size([760.0, 520.0]),
        ..Default::default()
    };
    eframe::run_native(
        "PIXORA PDF",
        options,
        Box::new(|_cc| Ok(Box::new(PdfApp::default()))),
    )
}

struct PdfApp {
    path: Option<PathBuf>,
    page_index: u16,
    page_count: u16,
    zoom: f32,
    texture: Option<egui::TextureHandle>,
    status: String,
}

impl Default for PdfApp {
    fn default() -> Self {
        Self {
            path: None,
            page_index: 0,
            page_count: 0,
            zoom: 1.0,
            texture: None,
            status: "Open a PDF to get started. Files are opened locally.".into(),
        }
    }
}

impl PdfApp {
    fn open_pdf(&mut self, path: PathBuf, ctx: &egui::Context) {
        match Self::render_pdf_page(&path, 0, self.zoom) {
            Ok((image, count)) => {
                self.path = Some(path.clone());
                self.page_index = 0;
                self.page_count = count;
                self.set_texture(image, ctx);
                self.status = format!("Opened {}", path.display());
            }
            Err(error) => self.status = format!("Could not open PDF: {error}"),
        }
    }

    fn render_pdf_page(path: &Path, index: u16, zoom: f32) -> Result<(egui::ColorImage, u16), String> {
        let library_path = Pdfium::pdfium_platform_library_name_at_path(".");
        let bindings = Pdfium::bind_to_library(library_path)
            .or_else(|_| Pdfium::bind_to_system_library())
            .map_err(|e| format!("PDF rendering engine unavailable: {e}"))?;
        let pdfium = Pdfium::new(bindings);
        let document = pdfium.load_pdf_from_file(path, None)
            .map_err(|e| format!("Invalid or password-protected PDF: {e}"))?;
        let count = document.pages().len();
        if count == 0 { return Err("This PDF has no pages.".into()); }
        let page_index = index.min(count - 1);
        let page = document.pages().get(page_index)
            .map_err(|e| format!("Could not read page: {e}"))?;
        let width = ((page.width().value as f32 * zoom).round() as i32).clamp(300, 2200) as u16;
        let bitmap = page.render_with_config(
            &PdfRenderConfig::new().set_target_width(width as i32)
        ).map_err(|e| format!("Could not render page: {e}"))?;
        let image = bitmap.as_image().to_rgba8();
        let size = [image.width() as usize, image.height() as usize];
        Ok((egui::ColorImage::from_rgba_unmultiplied(size, image.as_raw()), count as u16))
    }

    fn set_texture(&mut self, image: egui::ColorImage, ctx: &egui::Context) {
        if let Some(texture) = self.texture.as_mut() {
            texture.set(image, egui::TextureOptions::LINEAR);
        } else {
            self.texture = Some(ctx.load_texture("pixora-pdf-page", image, egui::TextureOptions::LINEAR));
        }
    }

    fn render_current(&mut self, ctx: &egui::Context) {
        if let Some(path) = self.path.clone() {
            match Self::render_pdf_page(&path, self.page_index, self.zoom) {
                Ok((image, count)) => {
                    self.page_count = count;
                    self.set_texture(image, ctx);
                    self.status = format!("Page {} of {}", self.page_index + 1, self.page_count);
                }
                Err(error) => self.status = error,
            }
        }
    }
}

impl eframe::App for PdfApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.heading("PIXORA PDF");
                ui.separator();
                if ui.button("Open PDF…").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("PDF documents", &["pdf"])
                        .pick_file() {
                        self.open_pdf(path, ctx);
                    }
                }
                ui.separator();
                if ui.button("◀ Previous").clicked() && self.page_index > 0 {
                    self.page_index -= 1;
                    self.render_current(ctx);
                }
                ui.label(format!("Page {} / {}", if self.page_count == 0 { 0 } else { self.page_index + 1 }, self.page_count));
                if ui.button("Next ▶").clicked() && self.page_index + 1 < self.page_count {
                    self.page_index += 1;
                    self.render_current(ctx);
                }
                ui.separator();
                if ui.button("−").clicked() {
                    self.zoom = (self.zoom - 0.1).max(0.5);
                    self.render_current(ctx);
                }
                ui.label(format!("{:.0}%", self.zoom * 100.0));
                if ui.button("+").clicked() {
                    self.zoom = (self.zoom + 0.1).min(2.5);
                    self.render_current(ctx);
                }
                if ui.button("Fit width").clicked() {
                    self.zoom = 1.0;
                    self.render_current(ctx);
                }
            });
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status);
                if let Some(path) = &self.path {
                    ui.separator();
                    ui.label(path.file_name().unwrap_or_default().to_string_lossy());
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.texture {
                egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
                    let size = texture.size_vec2();
                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        ui.add_space(((ui.available_width() - size.x).max(0.0)) / 2.0);
                        ui.image((texture.id(), size));
                    });
                    ui.add_space(16.0);
                });
            } else {
                ui.vertical_centered(|ui| {
                    ui.add_space(150.0);
                    ui.heading("Your documents, beautifully simple.");
                    ui.label("Open a PDF to read it here.");
                    if ui.button("Choose PDF…").clicked() {
                        if let Some(path) = rfd::FileDialog::new().add_filter("PDF documents", &["pdf"]).pick_file() {
                            self.open_pdf(path, ctx);
                        }
                    }
                });
            }
        });
    }
}
