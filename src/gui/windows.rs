mod statuswindow;
mod renamewindow;
mod deletewindow;

use eframe::egui;
use serde_json::json;

use crate::TimeSpent;

pub struct Window {
	pub status_window: bool,
	pub status_data: serde_json::Value,

	pub raw_data_window: bool,
	pub raw_data: serde_json::Value,

	pub delete_window: bool,
	pub delete_data: serde_json::Value,

	pub rename_window: bool,
	pub rename_data: serde_json::Value,
	pub rename_to: String,
	pub rename_error: String,
}

impl Window {
	pub fn new() -> Self {
		return Window {
			status_window: false, status_data: json!({}),
			raw_data_window: false, raw_data: json!({}),
			delete_window: false, delete_data: json!({}),

			rename_window: false, rename_data: json!({}), 
			rename_to: String::new(), rename_error: String::new(),
		}
	}
}

impl TimeSpent {
	pub fn draw_raw_data_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("Raw Data").open(&mut self.win.raw_data_window)
		.vscroll(true).show(ctx, |ui| {
			ui.label(format!("{:#?}", self.win.raw_data));
		});
	}
}