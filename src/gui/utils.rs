use std::{fs, path::Path};
use serde_json::Value as JsonValue;

pub fn get_info(json_dir: &Path) -> Vec<JsonValue> {
	let mut data: Vec<JsonValue> = match fs::read_dir(json_dir) {
		Ok(paths) => {
			let mut files = Vec::new();
			
			for path in paths {
				let proper_path = path.unwrap().path();

				let content = fs::read_to_string(&proper_path).unwrap();
					
				if let Ok(json) = serde_json::from_str(&content) {
					files.push(json);
				} else {
					println!("Error: Could not read {:?}", proper_path);
				}
			}

			files
		},

		Err(_) => Vec::new(),
	};

	data.sort_by_key(|json| json["totalTimeRun"].as_u64().unwrap());
	data.reverse();

	return data;
}

pub fn get_hidden_processes(hidden_processes_file: &Path) -> Vec<JsonValue> {
	if !hidden_processes_file.exists() {
		match fs::write(&hidden_processes_file, "[]".as_bytes()) {
			Ok(_) => println!("hidden.json Created"),
			Err(_) => println!("hidden.json Couldn't be Created"),
		}
	}

	let raw_content = fs::read_to_string(&hidden_processes_file)
					  .unwrap_or("[]".to_string());

	let content = match serde_json::from_str(&raw_content) {
		Ok(cont) => cont,
		Err(e) => {
			println!("{:?}", e);
			serde_json::json!([])
		},
	};

	return content.as_array().unwrap().to_vec()
}

pub fn format_time(time_in_secs: f64) -> String {
	let days = (time_in_secs / 86400.).floor();
	let hours = (time_in_secs / 3600.).floor() - days * 24.;
	let mins = ((time_in_secs / 60.) % 60.).floor();
	let secs = time_in_secs % 60.;

	let time = [days, hours, mins, secs];
	let time_symbols = ["d", "h", "m", "s"];

	let mut formatted_time = String::new();
	for (t, l) in time.iter().zip(time_symbols) {
		if t != &0. {
			formatted_time.push_str(&format!("{}{} ", t, l));
		}
	}

	return formatted_time
}