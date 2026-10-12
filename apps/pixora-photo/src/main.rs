//! PIXORA Photo: a first, deliberately small raster editor.

#![allow(clippy::all, clippy::pedantic)]

use chrono::{DateTime, Utc};
use eframe::egui;
use image::DynamicImage;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1180.0, 780.0])
            .with_min_inner_size([760.0, 520.0]),
        ..Default::default()
    };

    eframe::run_native(
        "PIXORA Photo",
        options,
        Box::new(|_cc| Ok(Box::new(PhotoApp::default()))),
    )
}

struct PhotoApp {
    image: Option<DynamicImage>,
    original: Option<DynamicImage>,
    texture: Option<egui::TextureHandle>,
    open_path: String,
    save_path: String,
    brightness: i32,
    status: String,
    dirty: bool,
    activation_key: String,
    installation_id: String,
    expires_at: Option<String>,
}

impl Default for PhotoApp {
    fn default() -> Self {
        let (installation_id, expires_at) = load_activation_record();
        Self {
            image: None,
            original: None,
            texture: None,
            open_path: String::new(),
            save_path: String::from("pixora-edited.png"),
            brightness: 0,
            status: String::from("Open an image to begin. Your files stay on your device."),
            dirty: false,
            activation_key: String::new(),
            installation_id,
            expires_at,
        }
    }
}

impl PhotoApp {
    fn is_activated(&self) -> bool {
        self.expires_at.as_deref().and_then(|value| DateTime::parse_from_rfc3339(value).ok())
            .is_some_and(|expiry| expiry.with_timezone(&Utc) > Utc::now())
    }

    fn activate_key(&mut self) {
        let key = self.activation_key.trim().to_ascii_uppercase();
        if key.is_empty() {
            self.status = "Enter the access key from the official PIXORA portal.".to_owned();
            return;
        }
        self.status = "Contacting PIXORA activation service…".to_owned();
        let payload = json!({
            "key": key,
            "installationPublicKey": self.installation_id
        });
        let result = reqwest::blocking::Client::new()
            .post("https://vrwumbswbcwpyhbpsiyx.supabase.co/functions/v1/activate-key")
            .header("apikey", "sb_publishable_ea72oQoOcoyhx-LvAIh2Gw_SYp3UCZi")
            .json(&payload)
            .send();

        match result {
            Ok(response) => {
                let status = response.status();
                let body = response.json::<Value>().unwrap_or_else(|_| json!({}));
                if status.is_success() && body.get("activated").and_then(Value::as_bool) == Some(true) {
                    if let Some(expiry) = body.get("expiresAt").and_then(Value::as_str) {
                        self.expires_at = Some(expiry.to_owned());
                        if save_activation_record(&self.installation_id, self.expires_at.as_deref()) {
                            self.activation_key.clear();
                            self.status = format!("PIXORA activated until {expiry}.");
                        } else {
                            self.expires_at = None;
                            self.status = "Activation succeeded, but this device could not save the activation record. Check your local app-data permissions and retry.".to_owned();
                        }
                    } else {
                        self.status = "Activation service returned an incomplete response.".to_owned();
                    }
                } else {
                    self.status = body.get("error").and_then(Value::as_str)
                        .unwrap_or("Activation failed. Please try again.")
                        .to_owned();
                }
            }
            Err(error) => {
                self.status = format!("Could not reach the activation service: {error}");
            }
        }
    }

    fn load_path(&mut self, path: &Path, ctx: &egui::Context) {
        match image::open(path) {
            Ok(image) => {
                self.open_path = path.display().to_string();
                self.save_path = default_output_path(path).display().to_string();
                self.original = Some(image.clone());
                self.image = Some(image);
                self.brightness = 0;
                self.dirty = false;
                self.status = format!("Opened {}", path.display());
                self.refresh_texture(ctx);
            }
            Err(error) => self.status = format!("Could not open image: {error}"),
        }
    }

    fn refresh_texture(&mut self, ctx: &egui::Context) {
        let Some(image) = self.image.as_ref() else {
            self.texture = None;
            return;
        };
        let rgba = image.to_rgba8();
        let size = [rgba.width() as usize, rgba.height() as usize];
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
        if let Some(texture) = self.texture.as_mut() {
            texture.set(color_image, egui::TextureOptions::LINEAR);
        } else {
            self.texture = Some(ctx.load_texture(
                "pixora-photo-canvas",
                color_image,
                egui::TextureOptions::LINEAR,
            ));
        }
    }

    fn apply_filter(
        &mut self,
        ctx: &egui::Context,
        operation: impl FnOnce(DynamicImage) -> DynamicImage,
    ) {
        if let Some(image) = self.image.take() {
            self.image = Some(operation(image));
            self.dirty = true;
            self.refresh_texture(ctx);
        }
    }

    fn save(&mut self) {
        let Some(image) = self.image.as_ref() else {
            self.status = "Open an image before saving.".to_owned();
            return;
        };
        let path = PathBuf::from(self.save_path.trim());
        if path.as_os_str().is_empty() {
            self.status = "Choose a save path first.".to_owned();
            return;
        }
        match image.save(&path) {
            Ok(()) => {
                self.dirty = false;
                self.status = format!("Saved {}", path.display());
            }
            Err(error) => self.status = format!("Could not save image: {error}"),
        }
    }
}

impl eframe::App for PhotoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.is_activated() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(90.0);
                    ui.heading("Welcome to PIXORA");
                    ui.label("Enter your one-time access key to activate this installation.");
                    ui.label("Get a key at https://azuremc-projects.github.io/PIXORA/key-portal/");
                    ui.add_space(16.0);
                    ui.add_sized([360.0, 32.0], egui::TextEdit::singleline(&mut self.activation_key)
                        .hint_text("PX-…").char_limit(64));
                    ui.add_space(8.0);
                    if ui.button("Activate PIXORA").clicked() {
                        self.activate_key();
                    }
                    ui.add_space(8.0);
                    ui.label("Each key can be activated once and provides 10 days from activation.");
                    ui.label(&self.status);
                });
            });
            return;
        }

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.heading("PIXORA Photo");
                ui.separator();
                if ui.button("Open image…").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter(
                            "Image files",
                            &["png", "jpg", "jpeg", "webp", "bmp", "gif", "tif", "tiff"],
                        )
                        .pick_file()
                    {
                        self.load_path(&path, ctx);
                    }
                }
                if ui.button("Save").clicked() {
                    self.save();
                }
                if ui.button("Save as…").clicked() {
                    let dialog = rfd::FileDialog::new()
                        .add_filter("PNG image", &["png"])
                        .add_filter("JPEG image", &["jpg", "jpeg"])
                        .add_filter("WebP image", &["webp"]);
                    if let Some(path) = dialog.save_file() {
                        self.save_path = path.display().to_string();
                        self.save();
                    }
                }
                if self.dirty {
                    ui.label("● Unsaved edits");
                }
            });
        });

        egui::SidePanel::left("tools")
            .resizable(true)
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.heading("Adjustments");
                ui.label("Brightness");
                let previous = self.brightness;
                ui.add(egui::Slider::new(&mut self.brightness, -100..=100).suffix("%"));
                if self.brightness != previous {
                    if let Some(original) = self.original.as_ref() {
                        self.image = Some(original.brighten(self.brightness * 2));
                        self.dirty = self.brightness != 0;
                        self.refresh_texture(ctx);
                    }
                }

                ui.separator();
                ui.label("One-click edits");
                if ui.button("Grayscale").clicked() {
                    self.apply_filter(ctx, |image| image.grayscale());
                }
                if ui.button("Invert colours").clicked() {
                    self.apply_filter(ctx, |mut image| {
                        image.invert();
                        image
                    });
                }
                if ui.button("Rotate 90° clockwise").clicked() {
                    self.apply_filter(ctx, |image| image.rotate90());
                }
                if ui.button("Rotate 180°").clicked() {
                    self.apply_filter(ctx, |image| image.rotate180());
                }
                if ui.button("Flip horizontally").clicked() {
                    self.apply_filter(ctx, |image| image.fliph());
                }
                if ui.button("Flip vertically").clicked() {
                    self.apply_filter(ctx, |image| image.flipv());
                }
                if ui.button("Reset to original").clicked() {
                    self.image = self.original.clone();
                    self.brightness = 0;
                    self.dirty = false;
                    self.refresh_texture(ctx);
                }

                ui.separator();
                ui.heading("Export");
                ui.label("Save path");
                ui.text_edit_singleline(&mut self.save_path);
                if ui.button("Save image").clicked() {
                    self.save();
                }

                if let Some(image) = self.image.as_ref() {
                    ui.separator();
                    ui.label(format!("{} × {} px", image.width(), image.height()));
                    ui.label(format!("Format: {}", image_format_label(&self.open_path)));
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);
            if let Some(texture) = self.texture.as_ref() {
                let available = ui.available_size();
                let texture_size = texture.size_vec2();
                let scale = (available.x / texture_size.x)
                    .min(available.y / texture_size.y)
                    .min(1.0);
                let display_size = texture_size * scale.max(0.01);
                ui.centered_and_justified(|ui| {
                    ui.add(egui::Image::new((texture.id(), display_size)));
                });
            } else {
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Your canvas is empty");
                        ui.label(
                            "Open a PNG, JPEG, WebP, BMP, GIF or TIFF image to start editing.",
                        );
                    });
                });
            }
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(&self.status);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("PIXORA {}", pixora_core::VERSION));
                });
            });
        });
    }
}

fn default_output_path(input: &Path) -> PathBuf {
    let stem = input.file_stem().unwrap_or_default().to_string_lossy();
    input.with_file_name(format!("{stem}-pixora.png"))
}

fn image_format_label(path: &str) -> &'static str {
    match Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("jpg" | "jpeg") => "JPEG",
        Some("webp") => "WebP",
        Some("bmp") => "BMP",
        Some("gif") => "GIF",
        Some("tif" | "tiff") => "TIFF",
        _ => "PNG or unknown",
    }
}


fn activation_record_path() -> Option<PathBuf> {
    dirs::data_local_dir().map(|base| base.join("PIXORA").join("activation.json"))
}

fn load_activation_record() -> (String, Option<String>) {
    if let Some(path) = activation_record_path() {
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(value) = serde_json::from_str::<Value>(&contents) {
                let id = value.get("installation_id").and_then(Value::as_str)
                    .filter(|value| value.len() >= 32)
                    .map(str::to_owned)
                    .unwrap_or_else(|| uuid::Uuid::new_v4().simple().to_string());
                let expiry = value.get("expires_at").and_then(Value::as_str).map(str::to_owned);
                return (id, expiry);
            }
        }
    }
    let id = uuid::Uuid::new_v4().simple().to_string();
    let _ = save_activation_record(&id, None);
    (id, None)
}

fn save_activation_record(installation_id: &str, expires_at: Option<&str>) -> bool {
    let Some(path) = activation_record_path() else { return false };
    let Some(parent) = path.parent() else { return false };
    if fs::create_dir_all(parent).is_err() { return false; }
    let value = json!({ "installation_id": installation_id, "expires_at": expires_at });
    fs::write(path, value.to_string()).is_ok()
}
