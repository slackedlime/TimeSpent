use std::path::PathBuf;
use chrono::{self, Datelike};

pub fn get_dirs() -> [PathBuf; 2] {
	let data_dir = dirs::data_dir().unwrap().join("Time-Spent");
	let processes_dir = data_dir.join("processes");
	let hidden_processes = data_dir.join("hidden.json");

	return [processes_dir, hidden_processes]
}

pub fn get_date() -> String {
	let t = chrono::offset::Local::now();
	return format!("{}/{}/{}", t.year(), t.month(), t.day());
}