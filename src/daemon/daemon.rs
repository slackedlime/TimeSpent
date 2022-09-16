#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[path = "../globals.rs"]
mod globals;

use std::{thread, fs, process, path::Path, time::Duration};
use sysinfo::{self, SystemExt, PidExt, ProcessExt};
use serde_json::json;

#[cfg(target_os = "windows")]
fn get_pid() -> u32 {
	use winapi::um::winuser;

	let mut pid: u32 = 0;

	unsafe {
		let hwnd = winuser::GetForegroundWindow();
		winuser::GetWindowThreadProcessId(hwnd, &mut pid);
	}

	return pid
}

#[cfg(target_os = "linux")]
fn get_pid() -> u32 {
	let output_opt = process::Command::new("xdotool")
					 .args(["getwindowfocus", "getwindowpid"])
					 .output();
	
	if output_opt.is_err() {
		println!("Failed to execute process");
		println!("Is xdotool installed?");

		process::exit(1);
	}

	let mut output = output_opt.unwrap().stdout;
	output.pop(); // To remove \n from the end

	let pid_string = String::from_utf8_lossy(&output);
	let pid: u32 = pid_string.parse().unwrap_or(0);

	return pid
}

fn get_focused_application(system: &sysinfo::System) -> (String, &Path) {

	let pid = get_pid(); 

	if let Some(process) = system.process(sysinfo::Pid::from_u32(pid)) {
		return (process.name().to_string(), process.exe())
		
	}

	return ("Idle".to_string(), Path::new("/"))
}

fn set_json_data(name: String, exe_dir: &Path, json_dir: &Path) 
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

	// totalTimeRun
	let mut info: serde_json::Value = serde_json::from_str(&contents)?;
	info["totalTimeRun"] = json!(info["totalTimeRun"].as_u64().unwrap() + 1);

	// perDayTimeRun
	let today = globals::get_date();

	if info["perDayTimeRun"].get(&today).is_none() {
		info["perDayTimeRun"][&today] = json!(0);
	}

	info["perDayTimeRun"][&today] = 
		json!(info["perDayTimeRun"][&today].as_u64().unwrap() + 1);
	//

	fs::write(&data_file, info.to_string().as_bytes())?;

	Ok(())
}

fn main() {
	let [processes_dir, _icon_dir] = globals::get_dirs(); 
	
	if !processes_dir.is_dir() {
		if let Err(e) = fs::create_dir_all(&processes_dir) {
			println!("Could not create {:?}", processes_dir);
			println!("Reason: {}", e);

			process::exit(1);
		}

		println!("Created {:?}", processes_dir);
	}

	// Make sysinfo only refresh the process list
	let only_processes = sysinfo::ProcessRefreshKind::new();
	let r = sysinfo::RefreshKind::new().with_processes(only_processes);
	
	let mut system = sysinfo::System::new_with_specifics(r);
	
	loop {
		system.refresh_processes_specifics(only_processes);
		let (name, exe) = get_focused_application(&system);

		if let Err(e) = set_json_data(name, exe, &processes_dir) {
			println!("Error: {}", e)
		}

		thread::sleep( Duration::from_secs(1) )
	}
}