use std::{fs, path::Path};
use serde_json::{json, Value as JsonValue};

pub fn set_json_data(name: String, exe_dir: &Path, json_dir: &Path, config: &JsonValue) 
	-> std::io::Result<()> {
		
	let data_file = json_dir.join(format!("{}.json", name));

	let friendly_name = match name.rsplit_once(".") {
		Some(n) => n.0,
		None => &name,
	};

	if !data_file.is_file() {
		let j = json!({
			"name": name,
			"friendlyName": friendly_name,
			"exeLocation": exe_dir.to_str().unwrap(),
			"totalTimeRun": 0,
			"perDayTimeRun": {}
		});

		fs::write(&data_file, j.to_string().as_bytes())?;
	}

	let contents = fs::read_to_string(&data_file)?;

	let tick_speed = config["tickSpeed"].as_u64().unwrap_or(1);

	// totalTimeRun
	let mut info: serde_json::Value = serde_json::from_str(&contents)?;
	info["totalTimeRun"] = json!(info["totalTimeRun"].as_u64().unwrap() + tick_speed);

	// perDayTimeRun
	let today = crate::globals::get_date();

	if info["perDayTimeRun"].get(&today).is_none() {
		info["perDayTimeRun"][&today] = json!(0);
	}

	info["perDayTimeRun"][&today] = 
		json!(info["perDayTimeRun"][&today].as_u64().unwrap() + tick_speed);
	//

	fs::write(&data_file, info.to_string().as_bytes())?;

	Ok(())
}