#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[path = "../globals.rs"]
mod globals;
mod write;

use std::{thread, fs, process, path::Path, time::Duration};
use sysinfo::{self, SystemExt, PidExt, ProcessExt};

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
		log!("Failed to execute process");
		log!("Is xdotool installed?");

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

fn main() {
	let file_dirs = globals::Dirs::new();
	let processes_dir = file_dirs.processes_dir;
	let config_file = file_dirs.daemon_config;

	if !processes_dir.is_dir() {
		if let Err(e) = fs::create_dir_all(&processes_dir) {
			log!("Could not create {:?} ({})", processes_dir, e);

			process::exit(1);
		}

		log!("Created {:?}", processes_dir);
	}
	
	let config = match write::get_config(&config_file) {
		Ok(json) => json,

		Err(e) => {
			log!("Couldn't write Config ({})", e);
			write::get_default_config()
		}
	};

	// Make sysinfo only refresh the process list
	let only_processes = sysinfo::ProcessRefreshKind::new();
	let r = sysinfo::RefreshKind::new().with_processes(only_processes);
	
	let mut system = sysinfo::System::new_with_specifics(r);

	// If TimeSpentDaemon is already running, then stop execution
	if system.processes_by_name("TimeSpentDaemon").count() > 1 {
		log!("The Daemon is already running");
		process::exit(1);
	}

	loop {
		system.refresh_processes_specifics(only_processes);
		let (name, exe) = get_focused_application(&system);

		if let Err(e) = write::set_json_data(name, exe, &processes_dir, &config) {
			log!("Couldn't write json ({})", e);
		}

		thread::sleep( 
			Duration::from_secs( config["tickSpeed"].as_u64().unwrap_or(1) ) 
		);
	}
}