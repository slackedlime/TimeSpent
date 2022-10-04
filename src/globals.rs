use chrono::{self, Datelike};
use std::path::PathBuf;

pub fn get_dirs() -> [PathBuf; 3] {
	let data_dir = dirs::data_dir().unwrap().join("Time-Spent");
	let processes_dir = data_dir.join("processes");
	let daemon_config = data_dir.join("daemon.json");
	let hidden_processes = data_dir.join("hidden.json");

	return [processes_dir, daemon_config, hidden_processes]
}

pub fn get_date() -> String {
	let t = chrono::offset::Local::now();  
	return format!("{}/{:0>2}/{:0>2}", t.year(), t.month(), t.day())
}