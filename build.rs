fn main() {
	#[cfg(all(target_os = "windows", debug_assertions))] {
		let mut res = winres::WindowsResource::new();
		res.set_icon("imgs/hummingbird_new.ico");

		if let Err(e) = res.compile() {
			eprintln!("Could not attach Icon to executable ({})", e);
		}
	}
}