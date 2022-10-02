#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[path = "../globals.rs"]
mod globals;
#[path = "../getconfig.rs"]
mod getconfig;

mod utils;
mod table;
mod windows;
mod contextmenu;

use std::path::PathBuf;

use eframe::egui;

struct TimeSpent {
	data: Vec<serde_json::Value>,
	win: windows::Window,
	processes_dir: PathBuf,

	config: serde_json::Value,

	hidden_processes_file: PathBuf,
	hidden_processes: Vec<serde_json::Value>,
	hide: bool,
}

impl TimeSpent {
	fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		let [processes_dir, config_file, hidden_processes_file] = globals::get_dirs();

		let hidden_processes = 
			utils::get_hidden_processes(&hidden_processes_file);

		let config = match getconfig::get_config(&config_file) {
			Ok(json) => json,
	
			Err(e) => {
				println!("Error: {}", e);
				getconfig::get_default_config()
			}
		};

		let data = utils::get_info(&processes_dir, &config);

		let win = windows::Window::new();

		return TimeSpent { 
			data, win, processes_dir, config, 
			hidden_processes_file, hidden_processes, hide: true
		}
	}

	fn refresh(&mut self) {
		self.hidden_processes = 
			utils::get_hidden_processes(&self.hidden_processes_file);

		self.data = utils::get_info(&self.processes_dir, &self.config);
	}

	fn draw_footerbar(&mut self, ctx: &egui::Context) {
		egui::TopBottomPanel::bottom("footer").default_height(30.)
		.show(ctx, |ui| {
			ui.horizontal_centered(|ui| {
				if ui.button("Refresh").clicked() {
					self.refresh()
				}

				let hide_button_text = format!( "{} Hidden Items", 
									   if self.hide {"Show"} else {"Hide"} );

				if ui.button(hide_button_text).clicked() {
					self.hide = !self.hide
				}

				ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
					egui::widgets::global_dark_light_mode_buttons(ui);
				});
			});
		});
	}
}

impl eframe::App for TimeSpent {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {

			egui::Frame::none()
				.outer_margin(egui::style::Margin {
					left: 0., right: 0.,
					top: 0., bottom: 25.,
				})
				.show(ui, |ui| {
					self.draw_table(ui);

					ui.separator();
				});

			self.draw_raw_data_window(ctx);
			self.draw_status_window(ctx);

			if self.win.delete_window {
				self.draw_delete_window(ctx);
			}

			if self.win.rename_window {
				self.draw_rename_window(ctx);
			}
		});

		self.draw_footerbar(ctx);
	}
}

fn main() {
	let icon_data = include_bytes!("../../imgs/hummingbird_new.ico");

	let icon = image::load_from_memory_with_format(
		icon_data, image::ImageFormat::Ico
	).expect("Could not load icon").blur(3.5).to_rgba8();


	let mut win_opts = eframe::NativeOptions::default();
	win_opts.initial_window_size = Some(egui::Vec2::new(550., 560.));
	win_opts.resizable = true;
	win_opts.default_theme = win_opts.system_theme()
							 .unwrap_or(eframe::Theme::Dark);
	win_opts.icon_data = Some(eframe::IconData {
		width: icon.width(),
		height: icon.height(),
		
		rgba: icon.into_raw(),
	});

	eframe::run_native("Time Spent", win_opts,
		Box::new(|cc| Box::new(TimeSpent::new(cc))));
}