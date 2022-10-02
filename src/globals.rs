use chrono::{self, Datelike};
use std::{fs, path::{Path, PathBuf}};
use serde_json::Value as JsonValue;

pub fn get_dirs() -> [PathBuf; 3] {
	let data_dir = dirs::data_dir().unwrap().join("Time-Spent");
	let processes_dir = data_dir.join("processes");
	let config_file = data_dir.join("config.json");
	let hidden_processes = data_dir.join("hidden.json");

	return [processes_dir, config_file, hidden_processes]
}

pub fn get_date() -> String {
	let t = chrono::offset::Local::now();  
	return format!("{}/{:0>2}/{:0>2}", t.year(), t.month(), t.day())
}

// ==CONFIG DOCUMENTATION==
// "tickSpeed" Controls how often the application checks for the focused app
// it is recommended to not put it above 10
//
// "autoDeleteCorrupted" deletes corrupted json files
// (Disable it if you plan on manually editing the Json)
//
// "safeWrite" decreases the probability of json being corrupted
// disabling it might increase performance (NOT RECOMMENDED)

pub fn get_default_config() -> JsonValue {
	return serde_json::json!({
		"tickSpeed": 1,
		"autoDeleteCorrupted": true,
		"safeWrite": true,
	});
}

pub fn get_config(file_path: &Path) -> std::io::Result<JsonValue> {
	if !file_path.is_file() {
		let default_json = get_default_config();

		let pretty_json = serde_json::to_string_pretty(&default_json).unwrap();

		fs::write(&file_path, pretty_json.as_bytes())?;
	}

	let contents = fs::read_to_string(&file_path)?;

	return Ok(serde_json::from_str(&contents)?);
}