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
	pub fn draw_status_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("Status").open(&mut self.win.status_window)
		.vscroll(true) .show(ctx, |ui| {
			let status_data = self.win.status_data["perDayTimeRun"]
							  .as_object().unwrap();

			if status_data.is_empty() {
				return
			}

			ui.horizontal(|ui| {
				ui.set_min_width(180.);

				ui.label("Executable path: ");

				ui.add(
					egui::Label::new(
						egui::RichText::new(

							if self.win.status_data["exeLocation"].is_null() {
								"null"
							} else {
								self.win.status_data["exeLocation"]
								.as_str().unwrap()
							}

						).monospace()
					).wrap(true)
				);

			});

			ui.add_space(5.);

			for i in status_data {
				ui.label(format!("{:?}", i));
			}
		});
	}

	pub fn draw_rename_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("Rename").show(ctx, |ui| {
			let mut rename_data = self.win.rename_data.clone();

			ui.heading(
				format!("What should {} be renamed to?", 
						rename_data["friendlyName"])
			);

			ui.add_space(3.);		

			ui.add(
				egui::widgets::TextEdit::singleline(&mut self.win.rename_to)
				.hint_text("New Name")
				.desired_width(120.)
			);

			if self.win.rename_to.is_empty() {
				self.win.rename_error = "Please Enter a New Name".to_string();
			
			} else if self.win.rename_to.len() > 20 {
				self.win.rename_error = 
					"Please Enter a Name Shorter than 20 Letters".to_string();
			
			} else {
				self.win.rename_error = String::new();
			}

			if !self.win.rename_error.is_empty() {
				ui.label(&self.win.rename_error);
			}
			
			ui.add_space(5.);

			ui.horizontal(|ui| {
				if ui.button("Rename").clicked() && self.win.rename_error.is_empty() {
					let filename = format!("{}.json", 
						rename_data["name"].as_str().unwrap());
	
					let fullpath = &self.processes_dir.join(filename);
	
					rename_data["friendlyName"] = json!(self.win.rename_to);

					if let Err(e) = std::fs::write(&fullpath, 
									rename_data.to_string().as_bytes()) {

						print!("{:?} Could not be written\nError: {}", fullpath, e)
					}

					
					self.refresh();
					self.win.rename_to = String::new();

					self.win.rename_window = false;
				}

				if ui.button("Cancel").clicked() {
					self.win.rename_error = String::new();
					self.win.rename_to = String::new();
					self.win.rename_window = false;
				}
			});
		});
	}

	pub fn draw_delete_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("Delete").show(ctx, |ui| {
			let data = self.win.delete_data.clone();

			ui.heading(
				format!("Are you sure that you want to delete {}?", data["friendlyName"])
			);
			
			ui.add_space(1.);

			ui.colored_label(
				egui::Color32::LIGHT_RED, 
				"This action can not be undone",
			);
			
			ui.add_space(5.);

			ui.horizontal(|ui| {
				if ui.button(egui::RichText::new("Delete").size(16.)).clicked() {
					let filename = format!("{}.json", 
						data["name"].as_str().unwrap());
					
					let fullpath = &self.processes_dir.join(filename);

					if let Err(e) = std::fs::remove_file(fullpath) {
						println!("Couldn't delete {}", data["name"]);
						println!("Error: {}", e);
					}

					self.refresh();
					self.win.delete_window = false;
				}

				ui.add_space(5.);

				if ui.button(egui::RichText::new("Cancel").size(16.)).clicked() {
					self.win.delete_window = false;
				}
			});

		});
	}

	pub fn draw_raw_data_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("Raw Data").open(&mut self.win.raw_data_window)
		.vscroll(true).show(ctx, |ui| {
			ui.label(format!("{:#?}", self.win.raw_data));
		});
	}
}