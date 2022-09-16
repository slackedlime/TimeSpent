use crate::utils::format_time;
use crate::globals;

use serde_json::json;

use eframe::egui;
use egui_extras::{TableBuilder, Size};

use crate::TimeSpent;

impl TimeSpent {
	pub fn draw_table(&mut self, ui: &mut egui::Ui) {
		TableBuilder::new(ui)
		.cell_layout(egui::Layout::left_to_right(egui::Align::Center))
		.column(Size::initial(200.))
		.column(Size::initial(140.))
		.column(Size::remainder())
		.resizable(true)
		.striped(true)

		.header(20., |mut header| {
			for title in ["Name", "Today", "Total"] {
				header.col(|ui| {
					ui.heading( title );
				});
			}
		})

		.body(|mut body| { 
			for d in self.data.clone() {

				let is_hidden = self.hidden_processes.contains(&json!(d["name"]));

				if self.hide && is_hidden {
					continue
				}
				
				body.row(20., |mut row| {
					row.col(|ui| {
						let name = d["friendlyName"].as_str().unwrap().to_string();

						if is_hidden {
							ui.colored_label(egui::Color32::DARK_GRAY, "⊗");

						} else if !self.hide {
							ui.colored_label(egui::Color32::DARK_GRAY, "○");
						}
						
						let response = ui.add(egui::Button::new(&name)
										 .fill(egui::Color32::TRANSPARENT));

						if response.clicked() {
							crate::open_window!(
								self.win.status_window, self.win.status_data, &d);
						}

						response
						.context_menu(|ui| {
							self.draw_context_menu(name, &d, ui);
						});
					});
					row.col(|ui| {
						let today = globals::get_date();

						if let Some(time) = d["perDayTimeRun"][today].as_f64() {
							ui.strong(format_time(time));
						} else {
							ui.strong("0s");
						}
						
					});
					row.col(|ui| {
						ui.strong(format_time(d["totalTimeRun"].as_f64().unwrap()));
					});
				});
			}
		});
	}
}